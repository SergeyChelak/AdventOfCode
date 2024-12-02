use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = i32;

pub struct AoC2024_02 {
    reports: Vec<Vec<Int>>,
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
    if report.is_empty() {
        return true;
    }
    let mut direction = 0;
    let count = report.len() - 1;
    let skip_check = |index: usize| {
        if is_safe_with_skip(report, index) {
            return true;
        }
        if is_safe_with_skip(report, index + 1) {
            return true;
        }
        if index > 0 && is_safe_with_skip(report, index - 1) {
            return true;
        }
        false
    };
    for (i, value) in report.iter().take(count).enumerate() {
        let next = report[i + 1];
        let diff = value.abs_diff(next);
        if !(1u32..=3).contains(&diff) {
            return skip_check(i);
        }

        let current = if *value < next { 1 } else { -1 };
        if direction == 0 {
            direction = current;
            continue;
        }
        if direction != current {
            return skip_check(i);
        }
    }
    true
}

fn is_safe_with_skip(report: &[Int], index: usize) -> bool {
    let filtered = report
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != index)
        .map(|(_, val)| *val)
        .collect::<Vec<Int>>();
    is_safe(&filtered)
}

fn is_safe(report: &[Int]) -> bool {
    let mut direction = 0;
    for (i, item) in report.iter().enumerate() {
        let Some(next) = report.get(i + 1) else {
            break;
        };
        let diff = next.abs_diff(*item);
        if !(1u32..=3).contains(&diff) {
            return false;
        }
        let current = if item < next { 1 } else { -1 };
        if direction == 0 {
            direction = current;
            continue;
        }
        if direction != current {
            return false;
        }
    }
    true
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
