use crate::solution::Solution;
use crate::utils::*;

use std::io;

enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Command {
    fn parse(s: &str) -> Self {
        if s.starts_with("turn on") {
            Self::TurnOn
        } else if s.starts_with("turn off") {
            Self::TurnOff
        } else if s.starts_with("toggle") {
            Self::Toggle
        } else {
            panic!("Unexpected command {s}")
        }
    }
}

struct Coordinate(usize, usize);

impl Coordinate {
    fn parse(s: &str) -> Self {
        let items: Vec<&str> = s.split(',').collect();
        let x = items[0]
            .parse::<usize>()
            .expect("Wrong x value, expected usize");
        let y = items[1]
            .parse::<usize>()
            .expect("Wrong y value, expected usize");
        Self(x, y)
    }
}

struct Instruction {
    command: Command,
    from: Coordinate,
    to: Coordinate,
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let tokens: Vec<&str> = s.split(' ').collect();
        let from_str = tokens[tokens.len() - 3];
        let to_str = tokens[tokens.len() - 1];
        Self {
            command: Command::parse(s),
            from: Coordinate::parse(from_str),
            to: Coordinate::parse(to_str),
        }
    }
}

type Matrix = Vec<Vec<i32>>;

pub struct AoC2015_06 {
    input: Vec<Instruction>,
}

impl AoC2015_06 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: Self::load_input()?,
        })
    }

    fn load_input() -> io::Result<Vec<Instruction>> {
        Ok(read_file_as_lines("input/aoc2015_06")?
            .iter()
            .map(|line| Instruction::from_str(line))
            .collect())
    }

    fn create_matrix() -> Matrix {
        let dim = 1000usize;
        vec![vec![0; dim]; dim]
    }

    fn get_lit_count(matrix: &Matrix) -> usize {
        matrix
            .iter()
            .map(|v| {
                v.iter()
                    .map(|u| if *u > 0 { *u } else { 0 } as usize)
                    .sum::<usize>()
            })
            .sum()
    }
}

#[allow(clippy::needless_range_loop)]
impl Solution for AoC2015_06 {
    fn part_one(&self) -> String {
        let mut matrix = Self::create_matrix();
        for cmd in &self.input {
            let from = &cmd.from;
            let to = &cmd.to;
            for row in from.0..=to.0 {
                for col in from.1..=to.1 {
                    matrix[row][col] = match &cmd.command {
                        Command::TurnOn => 1,
                        Command::TurnOff => 0,
                        Command::Toggle => 1 - matrix[row][col],
                    };
                }
            }
        }
        Self::get_lit_count(&matrix).to_string()
    }

    fn part_two(&self) -> String {
        let mut matrix = Self::create_matrix();
        for cmd in &self.input {
            let from = &cmd.from;
            let to = &cmd.to;
            for row in from.0..=to.0 {
                for col in from.1..=to.1 {
                    matrix[row][col] += match &cmd.command {
                        Command::TurnOn => 1,
                        Command::TurnOff => -1,
                        Command::Toggle => 2,
                    };
                    matrix[row][col] = matrix[row][col].max(0);
                }
            }
        }
        Self::get_lit_count(&matrix).to_string()
    }

    fn description(&self) -> String {
        "AoC 2015/Day 6: Probably a Fire Hazard".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_06_input_load_test() -> io::Result<()> {
        assert_eq!(AoC2015_06::new()?.input.len(), 300);
        Ok(())
    }

    #[test]
    fn aoc2015_06_correctness() -> io::Result<()> {
        let sol = AoC2015_06::new()?;
        assert_eq!(sol.part_one(), "377891");
        assert_eq!(sol.part_two(), "14110788");
        Ok(())
    }
}
