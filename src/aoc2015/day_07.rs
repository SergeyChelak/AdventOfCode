use crate::solution::Solution;
use crate::file_utils::*;

use std::io;
use std::collections::HashMap;

type Int = usize;

enum Argument {
    Const(Int),
    Token(String),
}

impl Argument {
    fn from_str(s: &str) -> Self {
        if let Ok(value) = s.parse::<Int>() {
            Argument::Const(value)
        } else {
            Argument::Token(s.to_string())
        }
    }
}

enum Expression {
    Unary(String, Argument),
    Binary(String, Argument, Argument),
}

enum Token {
    Value(Argument),
    Expression(Expression)
}

pub struct AoC2015_07 {
    tokens: HashMap<String, Token>,
}

impl AoC2015_07 {
    pub fn new() -> io::Result<Self> { 
        Ok(Self {
            tokens: Self::parse_input()?
        })
    }

    fn parse_input() -> io::Result<HashMap<String, Token>> {
        let tokens = read_file_as_lines("input/aoc2015_07")?
            .iter()
            .map(|line| Self::parse_line(line))
            .collect::<HashMap<String, Token>>();
        Ok(tokens)
    }

    fn parse_line(line: &str) -> (String, Token) {
        println!("{line}");
        let components = line.split(' ').collect::<Vec<&str>>();
        let count = components.len();
        if count < 2 || components[count - 2] != "->" {
            panic!("Invalid expression: {line}")
        }
        let token_name = components.last().expect("Input line shouldn't be empty");
        let token = match count {
            3 => Self::parse_assign(&components),
            4 => Self::parse_unary(&components),
            5 => Self::parse_binary(&components),
            _ => panic!("Invalid expression: {}", line),
        };
        (token_name.to_string(), token)
    }

    fn parse_assign(comp: &Vec<&str>) -> Token {
        let arg = Argument::from_str(comp[0]);
        Token::Value(arg)
    }

    fn parse_unary(comp: &Vec<&str>) -> Token {
        let fn_name = comp[0].to_string();
        let arg = Argument::from_str(comp[1]);
        let expr = Expression::Unary(fn_name, arg);
        Token::Expression(expr)
    }

    fn parse_binary(comp: &Vec<&str>) -> Token {
        let arg1 = Argument::from_str(comp[0]);
        let fn_name = comp[1].to_string();
        let arg2 = Argument::from_str(comp[2]);
        let expr = Expression::Binary(fn_name, arg1, arg2);
        Token::Expression(expr)
    }
}

impl Solution for AoC2015_07 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
    	"AoC 2015/Day 7: Some Assembly Required".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_07_input_load_test() -> io::Result<()> {
        let solution = AoC2015_07::new()?;
        assert!(solution.tokens.len() > 0);
        Ok(())
    }

    #[test]
    fn aoc2015_07_correctness() -> io::Result<()> {
        Ok(())
    }
}