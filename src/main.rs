use std::io;
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
    ];
    days.iter()
        .for_each(|s| execute(s));
    Ok(())
}

fn execute(solution: &Box<dyn Solution>) {
    println!();
    println!("{}", solution.description());
    println!("\tPart 1: {}", solution.part_one());
    println!("\tPart 2: {}", solution.part_two());
}
