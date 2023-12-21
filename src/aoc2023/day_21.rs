use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashSet, VecDeque};
use std::io;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    row: usize,
    col: usize,
}

enum MapElement {
    Plot,
    Rock,
}

pub struct AoC2023_21 {
    map: Vec<Vec<MapElement>>,
    start: Position,
}

impl AoC2023_21 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_21")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let mut start = Position { row: 0, col: 0 };
        let mut map = Vec::new();
        for (r, line) in lines.iter().enumerate() {
            let mut row = Vec::new();
            for (c, ch) in line.chars().enumerate() {
                if ch == '#' {
                    row.push(MapElement::Rock);
                } else {
                    if ch == 'S' {
                        start = Position { row: r, col: c };
                    }
                    row.push(MapElement::Plot);
                }
            }
            map.push(row);
        }
        Self { map, start }
    }

    fn search(&self, max_steps: usize) -> usize {
        let mut deque = VecDeque::from([(self.start, 0)]);
        let mut seen = HashSet::from([self.start]);
        let mut ans = HashSet::new();
        while let Some((item, step)) = deque.pop_front() {
            if step > max_steps {
                continue;
            }
            if step % 2 == 0 {
                ans.insert(item);
            }
            let Position { row, col } = item;
            let mut adjacent = Vec::new();
            if row > 0 {
                adjacent.push(Position { row: row - 1, col });
            }
            if row < self.map.len() - 1 {
                adjacent.push(Position { row: row + 1, col });
            }
            if col > 0 {
                adjacent.push(Position { row, col: col - 1 });
            }
            if col < self.map[row].len() - 1 {
                adjacent.push(Position { row, col: col + 1 });
            }
            for adj in adjacent {
                if matches!(self.map[adj.row][adj.col], MapElement::Rock) || seen.contains(&adj) {
                    continue;
                }
                seen.insert(adj);
                deque.push_back((adj, step + 1));
            }
        }
        ans.len()
    }
}

impl Solution for AoC2023_21 {
    fn part_one(&self) -> String {
        self.search(64).to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 21: Step Counter".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_21_input_load_test() -> io::Result<()> {
        let sol = AoC2023_21::new()?;
        assert!(!sol.map.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_21_ex1() {
        let lines = [
            "...........",
            ".....###.#.",
            ".###.##..#.",
            "..#.#...#..",
            "....#.#....",
            ".##..S####.",
            ".##..#...#.",
            ".......##..",
            ".##.#.####.",
            ".##..##.##.",
            "...........",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let puzzle = AoC2023_21::with_lines(&lines);
        assert_eq!(puzzle.search(6), 16);
    }

    #[test]
    fn aoc2023_21_correctness() -> io::Result<()> {
        let sol = AoC2023_21::new()?;
        assert_eq!(sol.part_one(), "3532");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
