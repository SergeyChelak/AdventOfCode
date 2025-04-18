use crate::solution::Solution;
use crate::utils::{Direction, Point2d};

use core::panic;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;

use super::intcode_computer::*;

type Position = Point2d<i32>;

const COLOR_BLACK: Int = 0;

pub struct AoC2019_11 {
    input: Memory,
}

impl AoC2019_11 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2019_11")?;
        let input = parse_program(&input);
        Ok(Self { input })
    }
}

impl Solution for AoC2019_11 {
    fn part_one(&self) -> String {
        let mut computer = IntcodeComputer::with_size(10 * 1024);
        computer.load_program(&self.input);

        let mut panels: HashMap<Position, Int> = HashMap::new();
        let mut position = Position::new(0, 0);
        let mut direction = Direction::Up;
        loop {
            let inp = panels.get(&position).unwrap_or(&COLOR_BLACK);
            computer.push_input(*inp);
            let status = computer.run();
            if !matches!(status, ExecutionStatus::WaitForInput) {
                break;
            }
            let movement = computer.pop_output().expect("Bad output for movement");
            let color = computer.pop_output().expect("Bad output for color");
            panels.insert(position, color);
            direction = match movement {
                0 => direction.turn_left(),
                1 => direction.turn_right(),
                _ => panic!("Unexpected movement value {movement}"),
            };
            position = match direction {
                Direction::Up => position.up(),
                Direction::Down => position.down(),
                Direction::Left => position.left(),
                Direction::Right => position.right(),
            }
        }
        panels.len().to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 11: Space Police".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_11_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_11_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "2255");
        Ok(())
    }

    #[test]
    fn aoc2019_11_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_11> {
        AoC2019_11::new()
    }
}
