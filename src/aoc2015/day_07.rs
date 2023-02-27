use crate::solution::Solution;
use crate::file_utils::*;

use std::io;

pub struct AoC2015_07 {
    // place required fields here
}

impl AoC2015_07 {
    pub fn new() -> io::Result<Self> {
        read_file_as_lines("input/aoc2015_07")?;
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2015_07 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
    	"AoC 2015/Day 7: Some Assembly Required".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_07_input_load_test() -> io::Result<()> {
        Ok(())
    }

    #[test]
    fn aoc2015_07_correctness() -> io::Result<()> {
        Ok(())
    }
}