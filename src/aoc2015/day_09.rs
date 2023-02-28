use crate::solution::Solution;
use crate::file_utils::*;

use std::io;

pub struct AoC2015_09 {
    // place required fields here
}

impl AoC2015_09 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2015_09 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
    	"AoC 2015/Day 9: All in a Single Night".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_09_input_load_test() -> io::Result<()> {
        Ok(())
    }

    #[test]
    fn aoc2015_09_correctness() -> io::Result<()> {
        let sol = AoC2015_09::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}