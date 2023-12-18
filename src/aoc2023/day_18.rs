use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet, VecDeque};
use std::io;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("A '{value}' is not expected direction value"),
        }
    }
}

type Int = i64;

struct PlanItem {
    direction: Direction,
    depth: Int,
    color: String,
}

impl From<&str> for PlanItem {
    fn from(value: &str) -> Self {
        let tokens = value.split(' ').collect::<Vec<_>>();
        let direction = Direction::from(tokens[0]);
        let depth = tokens[1].parse::<Int>().expect("Depth should be integer");
        Self {
            direction,
            depth,
            color: tokens[2].to_string(),
        }
    }
}

pub struct AoC2023_18 {
    plan: Vec<PlanItem>,
}

impl AoC2023_18 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_18")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let plan = lines
            .iter()
            .map(|s| PlanItem::from(s.as_str()))
            .collect::<Vec<PlanItem>>();
        Self { plan }
    }
}

impl Solution for AoC2023_18 {
    fn part_one(&self) -> String {
        square(&self.plan).to_string()
    }

    fn part_two(&self) -> String {
        let plan = self
            .plan
            .iter()
            .map(|item| {
                let color = &item.color[2..item.color.len() - 1];
                let depth = Int::from_str_radix(&color[..5], 16)
                    .expect("Color should be hexadecimal value");
                // The last hexadecimal digit encodes the direction to dig: 0 means R, 1 means D, 2 means L, and 3 means U.
                let direction = match &color[5..] {
                    "0" => Direction::Right,
                    "1" => Direction::Down,
                    "2" => Direction::Left,
                    "3" => Direction::Up,
                    val => panic!("Unexpected direction {val}"),
                };

                PlanItem {
                    depth,
                    direction,
                    color: "".to_string(),
                }
            })
            .collect::<Vec<_>>();
        square(&plan).to_string()
    }

    fn description(&self) -> String {
        "AoC 2023/Day 18: Lavaduct Lagoon".to_string()
    }
}

fn square(plan: &[PlanItem]) -> Int {
    let mut row_min = Int::MAX;
    let mut row_max: Int = 0;
    let mut col_min = Int::MAX;
    let mut col_max: Int = 0;

    let (mut row, mut col) = (0, 0);
    let mut map = HashMap::new();
    for item in plan {
        let (dr, dc) = match item.direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };
        for _ in 0..item.depth {
            row += dr;
            col += dc;
            map.insert((row, col), 0);
        }
        row_min = row_min.min(row);
        row_max = row_max.max(row);
        col_min = col_min.min(col);
        col_max = col_max.max(col);
    }

    let row_range = row_min - 1..=row_max + 1;
    let col_range = col_min - 1..=col_max + 1;

    let start = (row_min - 1, col_min - 1);
    let mut deque = VecDeque::from([start]);
    let mut seen = HashSet::from([start]);
    while !deque.is_empty() {
        let item = deque.pop_front().unwrap();

        [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .iter()
            .map(|(dr, dc)| (item.0 + dr, item.1 + dc))
            .filter(|elem| row_range.contains(&elem.0) && col_range.contains(&elem.1))
            .filter(|elem| map.get(elem).is_none())
            .for_each(|elem| {
                if seen.contains(&elem) {
                    return;
                }
                seen.insert(elem.clone());
                deque.push_back(elem.clone());
            });
    }
    let square = (row_range.end() - row_range.start() + 1)
        * (col_range.end() - col_range.start() + 1)
        - seen.len() as Int;
    square
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_18_input_load_test() -> io::Result<()> {
        let sol = AoC2023_18::new()?;
        assert!(!sol.plan.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_18_ex1() {
        let puzzle = puzzle();
        assert_eq!(puzzle.part_one(), "62")
    }

    #[test]
    fn aoc2023_18_ex2() {
        let puzzle = puzzle();
        assert_eq!(puzzle.part_two(), "952408144115")
    }

    fn puzzle() -> AoC2023_18 {
        let lines = [
            "R 6 (#70c710)",
            "D 5 (#0dc571)",
            "L 2 (#5713f0)",
            "D 2 (#d2c081)",
            "R 2 (#59c680)",
            "D 2 (#411b91)",
            "L 5 (#8ceee2)",
            "U 2 (#caa173)",
            "L 1 (#1b58a2)",
            "U 2 (#caa171)",
            "R 2 (#7807d2)",
            "U 3 (#a77fa3)",
            "L 2 (#015232)",
            "U 2 (#7a21e3)",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        AoC2023_18::with_lines(&lines)
    }

    #[test]
    fn aoc2023_18_correctness() -> io::Result<()> {
        let sol = AoC2023_18::new()?;
        assert_eq!(sol.part_one(), "106459");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
