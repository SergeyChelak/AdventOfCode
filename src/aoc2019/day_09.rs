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
        let mut computer = IntcodeComputer::with_memory(self.input.clone());
        computer.push_input(1);
        computer.run();

        computer.output().expect("Empty output").to_string()
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
    fn aoc2019_09_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "");
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
