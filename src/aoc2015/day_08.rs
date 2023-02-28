use crate::solution::Solution;
use crate::file_utils::*;

use std::io;

pub struct AoC2015_08 {
    input: Vec<String>,
}

impl AoC2015_08 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: read_file_as_lines("input/aoc2015_08")?
        })
    }
}

impl Solution for AoC2015_08 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
    	"AoC 2015/Day 8: Matchsticks".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_08_input_load_test() -> io::Result<()> {
        let sol = AoC2015_08::new()?;
        assert_eq!(sol.input.len(), 300);
        Ok(())
    }

    #[test]
    fn aoc2015_08_correctness() -> io::Result<()> {
        let sol = AoC2015_08::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}