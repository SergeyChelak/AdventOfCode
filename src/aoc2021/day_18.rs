use crate::solution::Solution;
use crate::utils::*;

use std::collections::LinkedList;
use std::io;

type Int = u32;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Regular(Int),
    Begin,
    End,
}

struct Digit {
    raw: LinkedList<Token>,
}

impl Digit {
    fn zero() -> Self {
        Self {
            raw: LinkedList::new(),
        }
    }

    fn magnitude(&self) -> Int {
        let mut stack = Vec::<Int>::new();

        for token in self.raw.iter() {
            match token {
                Token::Begin => continue,
                Token::Regular(x) => stack.push(*x),
                Token::End => {
                    let right = stack.pop().expect("Right value is missing");
                    let left = stack.pop().expect("Left value is missing");
                    let value = 3 * left + 2 * right;
                    stack.push(value);
                }
            }
        }

        assert!(stack.len() == 1, "Broken input");
        stack.pop().unwrap()
    }

    fn sum(&self, other: &Self) -> Self {
        let mut result = self
            .raw
            .iter()
            .chain(other.raw.iter())
            .cloned()
            .collect::<LinkedList<_>>();
        if !self.raw.is_empty() && !other.raw.is_empty() {
            result.push_front(Token::Begin);
            result.push_back(Token::End);
        }
        Self { raw: result }
    }

    fn sum_reduced(&self, other: &Self) -> Self {
        let mut result = self.sum(other);
        result.reduce();
        result
    }

    fn reduce(&mut self) {
        loop {
            if self.explode() {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
    }

    fn explode(&mut self) -> bool {
        let mut prev: Option<Token> = None;
        let mut depth = 0;
        let mut split_index: Option<usize> = None;
        for (index, token) in self.raw.iter().enumerate() {
            match (token, prev) {
                (Token::Begin, _) => depth += 1,
                (Token::End, _) => depth -= 1,
                (Token::Regular(_), Some(Token::Regular(_))) if depth > 4 => {
                    split_index = Some(index);
                    break;
                }
                _ => {}
            };
            prev = Some(token.clone());
        }

        let Some(split_index) = split_index else {
            return false;
        };

        let mut tail = self.raw.split_off(split_index + 1);

        let right = self.raw.pop_back().expect("Right did found before");
        let left = self.raw.pop_back().expect("Left did found before");

        if let Some(node) = self
            .raw
            .iter_mut()
            .rfind(|t| matches!(t, Token::Regular(_)))
        {
            *node = token_sum(&left, node);
        }

        if let Some(node) = tail.iter_mut().find(|t| matches!(t, Token::Regular(_))) {
            *node = token_sum(&right, node);
        }

        _ = self.raw.pop_back(); // remove '['
        _ = tail.pop_front(); // remove ']'

        self.raw.push_back(Token::Regular(0));

        self.raw.append(&mut tail);
        true
    }

    fn split(&mut self) -> bool {
        let mut split_index: Option<usize> = None;
        for (index, token) in self.raw.iter().enumerate() {
            match token {
                Token::Regular(x) if *x > 9 => {
                    split_index = Some(index);
                    break;
                }
                _ => {}
            };
        }
        let Some(split_index) = split_index else {
            return false;
        };
        let mut tail = self.raw.split_off(split_index);

        let value = tail.pop_front().unwrap();
        match value {
            Token::Regular(x) => {
                tail.push_front(Token::End);
                let left = x / 2;
                let right = x - left;
                tail.push_front(Token::Regular(right));
                tail.push_front(Token::Regular(left));
                tail.push_front(Token::Begin);
            }
            _ => unreachable!(),
        }

        self.raw.append(&mut tail);
        true
    }
}

fn token_sum(first: &Token, second: &Token) -> Token {
    match (first, second) {
        (Token::Regular(a), Token::Regular(b)) => Token::Regular(*a + *b),
        _ => unreachable!(),
    }
}

impl From<&str> for Digit {
    fn from(value: &str) -> Self {
        let raw = value
            .chars()
            .filter_map(|ch| match ch {
                '[' => Some(Token::Begin),
                ']' => Some(Token::End),
                x if x.is_ascii_digit() => Some(Token::Regular(x.to_digit(10).unwrap())),
                _ => None,
            })
            .collect::<LinkedList<_>>();
        Self { raw }
    }
}

pub struct AoC2021_18 {
    input: Vec<Digit>,
}

impl AoC2021_18 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2021_18")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(Digit::from)
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2021_18 {
    fn part_one(&self) -> String {
        let sum = self
            .input
            .iter()
            .fold(Digit::zero(), |acc, val| acc.sum_reduced(val));
        sum.magnitude().to_string()
    }

    fn part_two(&self) -> String {
        let mut max_sum = 0;
        for (i, first) in self.input.iter().enumerate() {
            for (j, second) in self.input.iter().enumerate() {
                if i == j {
                    continue;
                }
                let tmp = first.sum_reduced(second).magnitude();
                max_sum = max_sum.max(tmp);
            }
        }
        max_sum.to_string()
    }

    fn description(&self) -> String {
        "Day 18: Snailfish".to_string()
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Display;

    use super::*;

    #[test]
    fn aoc2021_18_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2021_18_magnitude() {
        let data = [
            ("[9,1]", 29),
            ("[[9,1],[1,9]]", 129),
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
        ];
        for (inp, out) in data {
            let digit = Digit::from(inp);
            assert_eq!(digit.magnitude(), out);
        }
    }

    #[test]
    fn aoc2021_18_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "4417");
        Ok(())
    }

    #[test]
    fn aoc2021_18_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "4796");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2021_18> {
        AoC2021_18::new()
    }

    #[test]
    fn aoc2021_18_explode() {
        let data = [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]][9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]][9,[5,[7,0]]]]",
            ),
        ];
        for (inp, out) in data {
            let mut digit = Digit::from(inp);
            digit.explode();
            let val = format!("{digit}");
            assert_eq!(out, val);
        }
    }

    #[test]
    fn aoc2021_18_split() {
        let mut digit = Digit::from("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        assert!(digit.explode());
        assert!(digit.explode());
        digit.split();
        let val = format!("{digit}");
        assert_eq!("[[[[0,7],4][[7,8][0,13]]][1,1]]", val);
    }

    impl Display for Digit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut prev: Option<Token> = None;
            for token in self.raw.iter() {
                match (token, prev) {
                    (Token::Begin, _) => write!(f, "[")?,
                    (Token::End, _) => write!(f, "]")?,
                    (Token::Regular(left), Some(Token::End)) => write!(f, ",{left}")?,
                    (Token::Regular(left), Some(Token::Begin)) => write!(f, "{left},")?,
                    (Token::Regular(val), _) => write!(f, "{val}")?,
                };
                prev = Some(token.clone());
            }
            Ok(())
        }
    }
}
