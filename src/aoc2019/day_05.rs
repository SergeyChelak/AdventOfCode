use crate::solution::Solution;

use std::fs::read_to_string;
use std::io;

use super::intcode_computer::{self, IntcodeComputer};

pub struct AoC2019_05 {
    input: intcode_computer::Memory,
}

impl AoC2019_05 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2019_05")?;
        Ok(Self::with_str(&input))
    }

    fn with_str(input: &str) -> Self {
        let input = intcode_computer::parse_program(input);
        Self { input }
    }
}

impl Solution for AoC2019_05 {
    fn part_one(&self) -> String {
        let mut computer = IntcodeComputer::new(self.input.clone(), 1);
        computer.run();
        computer.output().to_string()
    }

    fn part_two(&self) -> String {
        let mut computer = IntcodeComputer::new(self.input.clone(), 5);
        computer.run();
        computer.output().to_string()
    }

    fn description(&self) -> String {
        "Day 5: Sunny with a Chance of Asteroids".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_05_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_05_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "7692125");
        Ok(())
    }

    #[test]
    fn aoc2019_05_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "14340395");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_05> {
        AoC2019_05::new()
    }
}
