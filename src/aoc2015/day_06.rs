use crate::solution::Solution;
use crate::file_utils::*;

use std::io;

pub struct AoC2015_06 {
    // place required fields here
}

impl AoC2015_06 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            // initialize solution
        })
    }

}
impl Solution for AoC2015_06 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2015/Day 6: Probably a Fire Hazard".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc015_06_input_load_test() -> io::Result<()> {
        Ok(())
    }

    #[test]
    fn aoc2015_06_correctness() -> io::Result<()> {
        Ok(())
    }
}