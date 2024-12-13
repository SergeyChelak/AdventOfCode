use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = i32;

pub struct AoC2024_02 {
    reports: Vec2<Int>,
}

impl AoC2024_02 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2024_02")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let mut reports = Vec::new();
        for line in lines {
            let report = parse_report(line);
            reports.push(report);
        }
        Self { reports }
    }

    fn safe_count(&self, predicate: impl Fn(&[Int]) -> bool) -> usize {
        self.reports.iter().filter(|rep| predicate(rep)).count()
    }
}

impl Solution for AoC2024_02 {
    fn part_one(&self) -> String {
        self.safe_count(is_safe).to_string()
    }

    fn part_two(&self) -> String {
        self.safe_count(is_tolerate_safe).to_string()
    }

    fn description(&self) -> String {
        "2024/Day 2: Red-Nosed Reports".to_string()
    }
}

fn parse_report(line: &str) -> Vec<Int> {
    line.split_whitespace()
        .map(|s| s.parse::<Int>().expect("Non integer value passed {s}"))
        .collect::<Vec<Int>>()
}

fn is_tolerate_safe(report: &[Int]) -> bool {
    let count = report.len();
    for index in 0..count {
        let filtered = report
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != index)
            .map(|(_, val)| *val)
            .collect::<Vec<Int>>();
        if is_safe(&filtered) {
            return true;
        }
    }
    false
}

fn is_safe(report: &[Int]) -> bool {
    let mut iter = report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(l, r)| *l - *r);
    iter.clone().all(|val| (1..=3).contains(&val)) || iter.all(|val| (-3..=-1).contains(&val))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_02_input_load_test() -> io::Result<()> {
        let sol = AoC2024_02::new()?;
        assert!(!sol.reports.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2024_02_correctness() -> io::Result<()> {
        let sol = AoC2024_02::new()?;
        assert_eq!(sol.part_one(), "236");
        assert_eq!(sol.part_two(), "308");
        Ok(())
    }

    #[test]
    fn aoc2024_02_is_safe() {
        let check = |s: &str| {
            let rep = parse_report(s);
            is_safe(&rep)
        };
        assert!(check("7 6 4 2 1"));
        assert!(!check("1 2 7 8 9"));
        assert!(!check("9 7 6 2 1"));
        assert!(!check("1 3 2 4 5"));
        assert!(!check("8 6 4 4 1"));
        assert!(check("1 3 6 7 9"));
    }

    #[test]
    fn aoc2024_02_is_tolerate_safe() {
        let check = |s: &str| {
            let rep = parse_report(s);
            is_tolerate_safe(&rep)
        };
        assert!(check("7 6 4 2 1"));
        assert!(!check("1 2 7 8 9"));
        assert!(!check("9 7 6 2 1"));
        assert!(check("1 3 2 4 5"));
        assert!(check("8 6 4 4 1"));
        assert!(check("1 3 6 7 9"));
    }
}
