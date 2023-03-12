use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2015_21 {
    // place required fields here
}

impl AoC2015_21 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2015_21 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2015/Day 21: RPG Simulator 20XX"
        .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_21_input_load_test() -> io::Result<()> {
        let sol = AoC2015_21::new()?;
        Ok(())
    }

    #[test]
    fn aoc2015_21_correctness() -> io::Result<()> {
        let sol = AoC2015_21::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}