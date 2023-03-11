use std::io;
mod utils;
mod solution;

mod aoc2015;
use aoc2015::*;
use solution::Solution;

fn main() -> io::Result<()> {
    println!("Advent of Code");
    let days: Vec<Box<dyn Solution>> = vec![
        Box::new(AoC2015_01::new()?),
        Box::new(AoC2015_02::new()?),
        Box::new(AoC2015_03::new()?),
        Box::new(AoC2015_04::new()?),
        Box::new(AoC2015_05::new()?),
        Box::new(AoC2015_06::new()?),
        Box::new(AoC2015_07::new()?),
        Box::new(AoC2015_08::new()?),
        Box::new(AoC2015_09::new()?),
        Box::new(AoC2015_10::new()?),
        Box::new(AoC2015_11::new()?),
        Box::new(AoC2015_12::new()?),
        Box::new(AoC2015_13::new()?),
        Box::new(AoC2015_14::new()?),
        Box::new(AoC2015_15::new()?),
        Box::new(AoC2015_16::new()?),
        Box::new(AoC2015_17::new()?),
        Box::new(AoC2015_18::new()?),
        Box::new(AoC2015_19::new()?),
        Box::new(AoC2015_20::new()?)
    ];
    // days.iter()
    //     .for_each(execute);
    if let Some(day) = days.last() {
        execute(day);
    }
    Ok(())
}

fn execute(solution: &Box<dyn Solution>) {
    println!();
    println!("{}", solution.description());
    println!("\tPart 1: {}", solution.part_one());
    println!("\tPart 2: {}", solution.part_two());
}