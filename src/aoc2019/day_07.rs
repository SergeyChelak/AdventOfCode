use crate::solution::Solution;

use std::fs::read_to_string;
use std::io;

use super::intcode_computer;

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
}

impl Solution for AoC2019_07 {
    // fn part_one(&self) -> String {
    // }

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
