use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2016_06 {
    lines: Vec<String>
}

impl AoC2016_06 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2016_06")?;
        Ok(Self {
            lines
        })
    }
}

impl Solution for AoC2016_06 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2016/Day 6: Signals and Noise".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_06_input_load_test() -> io::Result<()> {
        let sol = AoC2016_06::new()?;
        assert!(sol.lines.len() > 0);
        Ok(())
    }

    #[test]
    fn aoc2016_06_correctness() -> io::Result<()> {
        let sol = AoC2016_06::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}