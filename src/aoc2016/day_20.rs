use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2016_20 {
    // place required fields here
}

impl AoC2016_20 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2016_20 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2016/Day 20: Firewall Rules".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_20_input_load_test() -> io::Result<()> {
        let sol = AoC2016_20::new()?;
        Ok(())
    }

    #[test]
    fn aoc2016_20_correctness() -> io::Result<()> {
        let sol = AoC2016_20::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}