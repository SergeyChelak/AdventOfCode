use crate::solution::Solution;
use crate::utils::*;

use std::{io, panic};

type Value = isize;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Number(Value),
    OpenParenthesis,
    CloseParenthesis,
    Plus,
    Star,
}

fn precedence_pt1(token: &Token) -> u8 {
    match token {
        Token::Number(_) | Token::OpenParenthesis | Token::CloseParenthesis => 0,
        Token::Plus => 1,
        Token::Star => 1,
    }
}

fn precedence_pt2(token: &Token) -> u8 {
    match token {
        Token::Number(_) | Token::OpenParenthesis | Token::CloseParenthesis => 0,
        Token::Plus => 2,
        Token::Star => 1,
    }
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let chars = {
        let mut arr = input.chars().collect::<Vec<_>>();
        arr.push('\0');
        arr
    };

    let mut idx = 0;
    while let Some(ch) = chars.get(idx) {
        match ch {
            '+' => tokens.push(Token::Plus),
            '*' => tokens.push(Token::Star),
            '(' => tokens.push(Token::OpenParenthesis),
            ')' => tokens.push(Token::CloseParenthesis),
            x if x.is_numeric() => {
                let mut end = idx;
                while chars[end].is_numeric() {
                    end += 1;
                }
                let value = chars[idx..end]
                    .iter()
                    .collect::<String>()
                    .parse::<Value>()
                    .expect("Operands must be numbers");
                tokens.push(Token::Number(value));
                idx = end;
                continue;
            }
            _ => {}
        }
        idx += 1;
    }
    tokens
}

/// converts natural expression to Reverse Polish Notation
fn convert_to_rpn(tokens: &[Token], prec: &dyn Fn(&Token) -> u8) -> Vec<Token> {
    let mut stack = Vec::<Token>::new();
    let mut rpn = Vec::new();

    for token in tokens {
        let t = token.clone();
        match token {
            Token::Number(_) => rpn.push(t),
            Token::OpenParenthesis => stack.push(t),
            Token::CloseParenthesis => {
                while let Some(token) = stack.pop() {
                    if matches!(token, Token::OpenParenthesis) {
                        break;
                    }
                    rpn.push(token);
                }
            }
            Token::Plus | Token::Star => {
                while stack.last().map(prec).unwrap_or(0) >= prec(token) {
                    let Some(op) = stack.pop() else {
                        break;
                    };
                    rpn.push(op);
                }
                stack.push(t);
            }
        }
    }

    while let Some(token) = stack.pop() {
        rpn.push(token);
    }
    rpn
}

/// evaluates Reverse Polish Notation
fn eval_rpn(tokens: &[Token]) -> Value {
    let mut stack = Vec::<Token>::new();

    let eval_binary = |s: &mut Vec<Token>, op: &dyn Fn(Value, Value) -> Value| -> Token {
        let b = s.pop().expect("Broken stack");
        let a = s.pop().expect("Broken stack");

        match (a, b) {
            (Token::Number(x), Token::Number(y)) => Token::Number(op(x, y)),
            _ => panic!("Invalid operand"),
        }
    };

    for t in tokens {
        let next = match t {
            Token::Number(_) => t.clone(),
            Token::Plus => eval_binary(&mut stack, &|a, b| a + b),
            Token::Star => eval_binary(&mut stack, &|a, b| a * b),
            _ => panic!("Unexpected token"),
        };
        stack.push(next);
    }

    assert_eq!(1, stack.len());
    match stack.last() {
        Some(Token::Number(x)) => *x,
        _ => panic!("Failed to evaluate RPN"),
    }
}

fn eval(s: &str, prec: &dyn Fn(&Token) -> u8) -> Value {
    let tokens = tokenize(s);
    let rpn = convert_to_rpn(&tokens, prec);
    eval_rpn(&rpn)
}

pub struct AoC2020_18 {
    input: Vec<String>,
}

impl AoC2020_18 {
    pub fn new() -> io::Result<Self> {
        let input = read_file_as_lines("input/aoc2020_18")?;
        Ok(Self { input })
    }

    fn calculate(&self, prec: &dyn Fn(&Token) -> u8) -> String {
        self.input
            .iter()
            .map(|s| eval(s.as_str(), &prec))
            .sum::<Value>()
            .to_string()
    }
}

impl Solution for AoC2020_18 {
    fn part_one(&self) -> String {
        self.calculate(&precedence_pt1)
    }

    fn part_two(&self) -> String {
        self.calculate(&precedence_pt2)
    }

    fn description(&self) -> String {
        "Day 18: Operation Order".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_18_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2020_18_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "1402255785165");
        Ok(())
    }

    #[test]
    fn aoc2020_18_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "119224703255966");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_18> {
        AoC2020_18::new()
    }

    #[test]
    fn aoc2020_18_tokenize() {
        let inp = "2 * 31 + (456 * 5000)";
        let tokens = tokenize(inp);
        assert_eq!(
            tokens,
            vec![
                Token::Number(2),
                Token::Star,
                Token::Number(31),
                Token::Plus,
                Token::OpenParenthesis,
                Token::Number(456),
                Token::Star,
                Token::Number(5000),
                Token::CloseParenthesis
            ]
        );
    }

    #[test]
    fn aoc2020_18_convert_rpn() {
        {
            let input = [
                Token::OpenParenthesis,
                Token::Number(5),
                Token::Star,
                Token::Number(4),
                Token::Plus,
                Token::Number(3),
                Token::Star,
                Token::Number(2),
                Token::CloseParenthesis,
                Token::Plus,
                Token::Number(1),
            ];
            let output = convert_to_rpn(&input, &normal_precedence);
            let expected = [
                Token::Number(5),
                Token::Number(4),
                Token::Star,
                Token::Number(3),
                Token::Number(2),
                Token::Star,
                Token::Plus,
                Token::Number(1),
                Token::Plus,
            ];
            assert_eq!(output, expected);
        }
    }

    #[test]
    fn aoc2020_18_eval() {
        let data = [
            ("1 + 2 * 3 + 4 * 5 + 6", 71),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 26),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
        ];
        for (inp, out) in data {
            assert_eq!(eval(inp, &precedence_pt1), out)
        }
    }

    fn normal_precedence(token: &Token) -> u8 {
        match token {
            Token::Number(_) | Token::OpenParenthesis | Token::CloseParenthesis => 0,
            Token::Plus => 1,
            Token::Star => 2,
        }
    }
}
