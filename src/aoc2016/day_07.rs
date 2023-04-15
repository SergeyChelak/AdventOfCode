use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2016_07 {
    input: Vec<String>
}

impl AoC2016_07 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: read_file_as_lines("input/aoc2016_07")?
        })
    }
}

impl Solution for AoC2016_07 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2016/Day 7: Internet Protocol Version 7".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_07_input_load_test() -> io::Result<()> {
        let sol = AoC2016_07::new()?;
        assert!(sol.input.len() > 0);
        Ok(())
    }

    #[test]
    fn aoc2016_07_correctness() -> io::Result<()> {
        let sol = AoC2016_07::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}