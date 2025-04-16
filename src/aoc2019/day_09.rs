use crate::solution::Solution;

use std::fs::read_to_string;
use std::io;

use super::intcode_computer::*;

pub struct AoC2019_09 {
    input: Memory,
}

impl AoC2019_09 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2019_09")?;
        Ok(Self::with_str(&input))
    }

    fn with_str(input: &str) -> Self {
        Self {
            input: parse_program(input),
        }
    }
}

impl Solution for AoC2019_09 {
    fn part_one(&self) -> String {
        let mut computer = IntcodeComputer::with_size(10 * 1024);
        computer.load_program(&self.input);
        computer.push_input(1);
        let status = computer.run();
        assert!(
            matches!(status, ExecutionStatus::Halted),
            "Unexpected execution status: {:?}",
            status
        );
        computer.pop_output().expect("Output is empty").to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 9: Sensor Boost".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_09_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_09_execution_1_test() {
        let program = [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut comp = IntcodeComputer::with_size(10 * 1024);
        comp.load_program(&program);
        let status = comp.run();
        assert!(matches!(status, ExecutionStatus::Halted));
        let output = comp.output();
        assert_eq!(output, program);
    }

    #[test]
    fn aoc2019_09_execution_2_test() {
        let program = [1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut comp = IntcodeComputer::with_size(10 * 1024);
        comp.load_program(&program);
        let status = comp.run();
        assert!(matches!(status, ExecutionStatus::Halted));
        let output = comp.pop_output().expect("Empty output").to_string();
        assert_eq!(output.len(), 16);
    }

    #[test]
    fn aoc2019_09_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "3598076521");
        Ok(())
    }

    #[test]
    fn aoc2019_09_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_09> {
        AoC2019_09::new()
    }
}
