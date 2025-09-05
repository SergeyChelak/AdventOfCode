use std::{io::Write, path::PathBuf};

use crate::{
    context::Context,
    generror::{GenError, GenResult},
};

pub fn generate_day(context: &Context) -> GenResult<()> {
    let (Some(day_file), Some(struct_name), Some(input_filename), Some(test_fn_prefix)) = (
        context.day_file_path(),
        context.day_struct_name(),
        context.day_input_file_name(),
        context.day_test_func_name_prefix(),
    ) else {
        return Ok(());
    };

    if day_file.exists() {
        return Err(GenError::new(
            "Cancelled creating day file because it's already exists",
        ));
    }

    create_file(day_file, &struct_name, &input_filename, &test_fn_prefix)?;
    patch_year_module();
    update_marker_file();
    Ok(())
}

fn create_file(
    day_file_path: PathBuf,
    struct_name: &str,
    input_filename: &str,
    test_fn_prefix: &str,
) -> GenResult<()> {
    let mut output = TEMPLATE_DAY_MODULE.to_string();

    output = output.replace(PLACEHOLDER_STRUCT_NAME, struct_name);
    output = output.replace(PLACEHOLDER_INPUT_FILENAME, input_filename);
    output = output.replace(PLACEHOLDER_TEST_FUNC_PREFIX, test_fn_prefix);

    let mut file = std::fs::File::create(day_file_path)?;
    file.write_all(output.as_bytes())?;
    file.flush()?;
    Ok(())
}

fn patch_year_module() {
    todo!()
}

fn update_marker_file() {
    todo!()
}

const PLACEHOLDER_STRUCT_NAME: &str = "${STRUCT_NAME}";
const PLACEHOLDER_INPUT_FILENAME: &str = "${INPUT_FILENAME}";
const PLACEHOLDER_TEST_FUNC_PREFIX: &str = "${TEST_FUNC_PREFIX}";

const TEMPLATE_DAY_MODULE: &str = r#"use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct ${STRUCT_NAME} {
    //
}

impl ${STRUCT_NAME} {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("${INPUT_FILENAME}")?;
        let input = std::fs::read_to_string("${INPUT_FILENAME}")?;
        Ok(Self {
            // do init
        })
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
