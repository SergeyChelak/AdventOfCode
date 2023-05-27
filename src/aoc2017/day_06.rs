use crate::solution::Solution;

use std::fs::read_to_string;
use std::io;

pub struct AoC2017_06 {
    banks: Vec<usize>,
}

impl AoC2017_06 {
    pub fn new() -> io::Result<Self> {
        let banks = read_to_string("input/aoc2017_06")?
            .split_whitespace()
            .map(|s| s.parse::<usize>().expect("Int value is expected"))
            .collect();
        Ok(Self {
            banks
        })
    }
}

impl Solution for AoC2017_06 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 6: Memory Reallocation".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_06_input_load_test() -> io::Result<()> {
        let sol = AoC2017_06::new()?;
        Ok(())
    }

    #[test]
    fn aoc2017_06_correctness() -> io::Result<()> {
        let sol = AoC2017_06::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}