use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = usize;

const OP_ADD: char = '+';
const OP_MUL: char = '*';

pub struct AoC2025_06 {
    numbers: Vec2<Int>,
    operations: Vec<char>,
}

impl AoC2025_06 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2025_06")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut numbers = Vec2::<Int>::new();
        let mut operations = Vec::<char>::new();

        for line in lines
            .iter()
            .map(|x| x.as_ref())
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
        {
            let tokens = line
                .split(char::is_whitespace)
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>();
            if line.contains('*') {
                for token in tokens {
                    match token {
                        "*" => operations.push(OP_MUL),
                        "+" => operations.push(OP_ADD),
                        _ => unreachable!("Unexpected operation"),
                    }
                }
            } else {
                let elems = tokens
                    .iter()
                    .map(|x| x.parse::<Int>().expect("Elements must be integer"))
                    .collect::<Vec<_>>();
                numbers.push(elems);
            }
        }

        Self {
            numbers,
            operations,
        }
    }
}

type Block = dyn Fn(Int, Int) -> Int;

impl Solution for AoC2025_06 {
    fn part_one(&self) -> String {
        let add: &Block = &|a: Int, b: Int| -> Int { a + b };
        let mul: &Block = &|a: Int, b: Int| -> Int { a * b };
        let mut result = 0;
        for (col, op) in self.operations.iter().enumerate() {
            let (initial, block) = match *op {
                OP_ADD => (0, add),
                OP_MUL => (1, mul),
                _ => unreachable!("unexpected operation"),
            };
            result += self.numbers.iter().map(|x| x[col]).fold(initial, block);
        }
        result.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 6: Trash Compactor".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2025_06_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.numbers.is_empty());
        assert!(!sol.operations.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2025_06_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "5171061464548");
        Ok(())
    }

    #[test]
    fn aoc2025_06_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2025_06> {
        AoC2025_06::new()
    }
}
