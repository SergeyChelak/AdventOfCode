use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2018_19 {
    // place required fields here
}

impl AoC2018_19 {
    pub fn new() -> io::Result<Self> {
        let _ = "input/aoc2018_19";
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2018_19 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 19: Go With The Flow".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_19_input_load_test() -> io::Result<()> {
        let sol = AoC2018_19::new()?;
        Ok(())
    }

    #[test]
    fn aoc2018_19_correctness() -> io::Result<()> {
        let sol = AoC2018_19::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
