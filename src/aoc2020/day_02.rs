use crate::solution::Solution;
use crate::utils::*;

use std::io;

struct Record {
    low: usize,
    high: usize,
    ch: char,
    password: String,
}

impl Record {
    fn is_valid_by_range(&self) -> bool {
        let count = self.password.chars().filter(|x| *x == self.ch).count();
        (self.low..=self.high).contains(&count)
    }

    fn is_valid_by_position(&self) -> bool {
        let chars = self.password.chars().collect::<Vec<_>>();
        if self.low == 0 || self.high == 0 {
            return false;
        }
        let (Some(&a), Some(&b)) = (chars.get(self.low - 1), chars.get(self.high - 1)) else {
            return false;
        };

        (a == self.ch || b == self.ch) && a != b
    }
}

impl From<&str> for Record {
    fn from(value: &str) -> Self {
        let (policy, password) = value.split_once(": ").expect("Invalid record format");
        let (range, ch) = policy.split_once(" ").expect("Invalid policy format");
        let (low, high) = range.split_once("-").expect("Invalid range format");
        let low = low.parse::<usize>().expect("Low value must be integer");
        let high = high.parse::<usize>().expect("High value must be integer");
        let ch = ch.chars().next().expect("Char is expected");
        Self {
            low,
            high,
            ch,
            password: password.to_string(),
        }
    }
}

pub struct AoC2020_02 {
    input: Vec<Record>,
}

impl AoC2020_02 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2020_02")?;
        Ok(Self::parse(&lines))
    }

    fn parse<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines.iter().map(|x| x.as_ref()).map(Record::from).collect();
        Self { input }
    }

    fn password_count(&self, criteria: impl Fn(&Record) -> bool) -> String {
        self.input
            .iter()
            .filter(|x| criteria(x))
            .count()
            .to_string()
    }
}

impl Solution for AoC2020_02 {
    fn part_one(&self) -> String {
        self.password_count(|x| x.is_valid_by_range())
    }

    fn part_two(&self) -> String {
        self.password_count(|x| x.is_valid_by_position())
    }

    fn description(&self) -> String {
        "Day 2: Password Philosophy".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_02_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2020_02_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "655");
        Ok(())
    }

    #[test]
    fn aoc2020_02_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "673");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_02> {
        AoC2020_02::new()
    }
}
