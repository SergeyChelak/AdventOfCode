use crate::{
    aoc2019::intcode_computer::IntcodeComputer, solution::Solution, utils::PermutationIterator,
};

use std::fs::read_to_string;
use std::io;

use super::intcode_computer::{self, ExecutionStatus, Int};

pub struct AoC2019_07 {
    input: intcode_computer::Memory,
}

impl AoC2019_07 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2019_07")?;
        Ok(Self::with_str(&input))
    }

    fn with_str(input: &str) -> Self {
        let input = intcode_computer::parse_program(input);
        Self { input }
    }

    fn thruster_output(&self, phases: &[Int]) -> Int {
        let amplifier_output = |input: Int, phase: Int| -> Option<Int> {
            let mut computer = IntcodeComputer::with_memory(&self.input);
            computer.push_input(phase);
            computer.push_input(input);
            computer.run();
            computer.pop_output()
        };
        let mut value = 0;
        for phase in phases {
            value = amplifier_output(value, *phase).expect("Empty output in 'thruster_output'");
        }
        value
    }

    fn thruster_feedback_output(&self, phases: &[Int]) -> Int {
        let mut computers = phases
            .iter()
            .map(|phase| {
                let mut computer = IntcodeComputer::with_memory(&self.input);
                computer.push_input(*phase);
                computer
            })
            .collect::<Vec<_>>();

        let mut value = 0;
        loop {
            let mut halted = false;
            for (i, _) in phases.iter().enumerate() {
                let comp = computers.get_mut(i).expect("Computer not exists");
                comp.push_input(value);
                let status = comp.run();
                match status {
                    ExecutionStatus::Halted => {
                        halted = true;
                    }
                    ExecutionStatus::WrongInstruction { code, pc } => {
                        panic!("Comp #{i} issued wrong instruction {} at {}", code, pc);
                    }
                    ExecutionStatus::WaitForInput => {}
                }
                value = comp
                    .pop_output()
                    .expect("Empty output in 'thruster_feedback_output'");
            }
            if halted {
                break;
            }
        }
        value
    }
}

impl Solution for AoC2019_07 {
    fn part_one(&self) -> String {
        PermutationIterator::from_array(&[0, 1, 2, 3, 4])
            .map(|arr| self.thruster_output(&arr))
            .max()
            .expect("Not found")
            .to_string()
    }

    fn part_two(&self) -> String {
        PermutationIterator::from_array(&[5, 6, 7, 8, 9])
            .map(|arr| self.thruster_feedback_output(&arr))
            .max()
            .expect("Not found")
            .to_string()
    }

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
        let out = puzzle.thruster_output(&[4, 3, 2, 1, 0]);
        assert_eq!(out, 43210)
    }

    #[test]
    fn aoc2019_07_output_check_2() {
        let program = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let puzzle = AoC2019_07::with_str(program);
        let out = puzzle.thruster_output(&[0, 1, 2, 3, 4]);
        assert_eq!(out, 54321)
    }

    #[test]
    fn aoc2019_07_output_check_3() {
        let program = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let puzzle = AoC2019_07::with_str(program);
        let out = puzzle.thruster_output(&[1, 0, 4, 3, 2]);
        assert_eq!(out, 65210)
    }

    #[test]
    fn aoc2019_07_feedback_output_check_1() {
        let program =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let puzzle = AoC2019_07::with_str(program);
        let out = puzzle.thruster_feedback_output(&[9, 8, 7, 6, 5]);
        assert_eq!(out, 139629729)
    }

    #[test]
    fn aoc2019_07_feedback_output_check_2() {
        let program = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        let puzzle = AoC2019_07::with_str(program);
        let out = puzzle.thruster_feedback_output(&[9, 7, 8, 5, 6]);
        assert_eq!(out, 18216)
    }

    #[test]
    fn aoc2019_07_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "844468");
        Ok(())
    }

    #[test]
    fn aoc2019_07_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "4215746");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_07> {
        AoC2019_07::new()
    }
}
