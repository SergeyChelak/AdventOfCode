use crate::solution::Solution;

use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;

type Int = u64;

pub struct AoC2024_11 {
    numbers: Vec<Int>,
}

impl AoC2024_11 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2024_11")?;
        Ok(Self::with_str(&input))
    }

    fn with_str(s: &str) -> Self {
        let numbers = s
            .split_whitespace()
            .map(|s| s.parse::<Int>().expect("Non numerical value found"))
            .collect::<Vec<_>>();
        Self { numbers }
    }
}

impl Solution for AoC2024_11 {
    fn part_one(&self) -> String {
        simulate(&self.numbers, 25).to_string()
    }

    fn part_two(&self) -> String {
        simulate(&self.numbers, 75).to_string()
    }

    fn description(&self) -> String {
        "2024/Day 11: Plutonian Pebbles".to_string()
    }
}

fn simulate(numbers: &[Int], times: u8) -> usize {
    let mut memo = HashMap::new();
    let mut total = 0;
    for value in numbers {
        total += count(*value, times, &mut memo);
    }
    total
}

fn count(value: Int, times: u8, memo: &mut HashMap<(Int, u8), usize>) -> usize {
    if times == 0 {
        return 1;
    }
    let key = (value, times);
    if let Some(result) = memo.get(&key) {
        return *result;
    }
    let result = if value == 0 {
        count(1, times - 1, memo)
    } else {
        let digits = digits_count(value);
        if digits.is_multiple_of(2) {
            let (a, b) = split(value, digits / 2);
            count(a, times - 1, memo) + count(b, times - 1, memo)
        } else {
            count(value * 2024, times - 1, memo)
        }
    };
    memo.insert(key, result);
    result
}

fn split(value: Int, at: u8) -> (Int, Int) {
    let f = pow10(at);
    let a = value / f;
    let b = value % f;
    (a, b)
}

fn pow10(i: u8) -> Int {
    if i == 0 {
        return 1;
    }
    (0..i).fold(1, |acc, _| acc * 10)
}

fn digits_count(value: Int) -> u8 {
    (value as f32).log10() as u8 + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_11_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.numbers.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2024_11_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "183435");
        Ok(())
    }

    #[test]
    fn aoc2024_11_case_1() -> io::Result<()> {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "55312");
        Ok(())
    }

    #[test]
    fn aoc2024_11_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "218279375708592");
        Ok(())
    }

    #[test]
    fn aoc2024_11_digits_count() {
        assert_eq!(1, digits_count(1));
        assert_eq!(2, digits_count(12));
        assert_eq!(3, digits_count(234));
        assert_eq!(4, digits_count(3456));
        assert_eq!(5, digits_count(45678));
    }

    #[test]
    fn aoc2024_11_pow10() {
        assert_eq!(1, pow10(0));
        assert_eq!(10, pow10(1));
        assert_eq!(100, pow10(2));
        assert_eq!(1000, pow10(3));
        assert_eq!(10000, pow10(4));
    }

    #[test]
    fn aoc2024_11_split() {
        assert_eq!(split(10, 1), (1, 0));
        assert_eq!(split(12, 1), (1, 2));
        assert_eq!(split(2340, 2), (23, 40));
        assert_eq!(split(2304, 2), (23, 4));
        assert_eq!(split(2345, 2), (23, 45));
        assert_eq!(split(234567, 3), (234, 567));
    }

    fn make_solution() -> io::Result<AoC2024_11> {
        AoC2024_11::new()
    }

    fn make_test_solution() -> AoC2024_11 {
        AoC2024_11::with_str("125 17")
    }
}
