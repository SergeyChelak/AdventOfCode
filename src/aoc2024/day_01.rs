use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

type Int = i32;

pub struct AoC2024_01 {
    list_a: Vec<Int>,
    list_b: Vec<Int>,
}

impl AoC2024_01 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2024_01")?;
        Ok(Self::new_with(&lines))
    }

    fn new_with(lines: &[String]) -> Self {
        let mut list_a = Vec::<Int>::with_capacity(lines.len());
        let mut list_b = Vec::<Int>::with_capacity(lines.len());
        for line in lines {
            let mut split = line
                .split_whitespace()
                .map(|x| x.parse::<Int>().expect("Failed to parse {x}"));
            let (Some(a), Some(b)) = (split.next(), split.next()) else {
                panic!("Failed to split {line}");
            };
            list_a.push(a);
            list_b.push(b);
        }
        Self { list_a, list_b }
    }
}

impl Solution for AoC2024_01 {
    fn part_one(&self) -> String {
        let mut a_sorted = self.list_a.clone();
        a_sorted.sort();
        let mut b_sorted = self.list_b.clone();
        b_sorted.sort();

        a_sorted
            .iter()
            .zip(b_sorted.iter())
            .map(|(a, b)| a.abs_diff(*b))
            .sum::<u32>()
            .to_string()
    }

    fn part_two(&self) -> String {
        let mut times = HashMap::<Int, usize>::new();
        for x in &self.list_b {
            let entry = times.entry(*x).or_insert(0);
            *entry += 1;
        }
        let mut sum = 0;
        for x in &self.list_a {
            let rate = *times.get(x).unwrap_or(&0);
            sum += *x * rate as Int
        }
        sum.to_string()
    }

    fn description(&self) -> String {
        "2024/Day 1: Historian Hysteria".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_01_input_load_test() -> io::Result<()> {
        let sol = AoC2024_01::new()?;
        assert!(!sol.list_a.is_empty());
        assert!(!sol.list_b.is_empty());
        assert_eq!(sol.list_a.len(), sol.list_b.len());
        Ok(())
    }

    #[test]
    fn aoc2024_01_correctness() -> io::Result<()> {
        let sol = AoC2024_01::new()?;
        assert_eq!(sol.part_one(), "1590491");
        assert_eq!(sol.part_two(), "22588371");
        Ok(())
    }

    #[test]
    fn aoc2024_01_case_01() {
        let puzzle = create_puzzle();
        assert_eq!(puzzle.part_one(), "11")
    }

    #[test]
    fn aoc2024_01_case_02() {
        let puzzle = create_puzzle();
        assert_eq!(puzzle.part_two(), "31")
    }

    fn create_puzzle() -> AoC2024_01 {
        #[rustfmt::skip]
        let input = [
            "3   4",
            "4   3",
            "2   5",
            "1   3",
            "3   9",
            "3   3",
        ].as_strings();
        AoC2024_01::new_with(&input)
    }
}
