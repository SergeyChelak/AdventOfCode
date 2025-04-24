use crate::{
    aoc2019::intcode_computer::{ExecutionStatus, IntcodeComputer},
    solution::Solution,
    utils::Point2d,
};

use std::io;
use std::{collections::HashMap, fs::read_to_string};

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

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 17: Set and Forget".to_string()
    }
}

type Point = Point2d<Int>;

fn convert_map(output: &[Int]) -> HashMap<Point, Int> {
    let mut result = HashMap::new();
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
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_17> {
        AoC2019_17::new()
    }
}
