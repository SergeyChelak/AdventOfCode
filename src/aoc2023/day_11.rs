use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = usize;
type Location = (Int, Int);

pub struct AoC2023_11 {
    locations: Vec<Location>,
}

impl AoC2023_11 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_11")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let rows = lines.len();
        let cols = lines[0].len();
        let mut is_empty_row = vec![true; rows];
        let mut is_empty_col = vec![true; cols];
        let mut locations: Vec<Location> = Vec::new();
        for (row, line) in lines.iter().enumerate() {
            let size_before = locations.len();
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    locations.push((row as Int, col as Int));
                    is_empty_col[col] = false;
                }
            }
            if locations.len() > size_before {
                is_empty_row[row] = false;
            }
        }

        for (row, _) in is_empty_row
            .iter()
            .enumerate()
            .filter(|(_, val)| **val)
            .rev()
        {
            locations
                .iter_mut()
                .filter(|elem| elem.0 > row)
                .for_each(|elem| elem.0 += 1);
        }

        for (col, _) in is_empty_col
            .iter()
            .enumerate()
            .filter(|(_, val)| **val)
            .rev()
        {
            locations
                .iter_mut()
                .filter(|elem| elem.1 > col)
                .for_each(|elem| elem.1 += 1);
        }
        Self { locations }
    }

    fn distance(&self, from: usize, to: usize) -> usize {
        let (row_a, col_a) = self.locations[from];
        let (row_b, col_b) = self.locations[to];
        row_a.abs_diff(row_b) + col_a.abs_diff(col_b)
    }
}

impl Solution for AoC2023_11 {
    fn part_one(&self) -> String {
        let mut sum = 0;
        let count = self.locations.len();
        for from in 0..count - 1 {
            for to in from + 1..count {
                sum += self.distance(from, to);
            }
        }
        sum.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 11: Cosmic Expansion".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_11_input_load_test() -> io::Result<()> {
        let sol = AoC2023_11::new()?;
        assert!(!sol.locations.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_11_ex1() {
        let lines = [
            "...#......",
            ".......#..",
            "#.........",
            "..........",
            "......#...",
            ".#........",
            ".........#",
            "..........",
            ".......#..",
            "#...#.....",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let puzzle = AoC2023_11::with_lines(&lines);
        assert_eq!(puzzle.part_one(), "374")
    }

    #[test]
    fn aoc2023_11_correctness() -> io::Result<()> {
        let sol = AoC2023_11::new()?;
        assert_eq!(sol.part_one(), "9521550");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
