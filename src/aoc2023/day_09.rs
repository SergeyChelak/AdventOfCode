use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = i32;

pub struct AoC2023_09 {
    input: Vec<Vec<Int>>,
}

impl AoC2023_09 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_09")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let input = lines
            .iter()
            .map(|s| {
                s.split_whitespace()
                    .map(|val| val.parse::<Int>().expect("Int value is expected"))
                    .collect::<Vec<_>>()
            })
            .collect();
        Self { input }
    }
}

impl Solution for AoC2023_09 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 9: Mirage Maintenance".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_09_input_load_test() -> io::Result<()> {
        let sol = AoC2023_09::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_09_correctness() -> io::Result<()> {
        let sol = AoC2023_09::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
