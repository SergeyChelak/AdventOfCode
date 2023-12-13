use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2023_13 {
    // place required fields here
}

impl AoC2023_13 {
    pub fn new() -> io::Result<Self> {
        let _ = read_file_as_chars("input/aoc2023_13")?;
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2023_13 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 13: Point of Incidence".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_13_input_load_test() -> io::Result<()> {
        let sol = AoC2023_13::new()?;
        Ok(())
    }

    #[test]
    fn aoc2023_13_correctness() -> io::Result<()> {
        let sol = AoC2023_13::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
