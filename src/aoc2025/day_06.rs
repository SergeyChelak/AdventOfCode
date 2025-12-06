use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = usize;

pub struct AoC2025_06 {
    columns: Vec2<String>,
}

impl AoC2025_06 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2025_06")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut columns = Vec::new();
        let ranges = column_ranges(lines.last().unwrap().as_ref());
        for window in ranges.windows(2) {
            let start = window[0];
            let end = window[1];
            let column = lines
                .iter()
                .map(|x| x.as_ref())
                .map(|s| s[start..end].to_string())
                .collect::<Vec<_>>();
            columns.push(column);
        }
        Self { columns }
    }

    fn compute(&self, transform: impl Fn(usize) -> Vec<Int>) -> String {
        let add: &Block = &|a: Int, b: Int| -> Int { a + b };
        let mul: &Block = &|a: Int, b: Int| -> Int { a * b };
        let mut result = 0;
        for (idx, column) in self.columns.iter().enumerate() {
            let op = column.last().expect("Column can't be empty").trim();
            let (initial, block) = match op {
                "+" => (0, add),
                "*" => (1, mul),
                _ => unreachable!("unexpected operation {op}"),
            };
            let items = transform(idx);
            result += items.into_iter().fold(initial, block);
        }
        result.to_string()
    }
}

type Block = dyn Fn(Int, Int) -> Int;

impl Solution for AoC2025_06 {
    fn part_one(&self) -> String {
        self.compute(|col| {
            self.columns[col]
                .iter()
                .take(self.columns[col].len() - 1)
                .map(|s| s.trim())
                .map(|s| {
                    s.parse::<Int>()
                        .unwrap_or_else(|_| panic!("Failed to parse {}", s))
                })
                .collect::<Vec<_>>()
        })
    }

    fn part_two(&self) -> String {
        self.compute(|col| {
            let items = self.columns[col]
                .iter()
                .take(self.columns[col].len() - 1)
                .map(|s| s.chars().rev().collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let mut result = Vec::new();
            let width = items[0].len();
            for i in 0..width {
                let value = items
                    .iter()
                    .map(|arr| arr[i])
                    .filter_map(|ch| ch.to_digit(10))
                    .fold(0, |acc, val| acc * 10 + val as usize);
                if value > 0 {
                    result.push(value);
                }
            }
            result
        })
    }

    fn description(&self) -> String {
        "Day 6: Trash Compactor".to_string()
    }
}

fn column_ranges(line: &str) -> Vec<usize> {
    #[derive(Clone, Copy)]
    enum State {
        NoPayload,
        Payload,
        OutOfPayload,
    }

    let mut result = vec![0usize];
    let mut state = State::NoPayload;
    for (idx, ch) in line.chars().enumerate() {
        let is_whitespace = ch.is_whitespace();
        match (state, is_whitespace) {
            (State::NoPayload, false) => {
                state = State::Payload;
            }
            (State::Payload, true) => {
                state = State::OutOfPayload;
            }
            (State::OutOfPayload, false) => {
                result.push(idx);
                state = State::Payload;
            }
            _ => {}
        }
    }
    result.push(line.len());
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2025_06_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.columns.is_empty());
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
        assert_eq!(sol.part_two(), "10189959087258");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2025_06> {
        AoC2025_06::new()
    }

    #[test]
    fn aoc2025_06_case_2() {
        let input = [
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ];
        let sol = AoC2025_06::parse_lines(&input);
        assert_eq!(sol.part_two(), "3263827")
    }
}
