use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2017_23 {
    // place required fields here
}

impl AoC2017_23 {
    pub fn new() -> io::Result<Self> {
        _ = read_file_as_lines("input/aoc2017_23")?;
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2017_23 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 23: Coprocessor Conflagration".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_23_input_load_test() -> io::Result<()> {
        let sol = AoC2017_23::new()?;
        Ok(())
    }

    #[test]
    fn aoc2017_23_correctness() -> io::Result<()> {
        let sol = AoC2017_23::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}