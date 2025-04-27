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
        let mut computer = IntcodeComputer::with_memory(&self.program);
        let mut count = 0;
        for y in 0..50 {
            for x in 0..50 {
                computer.push_input(x);
                computer.push_input(y);
                let status = computer.run();
                assert_eq!(status, ExecutionStatus::Halted);
                count += (computer.pop_output().unwrap() > 0) as usize;
                computer.reset();
            }
        }
        count.to_string()
    }

    fn part_two(&self) -> String {
        let mut computer = IntcodeComputer::with_memory(&self.program);
        let mut row = 200;
        let mut offset = 0;
        let target_dim = 100;
        loop {
            // correct offset
            loop {
                computer.reset();
                computer.push_input(offset);
                computer.push_input(row);
                let status = computer.run();
                assert_eq!(status, ExecutionStatus::Halted);
                let val = computer.pop_output().unwrap();
                if val > 0 {
                    break;
                }
                offset += 1;
            }
            let mut found = true;
            let top = row - target_dim + 1;
            let right = offset + target_dim - 1;
            for (x, y) in [
                // check if left height is enough
                (offset, top),
                // check if bottom width is enough
                (right, row),
                // check if right height is enough
                (right, top),
            ] {
                computer.reset();
                computer.push_input(x);
                computer.push_input(y);
                let status = computer.run();
                assert_eq!(status, ExecutionStatus::Halted);
                let val = computer.pop_output().unwrap();
                if val == 0 {
                    found = false;
                    break;
                }
            }
            if !found {
                row += 1;
                continue;
            }
            break offset * 10000 + top;
        }
        .to_string()
    }

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
        assert!(!sol.program.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_19_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "162");
        Ok(())
    }

    #[test]
    fn aoc2019_19_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "13021056");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_19> {
        AoC2019_19::new()
    }
}
