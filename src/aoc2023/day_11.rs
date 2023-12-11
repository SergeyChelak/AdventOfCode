use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
use std::io;

type Int = usize;
type Location = (Int, Int);

pub struct AoC2023_11 {
    locations: Vec<Location>,
    galaxy: HashMap<Location, usize>,
    rows: usize,
    cols: usize,
}

impl AoC2023_11 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_11")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let mut rows = lines.len();
        let mut cols = lines[0].len();
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

        rows += is_empty_row.len();
        cols += is_empty_col.len();

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

        let galaxy = HashMap::from_iter(
            locations
                .iter()
                .enumerate()
                .map(|(idx, value)| (*value, idx)),
        );
        Self {
            locations,
            galaxy,
            rows,
            cols,
        }
    }

    fn path_len(
        &self,
        from: usize,
        to: usize,
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        let mut seen = HashSet::from([self.locations[from]]);
        let mut steps = 0;
        let mut current = HashSet::from([self.locations[from]]);
        'bfs: while !current.is_empty() {
            let mut next = HashSet::new();
            for item in current {
                if let Some(&idx) = self.galaxy.get(&item) {
                    let start = from.min(idx);
                    let end = from.max(idx);
                    cache.insert((start, end), steps);
                    if idx == to {
                        break 'bfs;
                    }
                }
                let mut adjacent = Vec::new();
                let (row, col) = (item.0, item.1);
                if row > 0 {
                    adjacent.push((row - 1, col));
                }
                if row < self.rows - 1 {
                    adjacent.push((row + 1, col));
                }
                if col > 0 {
                    adjacent.push((row, col - 1));
                }
                if col < self.cols - 1 {
                    adjacent.push((row, col + 1));
                }
                for point in adjacent {
                    if seen.contains(&point) {
                        continue;
                    }
                    seen.insert(item);
                    next.insert(point);
                }
            }
            steps += 1;
            current = next;
        }
        steps
    }
}

impl Solution for AoC2023_11 {
    fn part_one(&self) -> String {
        let len = self.locations.len();
        let mut sum = 0;
        let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
        for from in 0..len - 1 {
            for to in from + 1..len {
                if let Some(cached) = cache.get(&(from, to)) {
                    sum += cached;
                } else {
                    sum += self.path_len(from, to, &mut cache);
                }
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
        assert_ne!(sol.rows, 0);
        assert_ne!(sol.cols, 0);
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
