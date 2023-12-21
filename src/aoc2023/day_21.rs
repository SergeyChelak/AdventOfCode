use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2023_21 {
    // place required fields here
}

impl AoC2023_21 {
    pub fn new() -> io::Result<Self> {
        let _ = read_file_as_lines("input/aoc2023_21")?;
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2023_21 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 21: Step Counter".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_21_input_load_test() -> io::Result<()> {
        let sol = AoC2023_21::new()?;
        Ok(())
    }

    #[test]
    fn aoc2023_21_correctness() -> io::Result<()> {
        let sol = AoC2023_21::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
