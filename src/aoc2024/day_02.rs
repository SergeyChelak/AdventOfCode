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
}

impl Solution for AoC2024_02 {
    fn part_one(&self) -> String {
        self.reports
            .iter()
            .filter(|rep| is_safe(rep))
            .count()
            .to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "2024/Day 2: Red-Nosed Reports".to_string()
    }
}

fn parse_report(line: &str) -> Vec<Int> {
    line.split_whitespace()
        .map(|s| s.parse::<Int>().expect("Non integer value passed {s}"))
        .collect::<Vec<Int>>()
}

#[derive(PartialEq)]
enum Progression {
    Undefined,
    Increasing,
    Decreasing,
}

fn is_safe(report: &[Int]) -> bool {
    let mut prog = Progression::Undefined;
    for (i, item) in report.iter().enumerate() {
        let Some(next) = report.get(i + 1) else {
            break;
        };
        let diff = next.abs_diff(*item);
        if !(1u32..=3).contains(&diff) {
            return false;
        }
        let cur_prog = if item < next {
            Progression::Increasing
        } else {
            Progression::Decreasing
        };
        if prog == Progression::Undefined {
            prog = cur_prog;
            continue;
        }
        if prog != cur_prog {
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
        assert_eq!(sol.part_two(), "");
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
}
