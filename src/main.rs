use std::io;
mod solution;

mod aoc2015;
use aoc2015::*;
use solution::Solution;

fn main() -> io::Result<()> {
    println!("Advent of Code");
    let solution = AoC2015_02::new();
    execute(&solution);
    Ok(())
}

fn execute(solution: &impl Solution) {
    println!();
    println!("{}", solution.description());
    println!("\tPart 1: {}", solution.part_one());
    println!("\tPart 2: {}", solution.part_two());
}
