use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

type Grid = Vec<Vec<char>>;

fn bfs_path_len(grid: &Grid, start: &Position, finish: &Position) -> usize {
    let mut steps = 0;
    let mut positions = vec![*start];
    let mut visited = HashSet::new();
    visited.insert(*start);
    'outer: while !positions.is_empty() {
        steps += 1;
        let mut next = Vec::new();
        for pos in positions {
            let (row, col) = (pos.row, pos.col);
            let mut adj = Vec::new();
            if row > 0 {
                adj.push(Position { row: row - 1, col })
            }
            if row < grid.len() - 1 {
                adj.push(Position { row: row + 1, col })
            }
            if col > 0 {
                adj.push(Position { row, col: col - 1 })
            }
            if col < grid[row].len() - 1 {
                adj.push(Position { row, col: col + 1 })
            }
            for p in adj {
                if grid[p.row][p.col] == '#' || visited.contains(&p) {
                    continue;
                }
                if p == *finish {
                    break 'outer;
                }
                visited.insert(p);
                next.push(p);
            }
        }
        positions = next;
    }
    steps
}

type Weights = Vec<Vec<usize>>;

pub struct AoC2016_24 {
    weights: Weights,
}

impl AoC2016_24 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2016_24")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let mut grid = Grid::new();
        let mut points = Vec::new();
        for row in 0..lines.len() {
            let chars = lines[row].chars().collect::<Vec<char>>();
            for col in 0..chars.len() {
                if chars[col].is_numeric() {
                    let number = chars[col].to_digit(10).expect("Numeric char expected");
                    let pos = Position { row, col };
                    points.push((number as usize, pos));
                }
            }
            grid.push(chars);
        }
        points.sort_by(|a, b| a.0.cmp(&b.0));
        let points = points
            .iter()
            .map(|(_, pos)| *pos)
            .collect::<Vec<Position>>();

        let len = points.len();
        let mut weights = vec![vec![usize::MAX; len]; len];
        for i in 0..len - 1 {
            for j in i + 1..len {
                let a = points[i];
                let b = points[j];
                let dist = bfs_path_len(&grid, &a, &b);
                weights[i][j] = dist;
                weights[j][i] = dist;
            }
        }
        Self { weights }
    }
}

impl Solution for AoC2016_24 {
    fn part_one(&self) -> String {
        let len = self.weights.len();
        let mut total = usize::MAX;
        (1..len)
            .collect::<Vec<usize>>()
            .permut_iter()
            .for_each(|arr| {
                let mut sum = 0usize;
                let mut prev = 0;
                for idx in arr {
                    sum += self.weights[prev][idx];
                    prev = idx;
                }
                total = total.min(sum);
            });
        total.to_string()
    }

    fn part_two(&self) -> String {
        let len = self.weights.len();
        let mut total = usize::MAX;
        (1..len)
            .collect::<Vec<usize>>()
            .permut_iter()
            .for_each(|arr| {
                let mut sum = 0usize;
                let mut prev = 0;
                for idx in arr {
                    sum += self.weights[prev][idx];
                    prev = idx;
                }
                sum += self.weights[prev][0];
                total = total.min(sum);
            });
        total.to_string()
    }

    fn description(&self) -> String {
        "AoC 2016/Day 24: Air Duct Spelunking".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_24_input_load_test() -> io::Result<()> {
        let sol = AoC2016_24::new()?;
        assert!(!sol.weights.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2016_24_correctness() -> io::Result<()> {
        let sol = AoC2016_24::new()?;
        assert_eq!(sol.part_one(), "464");
        assert_eq!(sol.part_two(), "652");
        Ok(())
    }

    #[test]
    fn aoc2016_24_example() {
        let lines = [
            "###########",
            "#0.1.....2#",
            "#.#######.#",
            "#4.......3#",
            "###########",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        let sol = AoC2016_24::with_lines(&lines);
        assert_eq!(sol.part_one(), "14");
    }
}
