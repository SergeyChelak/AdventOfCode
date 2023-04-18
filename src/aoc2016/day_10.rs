use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2016_10 {
    // place required fields here
}

impl AoC2016_10 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2016_10")?;
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2016_10 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2016/Day 10: Balance Bots".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_10_input_load_test() -> io::Result<()> {
        let sol = AoC2016_10::new()?;
        Ok(())
    }

    #[test]
    fn aoc2016_10_correctness() -> io::Result<()> {
        let sol = AoC2016_10::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}