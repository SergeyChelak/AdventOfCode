mod genmod;
use genmod::generate_module;

mod genday;
// use genday::

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

    Ok(())
}

fn show_usage(error: GenError) {
    eprintln!("{}", error.message())
}
