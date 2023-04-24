use crate::solution::Solution;
use std::fs::read_to_string;

use std::io;

pub struct AoC2016_18 {
    row: Vec<bool>,
}

impl AoC2016_18 {
    pub fn new() -> io::Result<Self> {
        let row = read_to_string("input/aoc2016_18")?.trim().to_string();
        Ok(Self::with_str(&row))
    }

    fn with_str(s: &str) -> Self {
        let row = s.chars().map(|ch| ch == '.').collect::<Vec<bool>>();
        Self { row }
    }
}

impl Solution for AoC2016_18 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2016/Day 18: Like a Rogue".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_18_input_load_test() -> io::Result<()> {
        let sol = AoC2016_18::new()?;
        assert!(!sol.row.len() > 0);
        Ok(())
    }

    #[test]
    fn aoc2016_18_correctness() -> io::Result<()> {
        let sol = AoC2016_18::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
