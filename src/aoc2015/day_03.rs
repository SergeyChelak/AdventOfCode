use crate::solution::Solution;
use crate::file_utils::*;

use std::io;

pub struct AoC2015_03 {
    input: Vec<char>,
}

impl AoC2015_03 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: read_file_as_chars("input/aoc2015_03")?,
        })
    }
}

impl Solution for AoC2015_03 {
    fn part_one(&self) -> String {
        "Part #1 isn't implemented yet".to_string()
    }

    fn part_two(&self) -> String {
        "Part #2 isn't implemented yet".to_string()
    }

    fn description(&self) -> String {
        "AoC 2015/Day 3".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc2015_03_input_load_test() -> io::Result<()> {
        let sol = AoC2015_03::new()?;
        assert!(sol.input.len() > 0);
        Ok(())
    }

    #[test]
    fn aoc2015_03_pt1_case1() {
        let result = AoC2015_03 {
            input: vec!['>']
        }.part_one();
        assert_eq!(result, "2");
    }

    #[test]
    fn aoc2015_03_pt1_case2() {
        let result = AoC2015_03 {
            input: vec!['^', '>', 'v', '<']
        }.part_one();
        assert_eq!(result, "4");
    }

    #[test]
    fn aoc2015_03_pt1_case3() {
        let result = AoC2015_03 {
            input: vec!['^', 'v', '^', 'v', '^', 'v', '^', 'v', '^', 'v']
        }.part_one();
        assert_eq!(result, "2");
    }
}
