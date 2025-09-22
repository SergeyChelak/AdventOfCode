use crate::solution::Solution;
use crate::utils::not_found;

use std::fs::read_to_string;
use std::io;

use super::intcode_computer::*;

pub struct AoC2019_21 {
    program: Memory,
}

impl AoC2019_21 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2019_21")?;
        Ok(Self::with_str(&input))
    }

    fn with_str(input: &str) -> Self {
        Self {
            program: parse_program(input),
        }
    }

    fn run_code<T: AsRef<str>>(&self, code: &[T], show_log: bool) -> Option<Int> {
        let mut computer = IntcodeComputer::with_memory(&self.program);
        let code = code
            .iter()
            .map(|x| x.as_ref())
            .collect::<Vec<_>>()
            .join("\n");
        computer.push_input_str(&code);
        computer.push_input(10);
        let status = computer.run();
        assert!(matches!(status, ExecutionStatus::Halted));
        let output = computer.sink_outputs();

        let Some(value) = output
            .last()
            .and_then(|&x| if x > 255 { Some(x) } else { None })
        else {
            if show_log {
                let text = output
                    .into_iter()
                    .map(|val| val as u8 as char)
                    .collect::<String>();
                println!("{text}");
            }
            return None;
        };
        Some(value)
    }
}

impl Solution for AoC2019_21 {
    fn part_one(&self) -> String {
        #[rustfmt::skip]
        let Some(result) = self.run_code(
            &[
                "NOT A J",
                "NOT B T",
                "OR T J",
                "NOT C T",
                "OR T J",
                "AND D J",
                "WALK",
            ],
            true,
        ) else {
            return "Not found".to_string();
        };
        result.to_string()
    }

    fn part_two(&self) -> String {
        #[rustfmt::skip]
        let Some(result) = self.run_code(
            &[
                "NOT A J",
                "NOT B T",
                "OR T J",
                "NOT C T",
                "OR T J",
                "AND D J",
                "AND E T",
                "OR H T",
                "AND T J",
                "RUN",
            ],
            true,
        ) else {
            return not_found();
        };
        result.to_string()
    }

    fn description(&self) -> String {
        "Day 21: Springdroid Adventure".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_21_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.program.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_21_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "19348840");
        Ok(())
    }

    #[test]
    fn aoc2019_21_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "1141857182");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_21> {
        AoC2019_21::new()
    }
}
