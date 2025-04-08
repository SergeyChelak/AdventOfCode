use std::io;
use std::ops::Deref;
use std::time::Instant;
mod execute_mode;
mod solution;
mod utils;

mod aoc2015;
mod aoc2016;
mod aoc2017;
mod aoc2018;
mod aoc2019;
mod aoc2023;
mod aoc2024;
use crate::aoc2015::puzzle_factory_2015;
use crate::aoc2016::puzzle_factory_2016;
use crate::aoc2017::puzzle_factory_2017;
use crate::aoc2018::puzzle_factory_2018;
use crate::aoc2019::puzzle_factory_2019;
use crate::aoc2023::puzzle_factory_2023;
use crate::aoc2024::puzzle_factory_2024;
use crate::solution::AggregatedFactory;
use execute_mode::{get_execute_mode, ExecuteMode};
use solution::Solution;

fn main() -> io::Result<()> {
    println!("Advent of Code");
    let Ok(mode) = get_execute_mode() else {
        println!("Failed to process parameters");
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
        ExecuteMode::Undefined => {
            println!("Input is missing in command line parameters or toml-file");
        }
    }
    Ok(())
}

fn execute_year_puzzles(factory: &AggregatedFactory, year: usize) {
    let mut found = false;
    for day in 1..=25 {
        let Some(puzzle) = factory.puzzle(year, day) else {
            continue;
        };
        let Ok(puzzle) = puzzle else {
            println!("Execution terminated because of failure of creation {year}/{day} puzzle");
            return;
        };
        found = true;
        execute(puzzle.deref());
    }
    if !found {
        println!("Nothing to execute");
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
    factory.add_factory(puzzle_factory_2019());
    factory.add_factory(puzzle_factory_2023());
    factory.add_factory(puzzle_factory_2024());
    factory
}

fn execute(solution: &dyn Solution) {
    println!();
    let mut description = solution.description();
    if description.is_empty() {
        description = "## UNTITLED PUZZLE ##".to_string();
    }
    println!("{}", description);
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
