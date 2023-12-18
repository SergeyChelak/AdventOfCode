use crate::solution::Solution;
use crate::utils::*;

use std::io;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    // The last hexadecimal digit encodes the direction to dig: 0 means R, 1 means D, 2 means L, and 3 means U.
    fn from(value: &str) -> Self {
        match value {
            "3" | "U" => Self::Up,
            "1" | "D" => Self::Down,
            "2" | "L" => Self::Left,
            "0" | "R" => Self::Right,
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
        plan_square(&self.plan).to_string()
    }

    fn part_two(&self) -> String {
        let plan = self
            .plan
            .iter()
            .map(|item| {
                let color = &item.color[2..item.color.len() - 1];
                let depth = Int::from_str_radix(&color[..5], 16)
                    .expect("Color should be hexadecimal value");
                let direction = Direction::from(&color[5..]);
                PlanItem {
                    depth,
                    direction,
                    color: "".to_string(),
                }
            })
            .collect::<Vec<_>>();
        plan_square(&plan).to_string()
    }

    fn description(&self) -> String {
        "AoC 2023/Day 18: Lavaduct Lagoon".to_string()
    }
}

type Vertex = (Int, Int);

fn plan_square(plan: &[PlanItem]) -> Int {
    let list = vertex_list(plan);
    square(&list)
}

fn vertex_list(plan: &[PlanItem]) -> Vec<Vertex> {
    let mut vertex = vec![(0, 0)];
    let (mut row, mut col) = (0, 0);
    for item in plan {
        match item.direction {
            Direction::Up => row -= item.depth,
            Direction::Down => row += item.depth,
            Direction::Left => col -= item.depth,
            Direction::Right => col += item.depth,
        };
        vertex.push((row, col));
    }
    vertex
}

// Shoelace formula
fn square(vertex: &[Vertex]) -> Int {
    let n = vertex.len();
    let mut upper_sum = vertex[n - 1].0 * vertex[0].1;
    let mut lower_sum = vertex[0].0 * vertex[n - 1].1;
    for i in 0..n - 1 {
        upper_sum += vertex[i].0 * vertex[i + 1].1;
        lower_sum += vertex[i + 1].0 * vertex[i].1;
    }
    (upper_sum.abs_diff(lower_sum) >> 1) as Int
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

    #[test]
    fn gauss_square() {
        let vertex = [(3, 4), (5, 11), (12, 8), (9, 5), (5, 6)];
        assert_eq!(square(&vertex), 30);

        let vertex = [(-1000, 500), (-500, 1000), (2, 10), (35, 60)];
        assert_eq!(square(&vertex), 339865);

        let vertex = [
            (51, -20),
            (15, 3),
            (45, 200),
            (100, -100),
            (201, 55),
            (70, -80),
            (25, 333),
            (999, 0),
            (500, 77),
            (5, -6),
        ];
        assert_eq!(square(&vertex), 124562);

        let vertex = [(13, -92), (44, 0), (-800, 30), (27, 2), (1, 2)];
        assert_eq!(square(&vertex), 1446);
    }
}
