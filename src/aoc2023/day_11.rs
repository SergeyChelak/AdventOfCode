use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
use std::io;

type Int = i32;
type Location = (Int, Int);

pub struct AoC2023_11 {
    locations: Vec<Location>,
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
        let mut empty_rows = HashSet::new();
        let mut empty_cols = HashSet::new();
        for i in 0usize..cols {
            empty_cols.insert(i);
        }
        let mut locations: Vec<Location> = Vec::new();
        for (row, line) in lines.iter().enumerate() {
            let size_before = locations.len();
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    locations.push((row as Int, col as Int));
                    empty_cols.remove(&col);
                }
            }
            if locations.len() == size_before {
                empty_rows.insert(row);
            }
        }

        rows += empty_rows.len();
        cols += empty_cols.len();

        for row in empty_rows {
            locations
                .iter_mut()
                .filter(|l| l.0 > row as Int)
                .for_each(|l| l.0 += 1);
        }

        for col in empty_cols {
            locations
                .iter_mut()
                .filter(|l| l.1 > col as Int)
                .for_each(|l| l.1 += 1);
        }

        Self {
            locations,
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
        let mut seen = HashSet::new(); //from([self.locations[from]]);
        let mut steps = 0;
        let mut current = vec![self.locations[from]];
        'bfs: while !current.is_empty() {
            steps += 1;
            let mut next = Vec::new();
            for (row, col) in current {
                seen.insert((row, col));
                let adjacent = [
                    (row + 1, col),
                    (row - 1, col),
                    (row, col + 1),
                    (row, col - 1),
                ];
                for loc in adjacent {
                    if loc == self.locations[to] {
                        cache.insert((from, to), steps);
                        break 'bfs;
                    }
                    if !(0..self.rows).contains(&(loc.0 as usize))
                        || !(0..self.cols).contains(&(loc.1 as usize))
                    {
                        continue;
                    }
                    if seen.contains(&loc) {
                        continue;
                    }
                    if let Some(index) = self.locations.iter().position(|elem| *elem == loc) {
                        let a = from.min(index);
                        let b = from.max(index);
                        cache.insert((a, b), steps);
                        seen.insert(loc);
                        continue;
                    }
                    if next.contains(&loc) {
                        continue;
                    }
                    next.push(loc)
                }
            }
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
                // print!("{} -> {}", from + 1, to + 1);
                let val = self.path_len(from, to, &mut cache);
                // print!(" calculated {val}");
                sum += val;
                // if let Some(cached) = cache.get(&(from, to)) {
                //     print!(" cached {cached}");
                // }
                // println!()
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
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
