use crate::solution::Solution;

use std::fs::read_to_string;
use std::io;

use super::intcode_computer::*;

pub struct AoC2019_19 {
    program: Memory,
}

impl AoC2019_19 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2019_19")?;
        Ok(Self::with_str(&input))
    }

    fn with_str(input: &str) -> Self {
        Self {
            program: parse_program(input),
        }
    }
}

impl Solution for AoC2019_19 {
    fn part_one(&self) -> String {
        let mut count = 0;
        for y in 0..50 {
            for x in 0..50 {
                let mut computer = IntcodeComputer::with_size(1024);
                computer.load_program(&self.program);
                computer.push_input(x);
                computer.push_input(y);
                let status = computer.run();
                assert_eq!(status, ExecutionStatus::Halted);
                count += (computer.pop_output().unwrap() > 0) as usize;
            }
        }
        count.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 19: Tractor Beam".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_19_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        Ok(())
    }

    #[test]
    fn aoc2019_19_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "");
        Ok(())
    }

    #[test]
    fn aoc2019_19_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_19> {
        AoC2019_19::new()
    }
}
