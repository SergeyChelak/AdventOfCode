use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2016_25 {
    // place required fields here
}

impl AoC2016_25 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2016_25 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2016/Day 25: Clock Signal".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_25_input_load_test() -> io::Result<()> {
        let sol = AoC2016_25::new()?;
        Ok(())
    }

    #[test]
    fn aoc2016_25_correctness() -> io::Result<()> {
        let sol = AoC2016_25::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}