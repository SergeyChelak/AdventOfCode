mod genmod;

use advent_of_code::str_to_file;
use genmod::generate_module;

mod genday;
use genday::generate_day;

mod generror;
use generror::{GenError, GenResult};

mod context;
use context::Context;

fn main() -> GenResult<()> {
    let result = Context::create();
    let Ok(context) = result else {
        show_usage(result.err().unwrap());
        return Ok(());
    };

    if !context.is_marker_file_exists() {
        eprintln!("Generator works in the root of AoC folder only");
        return Ok(());
    }

    generate_module(&context)?;

    generate_day(&context)
}

fn show_usage(error: GenError) {
    eprintln!("{}", error.message());
    println!("Boilerplate code generator for Advent of Code puzzles");
    println!("Usage:\n");
    println!("Generate new year module");
    println!("\tcargo r --bin generator -- -y2077");
    println!("Generate new year module (if not exists) and day module");
    println!("\tcargo r --bin generator -- -y2077 -d1");
    println!("\nNote: generator will not overwrite existing files");
}

fn string_array_to_file(path: &std::path::Path, lines: &[String]) -> std::io::Result<()> {
    let output = lines.join("\n");
    str_to_file(path, &output)
}
