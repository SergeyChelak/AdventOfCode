mod genmod;
use std::fs::read_to_string;

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
    eprintln!("{}", error.message())
}

fn file_to_string_array(path: &std::path::Path) -> std::io::Result<Vec<String>> {
    let arr = read_to_string(path)?
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    Ok(arr)
}

fn string_array_to_file(path: &std::path::Path, lines: &[String]) -> std::io::Result<()> {
    let output = lines.join("\n");
    str_to_file(path, &output)
}

fn str_to_file(path: &std::path::Path, output: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;
    std::io::Write::write_all(&mut file, output.as_bytes())?;
    std::io::Write::flush(&mut file)
}
