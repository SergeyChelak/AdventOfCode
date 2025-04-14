use crate::{aoc2019::intcode_computer::IntcodeComputer, solution::Solution};

use std::fs::read_to_string;
use std::io;

use super::intcode_computer::{self, Int};

pub struct AoC2019_07 {
    input: intcode_computer::Memory,
}

impl AoC2019_07 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2019_05")?;
        Ok(Self::with_str(&input))
    }

    fn with_str(input: &str) -> Self {
        let input = intcode_computer::parse_program(input);
        Self { input }
    }

    fn amplifier_output(&self, input: Int, phase: Int) -> Int {
        let mut computer = IntcodeComputer::with_memory(self.input.clone());
        computer.push_input(input);
        computer.push_input(phase);
        computer.run();
        computer.output()
    }

    fn thruster_output(&self, initial_input: Int, phases: &[Int]) -> Int {
        let mut value = initial_input;
        for phase in phases {
            let output = self.amplifier_output(value, *phase);
            value = output;
        }
        value
    }
}

impl Solution for AoC2019_07 {
    fn part_one(&self) -> String {
        todo!()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 7: Amplification Circuit".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_07_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_07_output_check_1() {
        let program = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let puzzle = AoC2019_07::with_str(program);
        let out = puzzle.thruster_output(0, &[4, 3, 2, 1, 0]);
        assert_eq!(out, 43210)
    }

    #[test]
    fn aoc2019_07_output_check_2() {
        let program = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let puzzle = AoC2019_07::with_str(program);
        let out = puzzle.thruster_output(0, &[0, 1, 2, 3, 4]);
        assert_eq!(out, 54321)
    }

    #[test]
    fn aoc2019_07_output_check_3() {
        let program = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let puzzle = AoC2019_07::with_str(program);
        let out = puzzle.thruster_output(0, &[1, 0, 4, 3, 2]);
        assert_eq!(out, 65210)
    }

    #[test]
    fn aoc2019_07_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "");
        Ok(())
    }

    #[test]
    fn aoc2019_07_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_07> {
        AoC2019_07::new()
    }
}
