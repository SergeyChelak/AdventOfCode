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

    fn input_iter(&self) -> impl Iterator<Item = i32> + '_ {
        self.input.iter().map(|ch| if *ch == '(' { 1 } else { -1 })
    }
}

impl Solution for AoC2015_01 {
    fn part_one(&self) -> String {
        self.input_iter().sum::<i32>().to_string()
    }

    fn part_two(&self) -> String {
        let mut level = 0;
        let mut index: Option<usize> = None;
        for (i, val) in self.input_iter().enumerate() {
            level += val;
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
