use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = isize;

struct Command {
    cmd_type: CommandType,
    amount: Int,
}

enum CommandType {
    Forward,
    Down,
    Up,
}

pub struct AoC2021_02 {
    input: Vec<Command>,
}

impl AoC2021_02 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2021_02")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let Ok(input) = lines
            .iter()
            .map(|s| s.as_ref())
            .map(Command::try_from)
            .collect::<Result<Vec<_>, _>>()
        else {
            panic!("Failed to parse input");
        };
        Self { input }
    }
}

impl Solution for AoC2021_02 {
    fn part_one(&self) -> String {
        let mut depth: Int = 0;
        let mut horizontal: Int = 0;
        self.input.iter().for_each(|cmd| match cmd.cmd_type {
            CommandType::Forward => horizontal += cmd.amount,
            CommandType::Down => depth += cmd.amount,
            CommandType::Up => depth -= cmd.amount,
        });
        (depth * horizontal).to_string()
    }

    fn part_two(&self) -> String {
        let mut aim: Int = 0;
        let mut horizontal: Int = 0;
        let mut depth: Int = 0;
        self.input.iter().for_each(|cmd| match cmd.cmd_type {
            CommandType::Forward => {
                horizontal += cmd.amount;
                depth += aim * cmd.amount;
            }
            CommandType::Down => aim += cmd.amount,
            CommandType::Up => aim -= cmd.amount,
        });
        (depth * horizontal).to_string()
    }

    fn description(&self) -> String {
        "Day 2: Dive!".to_string()
    }
}

impl TryFrom<&str> for Command {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let Some((cmd_type, amount)) = value.split_once(' ') else {
            return Err("Invalid input format".to_string());
        };
        let cmd_type = CommandType::try_from(cmd_type)?;
        let amount = amount
            .parse::<Int>()
            .map_err(|_| format!("Invalid amount value {amount}"))?;
        Ok(Self { cmd_type, amount })
    }
}

impl TryFrom<&str> for CommandType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "forward" => Ok(Self::Forward),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err(format!("Unknown direction {value}")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_02_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2021_02_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "1250395");
        Ok(())
    }

    #[test]
    fn aoc2021_02_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "1451210346");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2021_02> {
        AoC2021_02::new()
    }
}
