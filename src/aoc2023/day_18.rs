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

type Int = i32;

struct PlanItem {
    direction: Direction,
    depth: Int,
    color: usize,
}

impl From<&str> for PlanItem {
    fn from(value: &str) -> Self {
        let tokens = value.split(' ').collect::<Vec<_>>();
        let direction = Direction::from(tokens[0]);
        let depth = tokens[1].parse::<Int>().expect("Depth should be integer");
        let color = {
            let sub_str = tokens[2];
            usize::from_str_radix(&sub_str[2..sub_str.len() - 1], 16)
                .expect("Color should be a hex number")
        };
        Self {
            direction,
            depth,
            color,
        }
    }
}

type Position = (Int, Int);
type GroundMap = HashMap<Position, usize>;

pub struct AoC2023_18 {
    map: GroundMap,
    plan: Vec<PlanItem>,
    row_min: Int,
    row_max: Int,
    col_min: Int,
    col_max: Int,
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

        let mut row_min = Int::MAX;
        let mut row_max: Int = 0;
        let mut col_min = Int::MAX;
        let mut col_max: Int = 0;
        let (mut row, mut col) = (0, 0);
        let mut map = HashMap::new();
        for item in &plan {
            let (dr, dc) = match item.direction {
                Direction::Up => (-1, 0),
                Direction::Down => (1, 0),
                Direction::Left => (0, -1),
                Direction::Right => (0, 1),
            };
            for _ in 0..item.depth {
                row += dr;
                col += dc;
                map.insert((row, col), item.color);
            }
            row_min = row_min.min(row);
            row_max = row_max.max(row);
            col_min = col_min.min(col);
            col_max = col_max.max(col);
        }
        Self {
            map,
            plan,
            row_min,
            row_max,
            col_min,
            col_max,
        }
    }

    fn dump(&self) {
        for row in self.row_min - 1..=self.row_max + 1 {
            for col in self.col_min - 1..=self.col_max + 1 {
                if self.map.get(&(row, col)).is_some() {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!()
        }
    }
}

impl Solution for AoC2023_18 {
    fn part_one(&self) -> String {
        let row_range = self.row_min - 1..=self.row_max + 1;
        let col_range = self.col_min - 1..=self.col_max + 1;

        let start = (self.row_min - 1, self.col_min - 1);
        let mut deque = VecDeque::from([start]);
        let mut seen = HashSet::from([start]);
        while !deque.is_empty() {
            let item = deque.pop_front().unwrap();

            [(0, 1), (0, -1), (1, 0), (-1, 0)]
                .iter()
                .map(|(dr, dc)| (item.0 + dr, item.1 + dc))
                .filter(|elem| row_range.contains(&elem.0) && col_range.contains(&elem.1))
                .filter(|elem| self.map.get(elem).is_none())
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
            - seen.len() as i32;
        square.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 18: Lavaduct Lagoon".to_string()
    }
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
        puzzle.dump();
        assert_eq!(puzzle.part_one(), "62")
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
