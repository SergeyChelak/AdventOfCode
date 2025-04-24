use crate::{
    aoc2019::intcode_computer::{ExecutionStatus, IntcodeComputer},
    solution::Solution,
    utils::{Direction, Point2d},
};

use std::{collections::HashMap, fs::read_to_string};
use std::{collections::HashSet, io};

use super::intcode_computer::{parse_program, Int, Memory};

pub struct AoC2019_17 {
    input: Memory,
}

impl AoC2019_17 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2019_17")?;
        Ok(Self::with_str(&input))
    }

    fn with_str(s: &str) -> Self {
        Self {
            input: parse_program(s),
        }
    }
}

impl Solution for AoC2019_17 {
    fn part_one(&self) -> String {
        let map = build_map(&self.input);
        map.keys()
            .filter(|p| {
                [
                    Point::new(p.x, p.y - 1),
                    Point::new(p.x, p.y + 1),
                    Point::new(p.x - 1, p.y),
                    Point::new(p.x + 1, p.y),
                ]
                .iter()
                .map(|adj| map.contains_key(adj) as u8)
                .sum::<u8>()
                    > 2
            })
            .map(|p| p.x * p.y)
            .sum::<Int>()
            .to_string()
    }

    fn part_two(&self) -> String {
        let map = build_map(&self.input);
        let Some(pos) = bot_position(&map) else {
            panic!("Bot not found");
        };
        let Some(direction) = map.get(&pos).map(|x| Direction::from(*x)) else {
            panic!("Failed to determine direction");
        };
        let path = {
            let stack = build_path(&map, pos, direction);
            extract_path(&stack)
        };
        //
        let input = build_movement_instructions(&path);
        let mut computer = IntcodeComputer::with_size(10 * 1024);
        let mut program = self.input.clone();
        program[0] = 2;
        computer.load_program(&program);
        computer.push_input_str(&input);

        let status = computer.run();
        assert!(matches!(status, ExecutionStatus::Halted));
        computer
            .pop_output()
            .map(|x| x.to_string())
            .unwrap_or("Not found".to_string())
    }

    fn description(&self) -> String {
        "Day 17: Set and Forget".to_string()
    }
}

fn build_movement_instructions(input: &[String]) -> String {
    let max_length = 20;
    let path = input.join(",");

    let mut dictionary = HashSet::<String>::new();

    for from in 0..input.len() - 1 {
        for len in 1..input.len() - from {
            let pattern = input[from..from + len].join(",");
            if dictionary.contains(&pattern) {
                continue;
            }
            if !(3..=max_length).contains(&pattern.len()) {
                continue;
            }
            let count = path.matches(&pattern).count();
            if count > 1 {
                dictionary.insert(pattern);
            }
        }
    }

    let patterns = dictionary.into_iter().collect::<Vec<_>>();
    let allowed_set = [',', 'A', 'B', 'C'].into_iter().collect::<HashSet<_>>();
    for (a, a_pattern) in patterns.iter().enumerate() {
        let compressed = path.replace(a_pattern, "A");
        for (b, b_pattern) in patterns.iter().enumerate().skip(a) {
            let compressed = compressed.replace(b_pattern, "B");
            for c_pattern in patterns.iter().skip(b) {
                let compressed = compressed.replace(c_pattern, "C");
                if compressed.len() > max_length {
                    continue;
                }
                let is_valid = compressed.chars().all(|ch| allowed_set.contains(&ch));
                if is_valid {
                    return [
                        compressed,
                        a_pattern.clone(),
                        b_pattern.clone(),
                        c_pattern.clone(),
                        "n\n".to_string(),
                    ]
                    .join("\n");
                }
            }
        }
    }

    unreachable!()
}

fn extract_path(stack: &[StackNode]) -> Vec<String> {
    stack
        .iter()
        .skip(1)
        .map(|x| match x.path_element {
            PathElement::Direct => x.distance.to_string(),
            PathElement::Left => "L".to_string(),
            PathElement::Right => "R".to_string(),
        })
        .collect::<Vec<_>>()
}

type Point = Point2d<Int>;
type ScaffoldMap = HashMap<Point, Int>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PathElement {
    Left,
    Right,
    Direct,
}
#[derive(Debug, Clone)]
struct StackNode {
    path_element: PathElement,
    distance: usize,
    direction: Direction,
    position: Point,
}

fn build_path(
    map: &ScaffoldMap,
    start_position: Point,
    initial_direction: Direction,
) -> Vec<StackNode> {
    let total_nodes = map.keys().len();
    let mut visited = HashSet::<Point>::new();

    let mut stack = Vec::new();
    stack.push(StackNode {
        path_element: PathElement::Direct,
        distance: 0,
        direction: initial_direction,
        position: start_position,
    });

    while let Some(node) = stack.last() {
        visited.insert(node.position);
        if visited.len() == total_nodes {
            break;
        }

        // get adjacent
        let adjacent = Direction::all()
            .iter()
            .map(|dir| (*dir, node.position.move_by(*dir)))
            .filter(|(_, p)| map.get(p).is_some())
            .collect::<HashMap<_, _>>();

        // can move in current direction
        if let Some(point) = adjacent.get(&node.direction) {
            let next = StackNode {
                path_element: PathElement::Direct,
                distance: 1 + node.distance,
                direction: node.direction,
                position: *point,
            };
            if next.path_element == node.path_element {
                _ = stack.pop();
            }
            stack.push(next);
            continue;
        }

        // can turn left
        if adjacent.contains_key(&node.direction.turn_left()) {
            let next = StackNode {
                path_element: PathElement::Left,
                distance: 0,
                direction: node.direction.turn_left(),
                position: node.position,
            };
            stack.push(next);
            continue;
        }

        // can turn right
        if adjacent.contains_key(&node.direction.turn_right()) {
            let next = StackNode {
                path_element: PathElement::Right,
                distance: 0,
                direction: node.direction.turn_right(),
                position: node.position,
            };
            stack.push(next);
            continue;
        }
        unreachable!()
    }

    stack
}

fn bot_position(map: &ScaffoldMap) -> Option<Point> {
    map.iter()
        .find(|(_, v)| [b'^', b'v', b'<', b'>'].contains(&(**v as u8)))
        .map(|(k, _)| *k)
}

fn build_map(input: &[Int]) -> ScaffoldMap {
    let mut computer = IntcodeComputer::with_size(10 * 1024);
    computer.load_program(input);
    let status = computer.run();
    assert!(matches!(status, ExecutionStatus::Halted));
    let output = computer.sink_outputs();
    convert_map(&output)
}

fn convert_map(output: &[Int]) -> ScaffoldMap {
    let mut result = ScaffoldMap::new();
    let mut point = Point::new(0, 0);
    for val in output {
        match *val {
            46 => {}
            10 => {
                point = Point::new(0, point.y + 1);
                continue;
            }
            _ => {
                result.insert(point, *val);
            }
        }
        point = Point::new(point.x + 1, point.y);
    }
    result
}

impl Point {
    fn move_by(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Point::new(self.x, self.y - 1),
            Direction::Down => Point::new(self.x, self.y + 1),
            Direction::Left => Point::new(self.x - 1, self.y),
            Direction::Right => Point::new(self.x + 1, self.y),
        }
    }
}

impl From<Int> for Direction {
    fn from(value: Int) -> Self {
        match value as u8 {
            b'^' => Self::Up,
            b'v' => Self::Down,
            b'<' => Self::Left,
            b'>' => Self::Right,
            _ => panic!("Unexpected direction value {value}"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_17_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_17_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "4220");
        Ok(())
    }

    #[test]
    fn aoc2019_17_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "809736");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_17> {
        AoC2019_17::new()
    }
}
