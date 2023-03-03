use crate::solution::Solution;
use crate::file_utils::*;

use std::io;

pub struct AoC2015_12 {
    // place required fields here
}

impl AoC2015_12 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2015_12 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2015/Day 12: JSAbacusFramework.io".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_12_input_load_test() -> io::Result<()> {
        Ok(())
    }

    #[test]
    fn aoc2015_12_correctness() -> io::Result<()> {
        let sol = AoC2015_12::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}