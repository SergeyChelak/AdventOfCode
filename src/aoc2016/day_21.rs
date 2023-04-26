use crate::solution::Solution;
use crate::utils::*;

use std::io;

enum Operation {
    SwapPosition(usize, usize), // swap position X with position Y
    SwapLetter(char, char),     // swap letter X with letter Y
    RotateLeft(usize),          // rotate left X steps
    RotateRight(usize),         // rotate right X steps
    RotateLetterPosition(char), // rotate based on position of letter X
    ReversePosition(char, char),// reverse positions X through Y
    MovePosition(usize, usize), // move position X to position Y
}

impl Operation {
    fn from_str(s: &str) -> Self {
        let tokens = s.split(' ').collect::<Vec<&str>>();
        match (tokens[0], tokens[1]) {
            ("swap", "position") => Self::parse_swap_position(&tokens),
            ("swap", "letter") => Self::parse_swap_letter(&tokens),
            ("rotate", "left") => Self::parse_rotate_left(&tokens),
            ("rotate", "right") => Self::parse_rotate_right(&tokens),
            ("rotate", "based") => Self::parse_rotate_letter_position(&tokens),
            ("reverse", _) => Self::parse_reverse_position(&tokens),
            ("move", _) => Self::parse_move_position(&tokens),
            _ => panic!("Unexpected operation string {} {}...", tokens[0], tokens[1]),
        }
    }

    fn parse_swap_position(tokens: &[&str]) -> Self {
        let x = tokens[2].parse::<usize>()
            .expect("Swap position: 1st index isn't integer");
        let y = tokens[5].parse::<usize>()
            .expect("Swap position: 2nd index isn't integer");
        Self::SwapPosition(x, y)
    }

    fn parse_swap_letter(tokens: &[&str]) -> Self {
        let x = tokens[2].parse::<char>()
            .expect("Swap letter: 1st index isn't char");
        let y = tokens[5].parse::<char>()
            .expect("Swap letter: 2nd index isn't char");
        Self::SwapLetter(x, y)
    }

    fn parse_rotate_left(tokens: &[&str]) -> Self {
        let x = tokens[2].parse::<usize>()
            .expect("Rotate left: 1st index isn't integer");
        Self::RotateLeft(x)
    }

    fn parse_rotate_right(tokens: &[&str]) -> Self {
        let x = tokens[2].parse::<usize>()
            .expect("Rotate right: 1st index isn't integer");
        Self::RotateRight(x)
    }

    fn parse_rotate_letter_position(tokens: &[&str]) -> Self {
        let x = tokens[6].parse::<char>()
            .expect("Rotate based on position: 1st index isn't char");
        Self::RotateLetterPosition(x)
    }

    fn parse_reverse_position(tokens: &[&str]) -> Self {
        let x = tokens[2].parse::<char>()
            .expect("Reverse position: 1st index isn't char");
        let y = tokens[4].parse::<char>()
            .expect("Reverse position: 2nd index isn't char");
        Self::ReversePosition(x, y)
    }

    fn parse_move_position(tokens: &[&str]) -> Self {
        let x = tokens[2].parse::<usize>()
            .expect("Move position: 1st index isn't integer");
        let y = tokens[5].parse::<usize>()
            .expect("Move position: 2nd index isn't integer");
        Self::MovePosition(x, y)
    }
}

pub struct AoC2016_21 {
    operations: Vec<Operation>
}

impl AoC2016_21 {
    pub fn new() -> io::Result<Self> {
        let operations = read_file_as_lines("input/aoc2016_21")?
            .iter()
            .map(|s| Operation::from_str(s))
            .collect::<Vec<Operation>>();
        Ok(Self {
            operations
        })
    }
}

impl Solution for AoC2016_21 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2016/Day 21: Scrambled Letters and Hash".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_21_input_load_test() -> io::Result<()> {
        let sol = AoC2016_21::new()?;
        assert_eq!(sol.operations.len(), 100);
        Ok(())
    }

    #[test]
    fn aoc2016_21_correctness() -> io::Result<()> {
        let sol = AoC2016_21::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}