use crate::solution::Solution;

use std::fmt::Debug;
use std::io;
use std::str::FromStr;

type Int = u64;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<Int>,
    transform: Transform,
    logic: Logic,
}

impl Monkey {
    fn throw(&mut self, worry_div: Int) -> Vec<Throw> {
        let result = self
            .items
            .iter()
            .map(|x| self.transform.interpret(*x))
            .map(|x| x / worry_div)
            .map(|value| {
                let to = self.logic.index(value);
                Throw { to, value }
            })
            .collect::<Vec<_>>();
        self.items.clear();
        result
    }
}

struct Throw {
    to: usize,
    value: Int,
}

#[derive(Debug, Clone)]
enum Token {
    Arg,
    Add,
    Mul,
    Val(Int),
}

#[derive(Debug, Clone)]
struct Transform {
    tokens: Vec<Token>,
}

#[derive(Debug, Clone)]
struct Logic {
    divider: Int,
    true_idx: usize,
    false_idx: usize,
}

impl Transform {
    fn interpret(&self, arg: Int) -> Int {
        // dummy implementation
        assert!(self.tokens.len() == 3);
        let val = |t: &Token| -> Int {
            match t {
                Token::Arg => arg,
                Token::Val(x) => *x,
                _ => unreachable!(),
            }
        };

        let left = val(&self.tokens[0]);
        let right = val(&self.tokens[2]);

        match self.tokens[1] {
            Token::Add => left + right,
            Token::Mul => left * right,
            _ => unreachable!(),
        }
    }
}

impl Logic {
    fn index(&self, arg: Int) -> usize {
        if arg.is_multiple_of(self.divider) {
            self.true_idx
        } else {
            self.false_idx
        }
    }

    fn with(lines: &[&str]) -> Self {
        let divider = Self::parse::<Int>(lines[0], "Test: divisible by ");
        let true_idx = Self::parse::<usize>(lines[1], "If true: throw to monkey ");
        let false_idx = Self::parse::<usize>(lines[2], "If false: throw to monkey ");
        Logic {
            divider,
            true_idx,
            false_idx,
        }
    }

    fn parse<T>(input: &str, prefix: &str) -> T
    where
        T: FromStr,
        T::Err: Debug,
    {
        input
            .strip_prefix(prefix)
            .expect("Prefix doesn't match")
            .parse::<T>()
            .expect("Failed to parse value")
    }
}

impl From<&str> for Monkey {
    fn from(value: &str) -> Self {
        let lines = value
            .split("\n")
            .skip(1)
            .map(|x| x.trim())
            .collect::<Vec<_>>();

        let items = lines[0]
            .strip_prefix("Starting items: ")
            .map(|s| {
                s.split(", ")
                    .map(|x| x.parse::<Int>().expect("Item must be integer"))
                    .collect::<Vec<_>>()
            })
            .expect("Invalid items format");

        let transform = Transform::from(lines[1]);

        let logic = Logic::with(&lines[2..]);

        Monkey {
            items,
            transform,
            logic,
        }
    }
}

impl From<&str> for Transform {
    fn from(value: &str) -> Self {
        let tokens = value
            .strip_prefix("Operation: new = ")
            .map(|s| {
                s.split_ascii_whitespace()
                    .map(|t| match t {
                        "+" => Token::Add,
                        "*" => Token::Mul,
                        "old" => Token::Arg,
                        _ => Token::Val(
                            t.parse::<Int>()
                                .expect("Integer is expected in the expression"),
                        ),
                    })
                    .collect::<Vec<_>>()
            })
            .expect("Transform data format is wrong");
        Self { tokens }
    }
}

pub struct AoC2022_11 {
    input: Vec<Monkey>,
}

impl AoC2022_11 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2022_11")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        let input = data.split("\n\n").map(Monkey::from).collect::<Vec<_>>();
        Self { input }
    }

    fn simulate(&self, steps: usize, worry_div: Int) -> usize {
        let mut monkeys = self.input.clone();

        // less common multiple
        // Not general case!
        // this works only because input values are prime
        let lcm = monkeys.iter().map(|m| m.logic.divider).product::<Int>();

        let mut inspected = vec![0; monkeys.len()];
        for _ in 0..steps {
            for i in 0..monkeys.len() {
                let arr = monkeys[i].throw(worry_div);
                inspected[i] += arr.len();
                for throw in arr {
                    monkeys[throw.to].items.push(throw.value % lcm);
                }
            }
        }

        inspected.sort();
        inspected.iter().rev().take(2).product::<usize>()
    }
}

impl Solution for AoC2022_11 {
    fn part_one(&self) -> String {
        self.simulate(20, 3).to_string()
    }

    fn part_two(&self) -> String {
        self.simulate(10000, 1).to_string()
    }

    fn description(&self) -> String {
        "Day 11: Monkey in the Middle".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_11_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2022_11_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "117640");
        Ok(())
    }

    #[test]
    fn aoc2022_11_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "30616425600");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2022_11> {
        AoC2022_11::new()
    }
}
