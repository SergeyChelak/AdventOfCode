use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = usize;

pub struct AoC2022_01 {
    input: Vec2<Int>,
}

impl AoC2022_01 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2022_01")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        let input = data
            .split("\n\n")
            .map(|values| {
                values
                    .split('\n')
                    .map(|x| x.trim())
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<Int>().expect("Invalid calorie value"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2022_01 {
    fn part_one(&self) -> String {
        self.input
            .iter()
            .map(|arr| arr.iter().sum::<Int>())
            .max()
            .map(|x| x.to_string())
            .unwrap_or(not_found())
    }

    fn part_two(&self) -> String {
        let mut array = self
            .input
            .iter()
            .map(|arr| arr.iter().sum::<Int>())
            .collect::<Vec<_>>();
        array.sort();
        array.into_iter().rev().take(3).sum::<Int>().to_string()
    }

    fn description(&self) -> String {
        "Day 1: Calorie Counting".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_01_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2022_01_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "66719");
        Ok(())
    }

    #[test]
    fn aoc2022_01_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "198551");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2022_01> {
        AoC2022_01::new()
    }
}
