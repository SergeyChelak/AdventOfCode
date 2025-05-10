use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashSet, VecDeque};
use std::io;

type Position = Point2d<usize>;

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
        let mut start = Position { y: 0, x: 0 };
        let mut map = Vec::new();
        for (r, line) in lines.iter().enumerate() {
            let mut row = Vec::new();
            for (c, ch) in line.chars().enumerate() {
                if ch == '#' {
                    row.push(MapElement::Rock);
                } else {
                    if ch == 'S' {
                        start = Position { y: r, x: c };
                    }
                    row.push(MapElement::Plot);
                }
            }
            map.push(row);
        }
        Self { map, start }
    }

    fn search(&self, start: Position, max_steps: usize) -> usize {
        let mut deque = VecDeque::from([(start, max_steps)]);
        let mut seen = HashSet::from([start]);
        let mut ans = HashSet::new();
        while let Some((item, step)) = deque.pop_front() {
            if step % 2 == 0 {
                ans.insert(item);
            }
            if step == 0 {
                continue;
            }
            let adjacent = Direction::all()
                .iter()
                .filter_map(|dir| item.safe_moved_by(dir))
                .filter(|p| p.y < self.map.len() && p.x < self.map[item.y].len())
                .collect::<Vec<_>>();
            for adj in adjacent {
                if matches!(self.map[adj.y][adj.x], MapElement::Rock) || seen.contains(&adj) {
                    continue;
                }
                seen.insert(adj);
                deque.push_back((adj, step - 1));
            }
        }
        ans.len()
    }
}

impl Solution for AoC2023_21 {
    fn part_one(&self) -> String {
        self.search(self.start, 64).to_string()
    }

    fn part_two(&self) -> String {
        let steps = 26501365;
        let size = self.map.len();
        assert_eq!(size, self.map[0].len());
        assert_eq!(size / 2, self.start.x);
        assert_eq!(size / 2, self.start.y);
        assert_eq!(steps % size, size / 2);

        let grid_width = steps / size - 1;
        let odd = (grid_width / 2 * 2 + 1).pow(2);
        let even = ((grid_width + 1) / 2 * 2).pow(2);

        let odd_points = self.search(self.start, size * 2 + 1);
        let even_points = self.search(self.start, size * 2);

        let corner_t = self.search(Position::new(self.start.x, size - 1), size - 1);
        let corner_r = self.search(Position::new(0, self.start.y), size - 1);
        let corner_b = self.search(Position::new(self.start.x, 0), size - 1);
        let corner_l = self.search(Position::new(size - 1, self.start.y), size - 1);

        let small_tr = self.search(Position::new(0, size - 1), size / 2 - 1);
        let small_tl = self.search(Position::new(size - 1, size - 1), size / 2 - 1);
        let small_br = self.search(Position::zero(), size / 2 - 1);
        let small_bl = self.search(Position::new(size - 1, 0), size / 2 - 1);

        let large_tr = self.search(Position::new(0, size - 1), size * 3 / 2 - 1);
        let large_tl = self.search(Position::new(size - 1, size - 1), size * 3 / 2 - 1);
        let large_br = self.search(Position::zero(), size * 3 / 2 - 1);
        let large_bl = self.search(Position::new(size - 1, 0), size * 3 / 2 - 1);

        let result = odd * odd_points
            + even * even_points
            + corner_t
            + corner_r
            + corner_b
            + corner_l
            + (grid_width + 1) * (small_tr + small_tl + small_br + small_bl)
            + grid_width * (large_tr + large_tl + large_br + large_bl);
        result.to_string()
    }

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
        assert_eq!(puzzle.search(puzzle.start, 6), 16);
    }

    #[test]
    fn aoc2023_21_correctness() -> io::Result<()> {
        let sol = AoC2023_21::new()?;
        assert_eq!(sol.part_one(), "3532");
        assert_eq!(sol.part_two(), "590104708070703");
        Ok(())
    }
}
