use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = i64;

struct Item {
    hand: String,
    bid: Int,
}

pub struct AoC2023_07 {
    input: Vec<Item>,
}

impl AoC2023_07 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_07")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let input = lines
            .iter()
            .filter_map(|s| {
                let (h, b) = s.split_once(' ')?;
                let bid = b.parse::<Int>().ok()?;
                Some(Item {
                    hand: h.to_string(),
                    bid,
                })
            })
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2023_07 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 7: Camel Cards".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_07_input_load_test() -> io::Result<()> {
        let sol = AoC2023_07::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_07_correctness() -> io::Result<()> {
        let sol = AoC2023_07::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
