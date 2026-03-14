use crate::solution::Solution;

use std::io;

pub struct AoC2022_25 {
    input: Vec<String>,
}

impl AoC2022_25 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2022_25")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        Self::parse_lines(&data.lines().collect::<Vec<_>>())
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref().trim().to_string())
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2022_25 {
    fn part_one(&self) -> String {
        self.input
            .iter()
            .fold("0".to_string(), |acc, x| sum(acc.as_str(), x.as_str()))
    }

    fn description(&self) -> String {
        "Day 25: Full of Hot Air".to_string()
    }
}

fn sum(x: &str, y: &str) -> String {
    let mut carry: i8 = 0;

    let mut x_iter = x.chars().map(ch2i).rev().peekable();
    let mut y_iter = y.chars().map(ch2i).rev().peekable();

    let mut acc = Vec::new();
    let mut digit: i8;
    while x_iter.peek().is_some() || y_iter.peek().is_some() || carry != 0 {
        let l = x_iter.next().unwrap_or(0);
        let r = y_iter.next().unwrap_or(0);
        (carry, digit) = add(l, r, carry);
        acc.push(digit);
    }
    acc.into_iter().map(i2ch).rev().collect::<String>()
}

fn add(x: i8, y: i8, carry: i8) -> (i8, i8) {
    let sum = x + y + carry;
    let carry = (sum + 2).div_euclid(5);
    let digit = (sum + 2).rem_euclid(5) - 2;
    (carry, digit)
}

fn ch2i(val: char) -> i8 {
    match val {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => unreachable!(),
    }
}

fn i2ch(val: i8) -> char {
    match val {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_25_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2022_25_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "2==221=-002=0-02-000");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2022_25> {
        AoC2022_25::new()
    }
}
