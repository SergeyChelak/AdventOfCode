use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2017_18 {
    // place required fields here
}

impl AoC2017_18 {
    pub fn new() -> io::Result<Self> {
        read_file_as_lines("input/aoc2017_18")?;
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2017_18 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 18: Duet".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_18_input_load_test() -> io::Result<()> {
        let sol = AoC2017_18::new()?;
        Ok(())
    }

    #[test]
    fn aoc2017_18_correctness() -> io::Result<()> {
        let sol = AoC2017_18::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}