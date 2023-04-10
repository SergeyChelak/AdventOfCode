use crate::solution::Solution;
use crate::utils::*;

use std::fs::read_to_string;
use std::io;

enum Direction {
    Left(i32),
    Right(i32)
}

impl Direction {
    fn with_str(s: &str) -> Self {
        let dir = &s[..=0];
        let steps = s[1..].parse::<i32>().expect("Incorrect input format");
        match dir {
            "L" => Direction::Left(steps),
            "R" => Direction::Right(steps),
            _ => panic!("unexpected direction {}", dir)
        }
    }
}

pub struct AoC2016_01 {
    input: Vec<Direction>
}

impl AoC2016_01 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: Self::parse_input("input/aoc2016_01")?
        })
    }

    fn parse_input(file: &str) -> io::Result<Vec<Direction>> {
        Ok(read_to_string(file)?
            .trim()
            .split(", ")
            .map(|token| Direction::with_str(token))
            .collect::<Vec<Direction>>()
        )
    }
}

impl Solution for AoC2016_01 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2016/Day 1: No Time for a Taxicab".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_01_input_load_test() -> io::Result<()> {
        let sol = AoC2016_01::new()?;
        assert!(sol.input.len() > 0);
        Ok(())
    }

    #[test]
    fn aoc2016_01_correctness() -> io::Result<()> {
        let sol = AoC2016_01::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}