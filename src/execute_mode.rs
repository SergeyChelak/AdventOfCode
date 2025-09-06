use std::num::ParseIntError;

use advent_of_code::load_aoc_config;

#[derive(Debug)]
pub enum ExecuteMode {
    Single { year: usize, day: usize },
    Year { year: usize },
    Undefined,
}

impl ExecuteMode {
    fn by_parsing(year: Option<&String>, day: Option<&String>) -> Result<Self, ParseIntError> {
        let mut y: Option<usize> = None;
        if let Some(year) = year {
            y = Some(year.parse::<usize>()?);
        }
        let mut d: Option<usize> = None;
        if let Some(day) = day {
            d = Some(day.parse::<usize>()?);
        }
        Ok(Self::with(y, d))
    }

    fn with(year: Option<usize>, day: Option<usize>) -> Self {
        match (year, day) {
            (Some(year), None) => Self::Year { year },
            (Some(year), Some(day)) => Self::Single { year, day },
            _ => Self::Undefined,
        }
    }
}

pub fn get_execute_mode() -> std::io::Result<ExecuteMode> {
    let mut mode = get_execute_mode_from_arguments()?;
    if matches!(mode, ExecuteMode::Undefined) {
        mode = get_execute_mode_from_config()?;
    }
    Ok(mode)
}

fn get_execute_mode_from_arguments() -> std::io::Result<ExecuteMode> {
    let args: Vec<String> = std::env::args().collect();
    let mode = ExecuteMode::by_parsing(args.get(1), args.get(2))
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidInput, err))?;
    Ok(mode)
}

fn get_execute_mode_from_config() -> std::io::Result<ExecuteMode> {
    let puzzle_config = load_aoc_config()?.puzzle;
    let mode = ExecuteMode::with(puzzle_config.year, puzzle_config.day);
    Ok(mode)
}
