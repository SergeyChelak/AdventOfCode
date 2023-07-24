use crate::solution::Solution;

use std::fs::read_to_string;
use std::io;

type Int = i32;

pub struct AoC2018_08 {
    input: Vec<Int>,
}

impl AoC2018_08 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2018_08")?
            .split_whitespace()
            .map(|x| x.parse::<Int>().expect("Non int value in the input"))
            .collect();

        Ok(Self { input })
    }
}

impl Solution for AoC2018_08 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 8: Memory Maneuver".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_08_input_load_test() -> io::Result<()> {
        let sol = AoC2018_08::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_08_correctness() -> io::Result<()> {
        let sol = AoC2018_08::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
