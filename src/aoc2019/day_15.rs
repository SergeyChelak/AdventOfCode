use crate::solution::Solution;
use crate::utils::{Direction, Point2d};

use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io;

use super::intcode_computer::*;

pub struct AoC2019_15 {
    input: Memory,
}

impl AoC2019_15 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2019_15")?;
        Ok(Self::with_str(&input))
    }

    fn with_str(input: &str) -> Self {
        Self {
            input: parse_program(input),
        }
    }
}

impl Solution for AoC2019_15 {
    fn part_one(&self) -> String {
        let data = traverse_environment(&self.input);
        let Some(distance) = data.oxygen_position.and_then(|p| data.distance_map.get(&p)) else {
            return "Not found".to_string();
        };
        distance.to_string()
    }

    fn part_two(&self) -> String {
        let mut data = traverse_environment(&self.input);
        let Some(start) = data.oxygen_position else {
            return "Not found".to_string();
        };
        fill(&mut data.map, start).to_string()
    }

    fn description(&self) -> String {
        "Day 15: Oxygen System".to_string()
    }
}

type Position = Point2d<isize>;

impl Position {
    fn move_by(&self, direction: &Direction) -> Self {
        let x = self.x;
        let y = self.y;
        match direction {
            Direction::Up => Position::new(x, y - 1),
            Direction::Down => Position::new(x, y + 1),
            Direction::Left => Position::new(x - 1, y),
            Direction::Right => Position::new(x + 1, y),
        }
    }
}

const DIRECTION_NORTH: Int = 1;
const DIRECTION_SOUTH: Int = 2;
const DIRECTION_WEST: Int = 3;
const DIRECTION_EAST: Int = 4;

impl From<Direction> for Int {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => DIRECTION_NORTH,
            Direction::Down => DIRECTION_SOUTH,
            Direction::Left => DIRECTION_WEST,
            Direction::Right => DIRECTION_EAST,
        }
    }
}

const TILE_UNDEFINED: Int = -1;
const TILE_WALL: Int = 0;
const TILE_FREE: Int = 1;
const TILE_OXYGEN: Int = 2;

type TileMap = HashMap<Position, Int>;
type DistanceMap = HashMap<Position, Int>;

struct Environment {
    map: TileMap,
    distance_map: DistanceMap,
    oxygen_position: Option<Position>,
}

fn traverse_environment(input: &[Int]) -> Environment {
    let mut computer = IntcodeComputer::with_size(10 * 1024);
    computer.load_program(input);

    let mut position = Position::new(0, 0);
    let mut path: Vec<Direction> = Vec::new();

    let mut oxygen_position: Option<Position> = None;

    let mut map = TileMap::new();
    let mut distance_map: HashMap<Position, Int> = HashMap::new();
    map.insert(position, TILE_FREE);

    loop {
        let mut is_backtracking = false;
        // has adjacent?
        if let Some(dir) = Direction::all().iter().find(|dir| {
            let p = position.move_by(dir);
            let distance = distance_map.get(&p).unwrap_or(&Int::MAX);
            *distance > path.len() as Int + 1
        }) {
            path.push(*dir);
        } else if let Some(dir) = path.pop() {
            // backtrack
            path.push(dir.reverse());
            is_backtracking = true;
        }
        let Some(dir) = path.pop() else {
            break;
        };
        computer.push_input(dir.into());
        let status = computer.run();
        assert!(matches!(status, ExecutionStatus::WaitForInput));
        let Some(output) = computer.pop_output() else {
            panic!("Empty output")
        };

        let next = position.move_by(&dir);
        map.insert(next, output);
        if output != TILE_WALL {
            position = next;
            if !is_backtracking {
                path.push(dir);
            }
        }
        distance_map.insert(next, path.len() as Int);

        if output == TILE_OXYGEN {
            oxygen_position = Some(next)
        }
    }
    Environment {
        map,
        distance_map,
        oxygen_position,
    }
}

fn fill(map: &mut TileMap, start: Position) -> usize {
    let mut positions: HashSet<Position> = HashSet::new();
    positions.insert(start);
    let mut steps = 0;

    while !positions.is_empty() {
        let mut next = HashSet::new();
        for p in positions.iter() {
            Direction::all()
                .iter()
                .map(|dir| p.move_by(dir))
                .filter(|p| {
                    let tile = map.get(p).unwrap_or(&TILE_UNDEFINED);
                    *tile == TILE_FREE
                })
                .for_each(|p| {
                    next.insert(p);
                });
        }
        next.iter().for_each(|p| {
            map.insert(*p, TILE_OXYGEN);
        });
        positions = next;
        if !positions.is_empty() {
            steps += 1;
        }
    }
    steps
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_15_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_15_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "222");
        Ok(())
    }

    #[test]
    fn aoc2019_15_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "394");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_15> {
        AoC2019_15::new()
    }
}
