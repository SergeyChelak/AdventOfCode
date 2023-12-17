use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = u32;

pub struct AoC2023_17 {
    map: Vec<Vec<Int>>,
}

impl AoC2023_17 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_17")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let map = lines
            .iter()
            .map(|s| {
                s.chars()
                    .map(|ch| ch.to_digit(10).expect("Digit is expected"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { map }
    }
}

impl Solution for AoC2023_17 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 17: Clumsy Crucible".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_17_input_load_test() -> io::Result<()> {
        let sol = AoC2023_17::new()?;
        assert!(!sol.map.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_17_correctness() -> io::Result<()> {
        let sol = AoC2023_17::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
