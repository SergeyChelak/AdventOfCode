use std::io;

use crate::solution::*;
use crate::utils::*;

pub struct AoC2015_01 {
    input: Vec<char>,
}

impl AoC2015_01 {
    pub fn new() -> io::Result<Self> {
        let input = read_file_as_chars("input/aoc2015_01")?;
        Ok(Self { input })
    }
}

impl Solution for AoC2015_01 {
    fn part_one(&self) -> String {
        self.input
            .iter()
            .fold(0isize, |acc, val| acc + if *val == '(' { 1 } else { -1 })
            .to_string()
    }

    fn part_two(&self) -> String {
        let mut level = 0isize;
        let mut index: Option<usize> = None;
        for i in 0..self.input.len() {
            level += if self.input[i] == '(' { 1 } else { -1 };
            if level == -1 {
                index = Some(1 + i);
                break;
            }
        }
        if let Some(index) = index {
            index.to_string()
        } else {
            "Not found".to_string()
        }
    }

    fn description(&self) -> String {
        "AoC 2015/Day 1: Not Quite Lisp".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc2015_01_correctness() -> io::Result<()> {
        let sol = AoC2015_01::new()?;
        assert_eq!(sol.part_one(), "138".to_string());
        assert_eq!(sol.part_two(), "1771".to_string());
        Ok(())
    }
}
