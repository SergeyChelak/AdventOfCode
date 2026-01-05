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

        let dp = risk(
            |p| {
                if p.y < rows && p.x < cols {
                    return Some(self.input[p.y][p.x]);
                }
                None
            },
            Point::zero(),
        );
        dp.get(&Point::new(cols - 1, rows - 1))
            .map(|x| x.to_string())
            .unwrap_or(not_found())
    }

    fn part_two(&self) -> String {
        let tile_rows = self.input.len();
        let tile_cols = self.input[0].len();

        let rows = 5 * tile_rows;
        let cols = 5 * tile_cols;

        let dp = risk(
            |p| {
                if p.y >= rows || p.x >= cols {
                    return None;
                }
                let times_row = (p.y / tile_rows) as Int;
                let times_col = (p.x / tile_cols) as Int;

                let mut value = self.input[p.y % tile_rows][p.x % tile_cols];
                value = (value - 1 + times_col + times_row) % 9 + 1;
                Some(value)
            },
            Point::zero(),
        );
        dp.get(&Point::new(cols - 1, rows - 1))
            .map(|x| x.to_string())
            .unwrap_or(not_found())
    }

    fn description(&self) -> String {
        "Day 15: Chiton".to_string()
    }
}

fn risk(area: impl Fn(&Point) -> Option<Int>, start: Point) -> HashMap<Point, Int> {
    let mut dp = HashMap::<Point, Int>::new();
    dp.insert(start, 0);

    let mut queue = VecDeque::new();
    queue.push_front(start);
    while let Some(point) = queue.pop_back() {
        let weight = dp.get(&point).copied().unwrap();
        for (adj_p, adj_w) in Direction::all()
            .iter()
            .filter_map(|dir| point.safe_moved_by(dir))
            .filter_map(|p| {
                let w = area(&p)?;
                Some((p, w))
            })
        {
            let tmp = weight + adj_w;
            let is_better = if let Some(adj_weight) = dp.get(&adj_p) {
                *adj_weight > tmp
            } else {
                true
            };
            if is_better {
                dp.insert(adj_p, tmp);
                queue.push_front(adj_p);
            }
        }
    }
    dp
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
        assert_eq!(sol.part_two(), "2844");
        Ok(())
    }

    #[test]
    fn aoc2021_15_case2() {
        let sol = make_test_solution();
        assert_eq!(sol.part_two(), "315");
    }

    fn make_solution() -> io::Result<AoC2021_15> {
        AoC2021_15::new()
    }

    fn make_test_solution() -> AoC2021_15 {
        let lines = [
            "1163751742",
            "1381373672",
            "2136511328",
            "3694931569",
            "7463417111",
            "1319128137",
            "1359912421",
            "3125421639",
            "1293138521",
            "2311944581",
        ];
        AoC2021_15::parse_lines(&lines)
    }
}
