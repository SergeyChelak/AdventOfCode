use std::io;

mod aoc2015;
use aoc2015::*;

fn main() -> io::Result<()> {
    println!("Advent of Code");
    let aoc2015_01 = AoC2015_01::new()?;
    aoc2015_01.part1();
    aoc2015_01.part2();
    Ok(())
}
