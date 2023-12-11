use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

type Int = i32;
type Location = (Int, Int);

// fn dump_locations(locations: &[Location]) {
//     let mut cc = 0;
//     for loc in locations {
//         cc += 1;
//         let s = format!("{},{}", loc.0, loc.1);
//         print!("{s:10}");
//         if cc % 11 == 0 {
//             println!();
//         }
//     }
//     println!();
// }

pub struct AoC2023_11 {
    locations: HashSet<Location>,
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

        println!("Empty rows count: {}", empty_rows.len());
        println!("Empty cols count: {}", empty_cols.len());

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

        println!("Size {}x{}", rows, cols);

        Self {
            locations: HashSet::from_iter(locations),
            rows,
            cols,
        }
    }

    fn dump_map(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.locations.contains(&(row as Int, col as Int)) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!()
        }
    }
}

impl Solution for AoC2023_11 {
    // fn part_one(&self) -> String {
    // }

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
        puzzle.dump_map();
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
