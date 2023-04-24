use crate::solution::Solution;
use std::fs::read_to_string;

use std::io;

pub struct AoC2016_18 {
    row: Vec<bool>,
}

impl AoC2016_18 {
    pub fn new() -> io::Result<Self> {
        let row = read_to_string("input/aoc2016_18")?.trim().to_string();
        Ok(Self::with_str(&row))
    }

    fn with_str(s: &str) -> Self {
        let row = s.chars().map(|ch| ch == '.').collect::<Vec<bool>>();
        Self { row }
    }
}

impl Solution for AoC2016_18 {
    fn part_one(&self) -> String {
        let mut row = self.row.clone();
        let mut count = 0;
        for _ in 0..40 {
            count += safe_count(&row);
            row = next_row(&row);
        }
        count.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2016/Day 18: Like a Rogue".to_string()
    }
}

fn safe_count(row: &Vec<bool>) -> usize {
    row.iter().filter(|&item| *item).count()
}

fn next_row(row: &Vec<bool>) -> Vec<bool> {
    let len = row.len();
    let mut next = vec![false; len];
    for i in 0..len {
        let left = if i > 0 { row[i - 1] } else { true };
        let center = row[i];
        let right = if i < len - 1 { row[i + 1] } else { true };
        let is_trap = !left && !center && right
            || left && !center && !right
            || !left && center && right
            || left && center && !right;
        next[i] = !is_trap;
    }
    next
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_18_input_load_test() -> io::Result<()> {
        let sol = AoC2016_18::new()?;
        assert!(!sol.row.len() > 0);
        Ok(())
    }

    #[test]
    fn aoc2016_18_correctness() -> io::Result<()> {
        let sol = AoC2016_18::new()?;
        assert_eq!(sol.part_one(), "2016");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
