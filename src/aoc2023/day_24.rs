use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2023_24 {
    // place required fields here
}

impl AoC2023_24 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_24")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        todo!()
    }
}

impl Solution for AoC2023_24 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 24: Never Tell Me The Odds".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_24_input_load_test() -> io::Result<()> {
        let sol = AoC2023_24::new()?;
        Ok(())
    }

    #[test]
    fn aoc2023_24_correctness() -> io::Result<()> {
        let sol = AoC2023_24::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
