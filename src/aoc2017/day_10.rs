use crate::solution::Solution;

use std::{io, fs::read_to_string};

pub struct AoC2017_10 {
    // place required fields here
}

impl AoC2017_10 {
    pub fn new() -> io::Result<Self> {
        _ = read_to_string("input/aoc2017_10")?;
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2017_10 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 10: Knot Hash".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_10_input_load_test() -> io::Result<()> {
        let sol = AoC2017_10::new()?;
        Ok(())
    }

    #[test]
    fn aoc2017_10_correctness() -> io::Result<()> {
        let sol = AoC2017_10::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}