use crate::solution::Solution;
use crate::file_utils::*;

use std::io;

pub struct AoC2015_13 {
    // place required fields here
}

impl AoC2015_13 {
    pub fn new() -> io::Result<Self> {
        let _ = read_file_as_lines("input/aoc2015_13")?;
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2015_13 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2015/Day 13: Knights of the Dinner Table".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_13_input_load_test() -> io::Result<()> {
        Ok(())
    }

    #[test]
    fn aoc2015_13_correctness() -> io::Result<()> {
        let sol = AoC2015_13::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}