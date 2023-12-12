use crate::solution::Solution;
use crate::utils::*;

use std::io;

struct GroupInfo {
    pattern: String,
    damaged_lengths: Vec<usize>,
}

impl From<&str> for GroupInfo {
    fn from(value: &str) -> Self {
        let (pattern, lengths) = value.split_once(' ').expect("Separator not found");
        let damaged_lengths = lengths
            .split(',')
            .map(|s| s.parse::<usize>().expect("Length should be integer"))
            .collect::<Vec<_>>();
        Self {
            pattern: pattern.to_string(),
            damaged_lengths,
        }
    }
}

pub struct AoC2023_12 {
    input: Vec<GroupInfo>,
}

impl AoC2023_12 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_12")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let input = lines
            .iter()
            .map(|s| GroupInfo::from(s.as_str()))
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2023_12 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 12: Hot Springs".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_12_input_load_test() -> io::Result<()> {
        let sol = AoC2023_12::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_12_correctness() -> io::Result<()> {
        let sol = AoC2023_12::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
