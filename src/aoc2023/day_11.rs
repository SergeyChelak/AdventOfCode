use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = usize;
type Location = (Int, Int);

pub struct AoC2023_11 {
    locations: Vec<Location>,
    is_empty_row: Vec<bool>,
    is_empty_col: Vec<bool>,
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
        Self {
            locations,
            is_empty_row,
            is_empty_col,
        }
    }

    fn calculate(&self, sparse: usize) -> usize {
        // expand galaxy
        let mut locations = self.locations.clone();
        for (row, _) in self
            .is_empty_row
            .iter()
            .enumerate()
            .filter(|(_, val)| **val)
            .rev()
        {
            locations
                .iter_mut()
                .filter(|elem| elem.0 > row)
                .for_each(|elem| elem.0 += sparse);
        }

        for (col, _) in self
            .is_empty_col
            .iter()
            .enumerate()
            .filter(|(_, val)| **val)
            .rev()
        {
            locations
                .iter_mut()
                .filter(|elem| elem.1 > col)
                .for_each(|elem| elem.1 += sparse);
        }

        let mut sum = 0;
        let count = locations.len();
        for from in 0..count - 1 {
            for to in from + 1..count {
                let (row_a, col_a) = locations[from];
                let (row_b, col_b) = locations[to];
                sum += row_a.abs_diff(row_b) + col_a.abs_diff(col_b);
            }
        }
        sum
    }
}

impl Solution for AoC2023_11 {
    fn part_one(&self) -> String {
        self.calculate(1).to_string()
    }

    fn part_two(&self) -> String {
        self.calculate(1000000 - 1).to_string()
    }

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
        assert_eq!(puzzle_example().part_one(), "374")
    }

    #[test]
    fn aoc2023_11_ex2() {
        assert_eq!(puzzle_example().calculate(10 - 1), 1030);
        assert_eq!(puzzle_example().calculate(100 - 1), 8410);
    }

    fn puzzle_example() -> AoC2023_11 {
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
        AoC2023_11::with_lines(&lines)
    }

    #[test]
    fn aoc2023_11_correctness() -> io::Result<()> {
        let sol = AoC2023_11::new()?;
        assert_eq!(sol.part_one(), "9521550");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
