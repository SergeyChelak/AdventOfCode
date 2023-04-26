use crate::solution::Solution;
use crate::utils::*;

use std::io;

enum Operation {
    SwapPosition(usize, usize),    // swap position X with position Y
    SwapLetter(char, char),        // swap letter X with letter Y
    RotateLeft(usize),             // rotate left X steps
    RotateRight(usize),            // rotate right X steps
    RotateLetterPosition(char),    // rotate based on position of letter X
    ReversePosition(usize, usize), // reverse positions X through Y
    MovePosition(usize, usize),    // move position X to position Y
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
        let x = tokens[2]
            .parse::<usize>()
            .expect("Swap position: 1st index isn't integer");
        let y = tokens[5]
            .parse::<usize>()
            .expect("Swap position: 2nd index isn't integer");
        Self::SwapPosition(x, y)
    }

    fn parse_swap_letter(tokens: &[&str]) -> Self {
        let x = tokens[2]
            .parse::<char>()
            .expect("Swap letter: 1st index isn't char");
        let y = tokens[5]
            .parse::<char>()
            .expect("Swap letter: 2nd index isn't char");
        Self::SwapLetter(x, y)
    }

    fn parse_rotate_left(tokens: &[&str]) -> Self {
        let x = tokens[2]
            .parse::<usize>()
            .expect("Rotate left: 1st index isn't integer");
        Self::RotateLeft(x)
    }

    fn parse_rotate_right(tokens: &[&str]) -> Self {
        let x = tokens[2]
            .parse::<usize>()
            .expect("Rotate right: 1st index isn't integer");
        Self::RotateRight(x)
    }

    fn parse_rotate_letter_position(tokens: &[&str]) -> Self {
        let x = tokens[6]
            .parse::<char>()
            .expect("Rotate based on position: 1st index isn't char");
        Self::RotateLetterPosition(x)
    }

    fn parse_reverse_position(tokens: &[&str]) -> Self {
        let x = tokens[2]
            .parse::<usize>()
            .expect("Reverse position: 1st index isn't integer");
        let y = tokens[4]
            .parse::<usize>()
            .expect("Reverse position: 2nd index isn't integer");
        Self::ReversePosition(x, y)
    }

    fn parse_move_position(tokens: &[&str]) -> Self {
        let x = tokens[2]
            .parse::<usize>()
            .expect("Move position: 1st index isn't integer");
        let y = tokens[5]
            .parse::<usize>()
            .expect("Move position: 2nd index isn't integer");
        Self::MovePosition(x, y)
    }

    fn scramble(&self, inp: &str) -> String {
        match self {
            Self::SwapPosition(x, y) => swap_position(inp, *x, *y),
            Self::SwapLetter(x, y) => swap_letter(inp, *x, *y),
            Self::RotateLeft(x) => rotate_left(inp, *x),
            Self::RotateRight(x) => rotate_right(inp, *x),
            Self::RotateLetterPosition(x) => rotate_letter_position(inp, *x),
            Self::ReversePosition(x, y) => reverse_position(inp, *x, *y),
            Self::MovePosition(x, y) => move_position(inp, *x, *y),
        }
    }
}

fn swap_position(inp: &str, x: usize, y: usize) -> String {
    let mut chars = inp.chars().collect::<Vec<char>>();
    chars.swap(x, y);
    chars.iter().collect::<String>()
}

fn swap_letter(inp: &str, x: char, y: char) -> String {
    let chars = inp.chars().collect::<Vec<char>>();
    chars
        .iter()
        .map(|&ch| {
            if ch == x {
                y
            } else if ch == y {
                x
            } else {
                ch
            }
        })
        .collect::<String>()
}

fn rotate_left(inp: &str, x: usize) -> String {
    let mut chars = inp.chars().collect::<Vec<char>>();
    let len = chars.len();
    let step = x % len;
    let seg = &mut chars[..step];
    seg.reverse();
    let seg = &mut chars[step..];
    seg.reverse();
    chars.iter().rev().collect::<String>()
}

fn rotate_right(inp: &str, x: usize) -> String {
    let mut chars = inp.chars().collect::<Vec<char>>();
    let len = chars.len();
    let step = x % len;
    let seg = &mut chars[..len - step];
    seg.reverse();
    let seg = &mut chars[len - step..];
    seg.reverse();
    chars.iter().rev().collect::<String>()
}

fn rotate_letter_position(inp: &str, x: char) -> String {
    if let Some(mut index) = inp.find(x) {
        if index > 3 {
            index += 1;
        }
        rotate_right(inp, index + 1)
    } else {
        inp.to_string()
    }
}

fn reverse_position(inp: &str, x: usize, y: usize) -> String {
    let mut chars = inp.chars().collect::<Vec<char>>();
    let arr = &mut chars[x..=y];
    arr.reverse();
    chars.iter().collect::<String>()
}

fn move_position(inp: &str, x: usize, y: usize) -> String {
    let mut chars = inp.chars().collect::<Vec<char>>();
    let src = chars[x];
    if x > y {
        for i in (y + 1..=x).rev() {
            chars[i] = chars[i - 1];
        }
    } else {
        for i in x..y {
            chars[i] = chars[i + 1];
        }
    }
    chars[y] = src;
    chars.iter().collect::<String>()
}

pub struct AoC2016_21 {
    operations: Vec<Operation>,
}

impl AoC2016_21 {
    pub fn new() -> io::Result<Self> {
        let operations = read_file_as_lines("input/aoc2016_21")?
            .iter()
            .map(|s| Operation::from_str(s))
            .collect::<Vec<Operation>>();
        Ok(Self { operations })
    }
}

impl Solution for AoC2016_21 {
    fn part_one(&self) -> String {
        self.operations
            .iter()
            .fold("abcdefgh".to_string(), |acc, v| v.scramble(&acc))
    }

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
        assert_eq!(sol.part_one(), "bgfacdeh");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2016_21_op_swap_position() {
        assert_eq!(swap_position("abcde", 4, 0), "ebcda");
    }

    #[test]
    fn aoc2016_21_op_swap_letter() {
        assert_eq!(swap_letter("ebcda", 'd', 'b'), "edcba");
    }

    #[test]
    fn aoc2016_21_op_reverse_position() {
        assert_eq!(reverse_position("edcba", 0, 4), "abcde");
    }

    #[test]
    fn aoc2016_21_op_rotate_left() {
        assert_eq!(rotate_left("abcde", 1), "bcdea");
        assert_eq!(rotate_left("abcde", 2), "cdeab");
        assert_eq!(rotate_left("abcde", 3), "deabc");
    }

    #[test]
    fn aoc2016_21_op_rotate_right() {
        assert_eq!(rotate_right("1234567", 1), "7123456");
        assert_eq!(rotate_right("1234567", 2), "6712345");
        assert_eq!(rotate_right("1234567", 3), "5671234");
    }

    #[test]
    fn aoc2016_21_op_rotate_letter_position() {
        assert_eq!(rotate_letter_position("abdec", 'b'), "ecabd");
        assert_eq!(rotate_letter_position("ecabd", 'd'), "decab");
    }

    #[test]
    fn aoc2016_21_op_move_position() {
        assert_eq!(move_position("bcdea", 1, 4), "bdeac");
        assert_eq!(move_position("bdeac", 3, 0), "abdec");
    }
}
