use std::{
    io::{self, Write},
    path::PathBuf,
};

use crate::{
    context::Context,
    file_to_string_array,
    generror::{GenError, GenResult},
    string_array_to_file,
};

pub fn generate_module(context: &Context) -> GenResult<()> {
    {
        let year_folder = context.year_folder();
        if !year_folder.exists() {
            std::fs::create_dir(year_folder)?;
        }
    }
    let year_str = context.year().to_string();

    {
        let mod_file = context.year_mod_file_path();
        if mod_file.exists() {
            println!("Skip module generation due mod.rs already exists");
            return Ok(());
        }
        generate_mod_file(mod_file, &year_str)?;
    }

    {
        let main_file = context.main_file_path();
        if !main_file.exists() {
            return Err(GenError::new("Cancelling execution: main.rs not found"));
        }
        patch_main_file(main_file, &year_str)?;
    }
    Ok(())
}

fn generate_mod_file(path: PathBuf, year: &str) -> io::Result<()> {
    let mut file = std::fs::File::create(&path)?;
    let output = TEMPLATE_YEAR_MODULE.replace(YEAR_PLACEHOLDER, year);
    file.write_all(output.as_bytes())?;
    file.flush()
}

fn patch_main_file(path: PathBuf, year: &str) -> GenResult<()> {
    let mut lines = file_to_string_array(&path)?;

    let replace = [
        (
            MARKER_YEAR_MOD_INCLUDE,
            TEMPLATE_MODULE_INCLUDE,
            "Cancelled: mod marker not found in main.rs",
        ),
        (
            MARKER_FACTORY_REGISTER,
            TEMPLATE_FACTORY_REGISTER,
            "Cancelled: factory register marker not found in main.rs",
        ),
    ];

    for (marker, template, error) in replace {
        let Some(index) = lines.iter().position(|x| x.contains(marker)) else {
            return Err(GenError::new(error));
        };
        let s = template.replace(YEAR_PLACEHOLDER, year);
        lines.insert(index, s);
    }

    string_array_to_file(&path, &lines)?;
    Ok(())
}

const MARKER_YEAR_MOD_INCLUDE: &str = "// GENERATOR_MARKER: MOD_USE";
const MARKER_FACTORY_REGISTER: &str = "// GENERATOR_MARKER: ADD_FACTORY";

const YEAR_PLACEHOLDER: &str = "{$YEAR}";

const TEMPLATE_FACTORY_REGISTER: &str = "    factory.add_factory(puzzle_factory_{$YEAR}());";

const TEMPLATE_MODULE_INCLUDE: &str = r#"mod aoc{$YEAR};
use crate::aoc{$YEAR}::puzzle_factory_{$YEAR};
"#;

const TEMPLATE_YEAR_MODULE: &str = r#"
use super::solution::{PuzzleFactory, PuzzleFactoryMethod};

// GENERATOR_MARKER: DAY_MOD_USE

pub fn puzzle_factory_{$YEAR}() -> PuzzleFactory {
    let producers: Vec<&'static PuzzleFactoryMethod> = vec![
        // GENERATOR_MARKER: FACTORY_DAY
    ];
    PuzzleFactory::new({$YEAR}, producers)
}
"#;
