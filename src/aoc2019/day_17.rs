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
        let mut computer = IntcodeComputer::with_size(10 * 1024);
        computer.load_program(&self.input);
        let status = computer.run();
        assert!(matches!(status, ExecutionStatus::Halted));
        let output = computer.sink_outputs();
        let map = convert_map(&output);
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
        let mut computer = IntcodeComputer::with_size(10 * 1024);
        let mut program = self.input.clone();
        // build map
        computer.load_program(&program);
        let status = computer.run();
        assert!(matches!(status, ExecutionStatus::Halted));
        let output = computer.sink_outputs();
        // dump(&output);
        let map = convert_map(&output);
        let Some(pos) = bot_position(&map) else {
            panic!("Bot not found");
        };
        println!("{pos:?}");
        let Some(direction) = map.get(&pos).map(|x| Direction::from(*x)) else {
            panic!("Failed to determine direction");
        };
        // let path = build_path(&map, pos, direction);
        // format_path(&path);
        //
        program[0] = 2;
        computer.load_program(&program);

        let push_input = |comp: &mut IntcodeComputer, s: &str| {
            s.chars()
                .map(|ch| ch as u8 as Int)
                .for_each(|x| comp.push_input(x));
            comp.push_input(10);
        };

        push_input(&mut computer, "A,B,B,C,B,C,B,C,A,A");
        push_input(&mut computer, "L,6,R,8,L,4,R,8,L,12");
        push_input(&mut computer, "L,12,R,10,L,4");
        push_input(&mut computer, "L,12,L,6,L,4,L,4");
        push_input(&mut computer, "n");

        let status = computer.run();
        println!("Status: {status:?}");
        computer
            .pop_output()
            .map(|x| x.to_string())
            .unwrap_or("Not found".to_string())
    }

    fn description(&self) -> String {
        "Day 17: Set and Forget".to_string()
    }
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
        println!(
            "P: ({}, {})\tPE: {:?}\tDir: {:?}\tDist: {}\t SS: {}",
            node.position.x,
            node.position.y,
            node.path_element,
            node.direction,
            node.distance,
            stack.len()
        );

        visited.insert(node.position);
        if visited.len() == total_nodes {
            println!("Path found");
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
        if adjacent.get(&node.direction.turn_left()).is_some() {
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
        if adjacent.get(&node.direction.turn_right()).is_some() {
            let next = StackNode {
                path_element: PathElement::Right,
                distance: 0,
                direction: node.direction.turn_right(),
                position: node.position,
            };
            stack.push(next);
            continue;
        }

        _ = stack.pop();
    }

    stack
}

fn format_path(path: &[StackNode]) {
    let arr = path
        .iter()
        .map(|x| match x.path_element {
            PathElement::Direct => x.distance.to_string(),
            PathElement::Left => "L".to_string(),
            PathElement::Right => "R".to_string(),
        })
        .collect::<Vec<_>>();
    let s = arr.join(",");
    println!("{s}");
}

fn dump(output: &[Int]) {
    output
        .iter()
        .map(|x| *x as u8 as char)
        .for_each(|x| print!("{x}"));
    println!()
}

fn bot_position(map: &ScaffoldMap) -> Option<Point> {
    map.iter()
        .find(|(_, v)| [b'^', b'v', b'<', b'>'].contains(&(**v as u8)))
        .map(|(k, _)| *k)
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
