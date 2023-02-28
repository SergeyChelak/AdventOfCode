use crate::solution::Solution;
use crate::file_utils::*;

use std::io;

pub struct AoC2015_10 {
    input: String
}

impl AoC2015_10 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: "3113322113".to_string()
        })
    }
}

impl Solution for AoC2015_10 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
    	"AoC 2015/Day 10: Elves Look, Elves Say".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_10_correctness() -> io::Result<()> {
        let sol = AoC2015_10::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}