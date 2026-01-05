use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, VecDeque};
use std::io;

type Int = u32;

pub struct AoC2021_15 {
    input: Vec2<Int>,
}

impl AoC2021_15 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2021_15")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(|x| {
                x.chars()
                    .map(|ch| ch.to_digit(10).expect("Input must be digits only"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { input }
    }
}

type Point = Point2d<usize>;

impl Solution for AoC2021_15 {
    fn part_one(&self) -> String {
        let rows = self.input.len();
        let cols = self.input[0].len();

        let mut dp = HashMap::<Point, Int>::new();
        let start = Point::zero();
        dp.insert(start, 0);

        let mut queue = VecDeque::new();
        queue.push_front(start);

        while let Some(point) = queue.pop_back() {
            let weight = dp.get(&point).copied().unwrap();

            for adj in Direction::all()
                .iter()
                .filter_map(|dir| point.safe_moved_by(dir))
                .filter(|p| p.x < cols && p.y < rows)
            {
                let tmp = weight + self.input[adj.y][adj.x];
                let is_better = if let Some(adj_weight) = dp.get(&adj) {
                    *adj_weight > tmp
                } else {
                    true
                };
                if is_better {
                    dp.insert(adj, tmp);
                    queue.push_front(adj);
                }
            }
        }

        dp.get(&Point::new(cols - 1, rows - 1))
            .map(|x| x.to_string())
            .unwrap_or(not_found())
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 15: Chiton".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_15_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2021_15_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "429");
        Ok(())
    }

    #[test]
    fn aoc2021_15_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2021_15> {
        AoC2021_15::new()
    }
}
