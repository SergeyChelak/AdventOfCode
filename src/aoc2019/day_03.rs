use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;
use std::str::FromStr;

type Int = i32;

enum ParseError {
    UnknownDirection,
    InvalidValue,
}

#[derive(Debug, Clone, Copy)]
struct WireNode {
    value: Int,
    direction: Direction,
}

impl FromStr for WireNode {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match &s[..1] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(ParseError::UnknownDirection),
        };
        let value = s[1..]
            .parse::<Int>()
            .map_err(|_| ParseError::InvalidValue)?;
        Ok(Self { value, direction })
    }
}

type Position = Point2d<Int>;
type WirePositions = HashMap<Position, usize>;
type Wire = Vec<WireNode>;

fn wire_positions(wire: &Wire) -> WirePositions {
    let mut positions = WirePositions::new();
    let mut position = Position::new(0, 0);
    let mut steps = 0;
    for node in wire {
        let times = node.value;
        for _ in 0..times {
            position = match node.direction {
                Direction::Up => position.up(),
                Direction::Down => position.down(),
                Direction::Left => position.left(),
                Direction::Right => position.right(),
            };
            steps += 1;
            if positions.contains_key(&position) {
                continue;
            };

            positions.insert(position, steps);
        }
    }
    positions
}

fn min_dist(first: &WirePositions, second: &WirePositions) -> Option<usize> {
    first
        .iter()
        .filter(|(k, _)| second.contains_key(k))
        .map(|(k, _)| (k.x.abs() + k.y.abs()) as usize)
        .min()
}

fn min_steps(first: &WirePositions, second: &WirePositions) -> Option<usize> {
    first
        .iter()
        .filter_map(|(k, v)| {
            let other_v = second.get(k)?;
            Some(*other_v + *v)
        })
        .min()
}

fn solve<R>(wire_1: &Wire, wire_2: &Wire, resolver: R) -> String
where
    R: Fn(&WirePositions, &WirePositions) -> Option<usize>,
{
    let first = wire_positions(wire_1);
    let second = wire_positions(wire_2);
    resolver(&first, &second)
        .map(|x| x.to_string())
        .unwrap_or("Not found".to_string())
}

pub struct AoC2019_03 {
    wires: Vec<Wire>,
}

impl AoC2019_03 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2019_03")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let parse_wire_path = |s: &str| -> Result<Wire, ParseError> {
            s.split(',').map(WireNode::from_str).collect()
        };
        let wires = lines
            .iter()
            .map(|x| x.as_ref())
            .map(parse_wire_path)
            .map(|x| x.ok().expect("Failed to parse wire1 path"))
            .collect::<Vec<Wire>>();
        Self { wires }
    }
}

impl Solution for AoC2019_03 {
    fn part_one(&self) -> String {
        solve(&self.wires[0], &self.wires[1], min_dist)
    }

    fn part_two(&self) -> String {
        solve(&self.wires[0], &self.wires[1], min_steps)
    }

    fn description(&self) -> String {
        "Day 3: Crossed Wires".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_03_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.wires.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_03_case_1() -> io::Result<()> {
        let lines = ["R8,U5,L5,D3", "U7,R6,D4,L4"];
        let puzzle = AoC2019_03::with_lines(&lines);
        assert_eq!(puzzle.part_one(), "6");
        Ok(())
    }

    #[test]
    fn aoc2019_03_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "4981");
        Ok(())
    }

    #[test]
    fn aoc2019_03_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "164012");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_03> {
        AoC2019_03::new()
    }
}
