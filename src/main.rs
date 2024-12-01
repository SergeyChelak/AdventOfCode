use std::io;
use std::num::ParseIntError;
use std::ops::Deref;
use std::time::Instant;
mod solution;
mod utils;

mod aoc2015;
mod aoc2016;
mod aoc2017;
mod aoc2018;
mod aoc2023;
mod aoc2024;
use crate::aoc2015::puzzle_factory_2015;
use crate::aoc2016::puzzle_factory_2016;
use crate::aoc2017::puzzle_factory_2017;
use crate::aoc2018::puzzle_factory_2018;
use crate::aoc2023::puzzle_factory_2023;
use crate::aoc2024::puzzle_factory_2024;
use crate::solution::AggregatedFactory;
use solution::Solution;

#[derive(Debug)]
enum ExecuteMode {
    Single { year: usize, day: usize },
    Year { year: usize },
    Last,
}

impl ExecuteMode {
    fn year(year: &str) -> Result<Self, ParseIntError> {
        let year = year.parse::<usize>()?;
        Ok(Self::Year { year })
    }

    fn single(year: &str, day: &str) -> Result<Self, ParseIntError> {
        let year = year.parse::<usize>()?;
        let day = day.parse::<usize>()?;
        Ok(Self::Single { year, day })
    }
}

fn main() -> io::Result<()> {
    println!("Advent of Code");
    let Ok(mode) = get_execute_mode() else {
        println!("Incorrect parameters");
        return Ok(());
    };
    let factory = create_factory();
    match mode {
        ExecuteMode::Single { year, day } => {
            execute_puzzle(&factory, year, day);
        }
        ExecuteMode::Year { year } => {
            execute_year_puzzles(&factory, year);
        }
        ExecuteMode::Last => {
            execute_puzzle(&factory, 2024, 1);
        }
    }
    Ok(())
}

fn execute_year_puzzles(factory: &AggregatedFactory, year: usize) {
    for day in 1..=25 {
        let Some(puzzle) = factory.puzzle(year, day) else {
            continue;
        };
        let Ok(puzzle) = puzzle else {
            return;
        };
        execute(puzzle.deref());
    }
}

fn execute_puzzle(factory: &AggregatedFactory, year: usize, day: usize) {
    let puzzle = factory.puzzle(year, day);
    let Some(puzzle) = puzzle else {
        println!("Puzzle {year}\\{day} not found");
        return;
    };
    let Ok(puzzle) = puzzle else {
        println!("Failed to create solution for {year}\\{day} puzzle");
        return;
    };
    execute(puzzle.deref());
}

fn create_factory() -> AggregatedFactory {
    let mut factory = AggregatedFactory::new();
    factory.add_factory(puzzle_factory_2015());
    factory.add_factory(puzzle_factory_2016());
    factory.add_factory(puzzle_factory_2017());
    factory.add_factory(puzzle_factory_2018());
    factory.add_factory(puzzle_factory_2023());
    factory.add_factory(puzzle_factory_2024());
    factory
}

fn get_execute_mode() -> Result<ExecuteMode, ParseIntError> {
    let args: Vec<String> = std::env::args().collect();
    let values = (args.get(1), args.get(2));
    let params = match values {
        (Some(year), None) => ExecuteMode::year(year)?,
        (Some(year), Some(day)) => ExecuteMode::single(year, day)?,
        _ => ExecuteMode::Last,
    };
    Ok(params)
}

fn execute(solution: &dyn Solution) {
    println!();
    println!("{}", solution.description());
    let measure = |part: u8, proc: &dyn Fn() -> String| {
        let now = Instant::now();
        let result = proc();
        let duration = now.elapsed().as_millis();
        let title = format!("{} ms for part {}", duration, part);
        println!("{:>30}: {}", title, result);
    };
    measure(1, &|| solution.part_one());
    measure(2, &|| solution.part_two());
}
