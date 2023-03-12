use std::io;
mod utils;
mod solution;

mod aoc2015;
use solution::Solution;

fn main() -> io::Result<()> {
    println!("Advent of Code");
    let args: Vec<String> = std::env::args().collect();
    let is_run_all = if let Some(val) = args.get(1) {
        val == "all"
    } else {
        false
    };
    if is_run_all {
        run_collection(aoc2015::all_days());
    } else {
        if let Ok(day) = &aoc2015::last_day() {
            execute(day);
        }
    }
    Ok(())
}

fn run_collection(days: io::Result<Vec<Box<dyn Solution>>>) {
    days.expect("Data isn't valid")
    .iter()
    .for_each(execute);
}

fn execute(solution: &Box<dyn Solution>) {
    println!();
    println!("{}", solution.description());
    println!("\tPart 1: {}", solution.part_one());
    println!("\tPart 2: {}", solution.part_two());
}