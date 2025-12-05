use std::path::Path;

use advent_of_code::{file_to_string_array, write_aoc_config, AocConfig, PuzzleConfig};

use crate::{
    context::{Context, DayGenData},
    generror::{GenError, GenResult},
    str_to_file, string_array_to_file,
};

pub fn generate_day(context: &Context) -> GenResult<()> {
    let Some(day_data) = context.day_module_data() else {
        return Ok(());
    };

    if day_data.module_file_path.exists() {
        return Err(GenError::new(
            "Cancelled creating day file because it's already exists",
        ));
    }

    create_file(&day_data)?;
    patch_year_module(&context.year_mod_file_path(), &day_data)?;
    update_marker_file(context.year(), day_data.day)?;
    Ok(())
}

fn create_file(data: &DayGenData) -> std::io::Result<()> {
    let mut output = TEMPLATE_DAY_MODULE.to_string();

    output = output.replace(PLACEHOLDER_STRUCT_NAME, &data.struct_name);
    output = output.replace(PLACEHOLDER_INPUT_FILENAME, &data.day_input_file_name);
    output = output.replace(
        PLACEHOLDER_TEST_FUNC_PREFIX,
        &data.day_test_func_name_prefix,
    );

    str_to_file(&data.module_file_path, &output)
}

fn patch_year_module(year_mod_file_path: &Path, data: &DayGenData) -> GenResult<()> {
    let mut lines = file_to_string_array(year_mod_file_path)?;
    let include_str = {
        let value = TEMPLATE_INCLUDE_DAY.replace(PLACEHOLDER_MODULE_NAME, &data.module_name);
        (MARKER_DAY_MOD_INCLUDE, value)
    };
    let register_str = {
        let value = TEMPLATE_REGISTER_DAY.replace(PLACEHOLDER_STRUCT_NAME, &data.struct_name);
        (MARKER_FACTORY_DAY, value)
    };

    for (marker, insertion) in [include_str, register_str] {
        let Some(index) = lines.iter().position(|x| x.contains(marker)) else {
            let message = format!("Marker '{marker}' not found year module file");
            return Err(GenError::new(message));
        };
        lines.insert(index, insertion);
    }

    string_array_to_file(year_mod_file_path, &lines)?;
    Ok(())
}

fn update_marker_file(year: usize, day: usize) -> GenResult<()> {
    let puzzle = PuzzleConfig::with(year, day);
    let config = AocConfig { puzzle };
    write_aoc_config(config)?;
    Ok(())
}

const MARKER_DAY_MOD_INCLUDE: &str = "// GENERATOR_MARKER: DAY_MOD_USE";
const MARKER_FACTORY_DAY: &str = "// GENERATOR_MARKER: FACTORY_DAY";

const PLACEHOLDER_STRUCT_NAME: &str = "${STRUCT_NAME}";
const PLACEHOLDER_INPUT_FILENAME: &str = "${INPUT_FILENAME}";
const PLACEHOLDER_TEST_FUNC_PREFIX: &str = "${TEST_FUNC_PREFIX}";
const PLACEHOLDER_MODULE_NAME: &str = "${MODULE_NAME}";

const TEMPLATE_INCLUDE_DAY: &str = r#"mod ${MODULE_NAME};
use ${MODULE_NAME}::*;
"#;

const TEMPLATE_REGISTER_DAY: &str = r#"        &|| Ok(Box::new(${STRUCT_NAME}::new()?)),"#;

const TEMPLATE_DAY_MODULE: &str = r#"use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct ${STRUCT_NAME} {
    // input: ...
}

impl ${STRUCT_NAME} {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("${INPUT_FILENAME}")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        todo!()
    }

    pub fn _new() -> io::Result<Self> {
        let data = std::fs::read_to_string("${INPUT_FILENAME}")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
         todo!()
    }
}

impl Solution for ${STRUCT_NAME} {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ${TEST_FUNC_PREFIX}_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        // assert!(!sol.input.is_empty());
        Ok(())
    }


    #[test]
    fn ${TEST_FUNC_PREFIX}_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "");
        Ok(())
    }

    #[test]
    fn ${TEST_FUNC_PREFIX}_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<${STRUCT_NAME}> {
        ${STRUCT_NAME}::new()
    }
}
"#;
