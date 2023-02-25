use crate::solution::Solution;
use crate::file_utils::*;

use std::io;

pub struct AoC2015_05 {
    input: Vec<String>,
}

impl AoC2015_05 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: read_file_as_lines("input/aoc2015_05")?
        })
    }
}

impl Solution for AoC2015_05 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    // fn description(&self) -> String {
    // "".to_string()
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_05_input_load_test() -> io::Result<()> {
        let sol = AoC2015_05::new()?;
        assert!(sol.input.len() > 0);
        Ok(())
    }

    #[test]
    fn aoc2015_05_correctness() -> io::Result<()> {
        Ok(())
    }
}