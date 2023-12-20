use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2023_20 {
    // place required fields here
}

impl AoC2023_20 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_20")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        todo!()
    }
}

impl Solution for AoC2023_20 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 20: Pulse Propagation".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_20_input_load_test() -> io::Result<()> {
        let sol = AoC2023_20::new()?;
        Ok(())
    }

    #[test]
    fn aoc2023_20_correctness() -> io::Result<()> {
        let sol = AoC2023_20::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
