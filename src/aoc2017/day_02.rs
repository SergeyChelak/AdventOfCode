use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Value = u32;
type Grid = Vec<Vec<Value>>;

fn checksum(grid: &Grid) -> Value {
    grid.iter()
        .map(|row| {
            let max = row.iter().max().expect("Row shouldn't be empty");
            let min = row.iter().min().expect("Row shouldn't be empty");
            max - min
        })
        .sum()
}

pub struct AoC2017_02 {
    grid: Grid,
}

impl AoC2017_02 {
    pub fn new() -> io::Result<Self> {
        let grid = Self::parse_input("input/aoc2017_02")?;
        Ok(Self { grid })
    }

    fn parse_input(path: &str) -> io::Result<Grid> {
        let grid = read_file_as_lines(path)?
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|token| token.parse::<Value>().expect("Can't parse {token} value"))
                    .collect::<Vec<Value>>()
            })
            .collect();
        Ok(grid)
    }
}

impl Solution for AoC2017_02 {
    fn part_one(&self) -> String {
        checksum(&self.grid).to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 2: Corruption Checksum".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_02_input_load_test() -> io::Result<()> {
        let sol = AoC2017_02::new()?;
        assert!(!sol.grid.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_02_checksum() {
        let grid = vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]];
        assert_eq!(checksum(&grid), 18)
    }

    #[test]
    fn aoc2017_02_correctness() -> io::Result<()> {
        let sol = AoC2017_02::new()?;
        assert_eq!(sol.part_one(), "44216");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
