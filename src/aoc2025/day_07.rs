use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
use std::io;

type Int = usize;
type Point = Point2d<Int>;

pub struct AoC2025_07 {
    splitters: HashSet<Point>,
    start: Point,
    height: Int,
}

impl AoC2025_07 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2025_07")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut splitters = HashSet::new();
        let mut start = Point::zero();

        for (row, line) in lines.iter().enumerate().map(|(row, s)| (row, s.as_ref())) {
            for (col, ch) in line.chars().enumerate() {
                let point = Point::new(col, row);
                match ch {
                    'S' => {
                        start = point;
                    }
                    '^' => {
                        splitters.insert(point);
                    }
                    _ => {}
                }
            }
        }

        Self {
            splitters,
            start,
            height: lines.len(),
        }
    }
}

impl Solution for AoC2025_07 {
    fn part_one(&self) -> String {
        let mut beams = HashSet::new();
        beams.insert(self.start);
        let mut row = 0usize;
        let mut splits = 0;
        while row < self.height {
            let mut next = HashSet::new();
            for beam in beams {
                let tmp = beam.moved_by(&Direction::Down);
                if self.splitters.contains(&tmp) {
                    splits += 1;
                    let left = tmp.moved_by(&Direction::Left);
                    if !self.splitters.contains(&left) {
                        next.insert(left);
                    }
                    let right = tmp.moved_by(&Direction::Right);
                    if !self.splitters.contains(&right) {
                        next.insert(right);
                    }
                } else {
                    next.insert(tmp);
                }
            }
            beams = next;
            row += 1;
        }
        splits.to_string()
    }

    fn part_two(&self) -> String {
        fn dfs(
            current: Point,
            splitters: &HashSet<Point>,
            height: usize,
            memo: &mut HashMap<Point, usize>,
        ) -> usize {
            if let Some(value) = memo.get(&current) {
                return *value;
            }
            if current.y == height {
                return 1;
            }

            let tmp = current.moved_by(&Direction::Down);
            let value = if splitters.contains(&current) {
                let left = tmp.moved_by(&Direction::Left);
                let right = tmp.moved_by(&Direction::Right);
                dfs(left, splitters, height, memo) + dfs(right, splitters, height, memo)
            } else {
                dfs(tmp, splitters, height, memo)
            };

            memo.insert(current, value);
            value
        }

        dfs(
            self.start,
            &self.splitters,
            self.height,
            &mut HashMap::new(),
        )
        .to_string()
    }

    fn description(&self) -> String {
        "Day 7: Laboratories".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2025_07_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.splitters.is_empty());
        assert_ne!(sol.start, Point::zero());
        assert!(sol.height > 0);
        Ok(())
    }

    #[test]
    fn aoc2025_07_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "1640");
        Ok(())
    }

    #[test]
    fn aoc2025_07_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "40999072541589");
        Ok(())
    }

    #[test]
    fn aoc2025_07_case_1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "21");
    }

    #[test]
    fn aoc2025_07_case_2() {
        let sol = make_test_solution();
        assert_eq!(sol.part_two(), "40");
    }

    fn make_solution() -> io::Result<AoC2025_07> {
        AoC2025_07::new()
    }

    fn make_test_solution() -> AoC2025_07 {
        let lines = [
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ];
        AoC2025_07::parse_lines(&lines)
    }
}
