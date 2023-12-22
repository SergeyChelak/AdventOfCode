use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2023_22 {
    // place required fields here
}

impl AoC2023_22 {
    pub fn new() -> io::Result<Self> {
        let _ = "input/aoc2023_22";
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2023_22 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 22: Sand Slabs".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_22_input_load_test() -> io::Result<()> {
        let sol = AoC2023_22::new()?;
        Ok(())
    }

    #[test]
    fn aoc2023_22_correctness() -> io::Result<()> {
        let sol = AoC2023_22::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
