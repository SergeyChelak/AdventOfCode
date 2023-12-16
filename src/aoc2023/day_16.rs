use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2023_16 {
    // place required fields here
}

impl AoC2023_16 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_16")?;
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2023_16 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 16: The Floor Will Be Lava".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_16_input_load_test() -> io::Result<()> {
        let sol = AoC2023_16::new()?;
        Ok(())
    }

    #[test]
    fn aoc2023_16_correctness() -> io::Result<()> {
        let sol = AoC2023_16::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
