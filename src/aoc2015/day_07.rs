use crate::solution::Solution;
use crate::file_utils::*;

use std::io;
use std::collections::HashMap;

type Int = u16;
type Memo = HashMap<String, Int>;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
enum Token {
    Value(Argument),
    Function(String, Vec<Argument>)
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
        let lines = read_file_as_lines("input/aoc2015_07")?;
        Ok(Self::parse_input_from_lines(&lines))
    }

    fn parse_input_from_lines(lines: &Vec<String>) -> HashMap<String, Token> {
        lines
            .iter()
            .map(|line| Self::parse_line(line))
            .collect::<HashMap<String, Token>>()
    }

    fn parse_line(line: &str) -> (String, Token) {
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
        Token::Function(fn_name, vec![arg])
    }

    fn parse_binary(comp: &Vec<&str>) -> Token {
        let arg1 = Argument::from_str(comp[0]);
        let fn_name = comp[1].to_string();
        let arg2 = Argument::from_str(comp[2]);
        Token::Function(fn_name, vec![arg1, arg2])
    }

    fn eval(&self, name: &str, memo: &mut Memo) -> Int {
        if let Some(value) = memo.get(name) {
            return *value;
        }
        let Some(token) = self.tokens.get(name) else {
            panic!("Token '{name}' not found")
        };
        let value = match token {
            Token::Value(arg) => {
                self.eval_arg(arg, memo)
            },
            Token::Function(fn_name, args) => {
                let params = args.iter()
                    .map(|arg| self.eval_arg(arg, memo))
                    .collect::<Vec<Int>>();
                Self::compute(fn_name, &params)
            },
        };
        memo.insert(name.to_string(), value);
        value
    }

    fn eval_arg(&self, arg: &Argument, memo: &mut Memo) -> Int {
        match arg {
            Argument::Const(v) => *v,
            Argument::Token(other) => self.eval(other, memo),
        }
    }

    fn compute(fn_name: &str, params: &Vec<Int>) -> Int {
        match fn_name {
            "AND" => params[0] & params[1],
            "OR" => params[0] | params[1],
            "NOT" => !params[0],
            "RSHIFT" => params[0] >> params[1],
            "LSHIFT" => params[0] << params[1],
            _ => panic!("Unknown function '{fn_name}")
        }
    }
}

impl Solution for AoC2015_07 {
    fn part_one(&self) -> String {
        let mut memo: Memo = HashMap::new();
        self.eval("a", &mut memo)
            .to_string()
    }

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
        let sol = AoC2015_07::new()?;
        assert_eq!(sol.part_one(), "3176");
        // assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2015_07_case1() {
        let input = vec![
            "123 -> x",
            "456 -> y",
            "x AND y -> d",
            "x OR y -> e",
            "x LSHIFT 2 -> f",
            "y RSHIFT 2 -> g",
            "NOT x -> h",
            "NOT y -> i"
        ].iter().map(|x| x.to_string()).collect();
        let tokens = AoC2015_07::parse_input_from_lines(&input);
        let solution = AoC2015_07 {
            tokens,
        };
        let mut memo: Memo = HashMap::new();
        assert_eq!(solution.eval("d", &mut memo), 72);
        assert_eq!(solution.eval("e", &mut memo), 507);
        assert_eq!(solution.eval("f", &mut memo), 492);
        assert_eq!(solution.eval("g", &mut memo), 114);
        assert_eq!(solution.eval("h", &mut memo), 65412);
        assert_eq!(solution.eval("i", &mut memo), 65079);
        assert_eq!(solution.eval("x", &mut memo), 123);
        assert_eq!(solution.eval("y", &mut memo), 456);
    }
}