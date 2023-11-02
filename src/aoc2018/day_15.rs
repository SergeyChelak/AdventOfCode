use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2018_15 {
    // place required fields here
}

impl AoC2018_15 {
    pub fn new() -> io::Result<Self> {
        _ = read_file_as_lines("input/aoc2018_15")?;
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2018_15 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 15: Beverage Bandits".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_15_input_load_test() -> io::Result<()> {
        let sol = AoC2018_15::new()?;
        Ok(())
    }

    #[test]
    fn aoc2018_15_correctness() -> io::Result<()> {
        let sol = AoC2018_15::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}