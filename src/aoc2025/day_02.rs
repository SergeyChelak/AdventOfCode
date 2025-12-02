use crate::solution::Solution;

use std::io;

type Int = usize;

struct Range {
    first: Int,
    last: Int,
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let (first, last) = value.split_once('-').expect("Invalid input format");
        let first = first
            .parse::<Int>()
            .expect("Integer value expected in ranges");
        let last = last
            .parse::<Int>()
            .expect("Integer value expected in ranges");
        Self { first, last }
    }
}

impl Range {
    fn id_sum(&self, criteria: impl Fn(Int) -> bool) -> usize {
        (self.first..=self.last)
            .filter(|x| criteria(*x))
            .sum::<usize>()
    }
}

fn digits_repeating(x: Int) -> bool {
    let chars = x.to_string().chars().collect::<Vec<_>>();
    for size in 1..=chars.len() >> 1 {
        let mut chunks = chars.chunks(size);
        let Some(first) = chunks.next() else {
            continue;
        };
        if chunks.all(|chunk| chunk == first) {
            return true;
        }
    }
    false
}

fn digits_repeat_twice(x: Int) -> bool {
    let digits = 1 + x.ilog10();
    if !digits.is_multiple_of(2) {
        return false;
    }
    let factor = 10usize.pow(digits >> 1);
    let left = x / factor;
    let right = x % factor;
    left == right
}

pub struct AoC2025_02 {
    input: Vec<Range>,
}

impl AoC2025_02 {
    pub fn new() -> io::Result<Self> {
        let input = std::fs::read_to_string("input/aoc2025_02")?;
        Ok(Self::parse(&input))
    }

    fn parse(input: &str) -> Self {
        let ranges = input.trim().split(',').map(Range::from).collect::<Vec<_>>();
        Self { input: ranges }
    }

    fn calculate(&self, criteria: impl Fn(Int) -> bool) -> String {
        self.input
            .iter()
            .map(|x| x.id_sum(&criteria))
            .sum::<Int>()
            .to_string()
    }
}

impl Solution for AoC2025_02 {
    fn part_one(&self) -> String {
        self.calculate(digits_repeat_twice)
    }

    fn part_two(&self) -> String {
        self.calculate(digits_repeating)
    }

    fn description(&self) -> String {
        "Day 2: Gift Shop".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2025_02_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2025_02_digit_repeat_twice_test() {
        assert!(digits_repeat_twice(11));
        assert!(digits_repeat_twice(22));
        assert!(!digits_repeat_twice(10));
        assert!(digits_repeat_twice(1010));
        assert!(digits_repeat_twice(1188511885));
        assert!(digits_repeat_twice(38593859));
    }

    #[test]
    fn aoc2025_02_digits_repeating_test() {
        assert!(digits_repeating(11));
        assert!(!digits_repeating(10));
        assert!(digits_repeating(1111111));
        assert!(digits_repeating(22));
        assert!(digits_repeating(999));
        assert!(digits_repeating(1010));
        assert!(digits_repeating(1188511885));
        assert!(digits_repeating(2121212121));
        assert!(digits_repeating(824824824));
    }

    #[test]
    fn aoc2025_02_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "19605500130");
        Ok(())
    }

    #[test]
    fn aoc2025_02_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "36862281418");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2025_02> {
        AoC2025_02::new()
    }
}
