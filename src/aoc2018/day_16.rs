use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2018_16 {
    // place required fields here
}

impl AoC2018_16 {
    pub fn new() -> io::Result<Self> {
        _ = read_file_as_lines("input/aoc2018_16")?;
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2018_16 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 16: Chronal Classification".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_16_input_load_test() -> io::Result<()> {
        let sol = AoC2018_16::new()?;
        Ok(())
    }

    #[test]
    fn aoc2018_16_correctness() -> io::Result<()> {
        let sol = AoC2018_16::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}