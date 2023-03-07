use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2015_16 {
    // place required fields here
}

impl AoC2015_16 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2015_16")?;
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2015_16 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2015/Day 16: Aunt Sue".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_16_input_load_test() -> io::Result<()> {
        let sol = AoC2015_16::new()?;
        Ok(())
    }

    #[test]
    fn aoc2015_16_correctness() -> io::Result<()> {
        let sol = AoC2015_16::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}