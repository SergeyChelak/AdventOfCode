use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2015_18 {
    // place required fields here
}

impl AoC2015_18 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2015_18 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2015/Day 18: Like a GIF For Your Yard".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_18_input_load_test() -> io::Result<()> {
        let sol = AoC2015_18::new()?;
        Ok(())
    }

    #[test]
    fn aoc2015_18_correctness() -> io::Result<()> {
        let sol = AoC2015_18::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}