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

    fn safe_cells_count(&self, rows: usize) -> usize {
        let mut row = self.row.clone();
        let mut count = 0;
        for _ in 0..rows {
            count += row.iter().filter(|&item| *item).count();
            row = next_row(&row);
        }
        count
    }
}

impl Solution for AoC2016_18 {
    fn part_one(&self) -> String {
        self.safe_cells_count(40).to_string()
    }

    fn part_two(&self) -> String {
        self.safe_cells_count(400000).to_string()
    }

    fn description(&self) -> String {
        "AoC 2016/Day 18: Like a Rogue".to_string()
    }
}

fn next_row(row: &[bool]) -> Vec<bool> {
    let len = row.len();
    let mut next = vec![false; len];
    for i in 0..len {
        let left = if i > 0 { row[i - 1] } else { true };
        let right = if i < len - 1 { row[i + 1] } else { true };
        let is_trap = !left && right || left && !right;
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
        assert_eq!(sol.part_two(), "19998750");
        Ok(())
    }
}
