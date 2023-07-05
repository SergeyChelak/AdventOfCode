use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2018_01 {
    // place required fields here
}

impl AoC2018_01 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2018_01 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 1: Chronal Calibration".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_01_input_load_test() -> io::Result<()> {
        let sol = AoC2018_01::new()?;
        Ok(())
    }

    #[test]
    fn aoc2018_01_correctness() -> io::Result<()> {
        let sol = AoC2018_01::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}