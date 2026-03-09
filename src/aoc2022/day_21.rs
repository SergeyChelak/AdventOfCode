use crate::solution::Solution;
use crate::utils::not_found;

use std::collections::HashMap;
use std::io;

type Int = i64;

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

enum Value {
    Expression(String, Operation, String),
    Const(Int),
}

fn parse_expression(input: &str) -> (String, Value) {
    let (name, expr) = input.split_once(": ").expect("Incorrect expression");

    let tokens = expr.split_ascii_whitespace().collect::<Vec<_>>();
    let value = match tokens.len() {
        3 => {
            let l = tokens[0].to_string();
            let op = Operation::from(tokens[1]);
            let r = tokens[2].to_string();
            Value::Expression(l, op, r)
        }
        1 => {
            let val = tokens[0].parse::<Int>().expect("Invalid constant value");
            Value::Const(val)
        }
        _ => unreachable!(),
    };

    (name.to_string(), value)
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => unreachable!(),
        }
    }
}

type Expressions = HashMap<String, Value>;

pub struct AoC2022_21 {
    input: Expressions,
}

impl AoC2022_21 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2022_21")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        Self::parse_lines(&data.lines().collect::<Vec<_>>())
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|s| s.as_ref())
            .map(parse_expression)
            .collect::<Expressions>();
        Self { input }
    }
}

impl Solution for AoC2022_21 {
    fn part_one(&self) -> String {
        eval(ROOT, None, &self.input)
            .map(|x| x.to_string())
            .unwrap_or(not_found())
    }

    fn part_two(&self) -> String {
        calc(&self.input)
            .map(|x| x.to_string())
            .unwrap_or(not_found())
    }

    fn description(&self) -> String {
        "Day 21: Monkey Math".to_string()
    }
}

const ROOT: &str = "root";
const HUMAN: &str = "humn";

fn calc(exprs: &Expressions) -> Option<Int> {
    let root = exprs.get(ROOT)?;
    match root {
        Value::Expression(l, _, r) => {
            let eval_left = eval(l, Some(HUMAN), exprs);
            let eval_right = eval(r, Some(HUMAN), exprs);
            match (eval_left, eval_right) {
                (None, Some(x)) => find_value(l, x, exprs),
                (Some(x), None) => find_value(r, x, exprs),
                _ => None,
            }
        }
        _ => None,
    }
}

fn find_value(key: &str, target: Int, exprs: &Expressions) -> Option<Int> {
    if key == HUMAN {
        return Some(target);
    }
    let val = exprs.get(key)?;
    match val {
        Value::Expression(l, op, r) => {
            let eval_left = eval(l, Some(HUMAN), exprs);
            let eval_right = eval(r, Some(HUMAN), exprs);

            match (eval_left, eval_right) {
                (None, Some(x)) => {
                    // target = X op Const
                    let expected = match op {
                        Operation::Add => target - x,
                        Operation::Sub => target + x,
                        Operation::Mul => target / x,
                        Operation::Div => target * x,
                    };

                    find_value(l, expected, exprs)
                }
                (Some(x), None) => {
                    // target = Const op X
                    let expected = match op {
                        Operation::Add => target - x,
                        Operation::Sub => x - target,
                        Operation::Mul => target / x,
                        Operation::Div => x / target,
                    };

                    find_value(r, expected, exprs)
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn eval(key: &str, var: Option<&str>, exprs: &Expressions) -> Option<Int> {
    if Some(key) == var {
        return None;
    }
    let val = exprs.get(key)?;
    match val {
        Value::Const(x) => Some(*x),
        Value::Expression(l, op, r) => {
            let l_val = eval(l.as_str(), var, exprs)?;
            let r_val = eval(r.as_str(), var, exprs)?;
            let calc = match op {
                Operation::Add => l_val + r_val,
                Operation::Sub => l_val - r_val,
                Operation::Mul => l_val * r_val,
                Operation::Div => l_val / r_val,
            };
            Some(calc)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_21_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2022_21_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "268597611536314");
        Ok(())
    }

    #[test]
    fn aoc2022_21_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "3451534022348");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2022_21> {
        AoC2022_21::new()
    }
}
