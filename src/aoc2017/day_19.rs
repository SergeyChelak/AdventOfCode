use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Matrix = Vec<Vec<char>>;

pub struct AoC2017_19 {
    maze: Matrix
}

impl AoC2017_19 {
    pub fn new() -> io::Result<Self> {
        let maze = read_file_as_lines("input/aoc2017_19")?
            .iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Matrix>();
        Ok(Self {
            maze
        })
    }
}

impl Solution for AoC2017_19 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 19: A Series of Tubes".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_19_input_load_test() -> io::Result<()> {
        let sol = AoC2017_19::new()?;
        assert!(!sol.maze.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_19_correctness() -> io::Result<()> {
        let sol = AoC2017_19::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}