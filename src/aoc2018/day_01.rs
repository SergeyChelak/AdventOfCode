use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2018_01 {
    input: Vec<i32>,
}

impl AoC2018_01 {
    pub fn new() -> io::Result<Self> {
        let (input, _): (Vec<_>, Vec<_>) = read_file_as_lines("input/aoc2018_01")?
            .iter()
            .map(|s| s.parse::<i32>())
            .partition(Result::is_ok);
        let input = input.into_iter().map(Result::unwrap).collect();
        Ok(Self { input })
    }
}

impl Solution for AoC2018_01 {
    fn part_one(&self) -> String {
        self.input.iter().sum::<i32>().to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 1: Chronal Calibration".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_01_input_load_test() -> io::Result<()> {
        let sol = AoC2018_01::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_01_correctness() -> io::Result<()> {
        let sol = AoC2018_01::new()?;
        assert_eq!(sol.part_one(), "543");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
