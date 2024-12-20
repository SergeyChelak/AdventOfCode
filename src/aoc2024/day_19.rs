use crate::solution::Solution;

use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;

pub struct AoC2024_19 {
    patterns: Vec<String>,
    designs: Vec<String>,
}

impl AoC2024_19 {
    pub fn new() -> io::Result<Self> {
        let string = read_to_string("input/aoc2024_19")?;
        Ok(Self::with_str(&string))
    }

    fn with_str<T: AsRef<str>>(s: T) -> Self {
        let (pattern, design) = s.as_ref().split_once("\n\n").expect("Invalid input format");

        let pattern = pattern
            .split(", ")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        let design = design
            .split('\n')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        Self {
            patterns: pattern,
            designs: design,
        }
    }
}

impl Solution for AoC2024_19 {
    fn part_one(&self) -> String {
        let mut total = 0usize;
        let mut memo = HashMap::new();
        for design in &self.designs {
            if path_count(design, &self.patterns, &mut memo) > 0 {
                total += 1;
            }
        }
        total.to_string()
    }

    fn part_two(&self) -> String {
        let mut total = 0usize;
        let mut memo = HashMap::new();
        for design in &self.designs {
            total += path_count(design, &self.patterns, &mut memo);
        }
        total.to_string()
    }

    fn description(&self) -> String {
        "Day 19: Linen Layout".to_string()
    }
}

fn path_count(design: &str, patterns: &[String], memo: &mut HashMap<String, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(val) = memo.get(design) {
        return *val;
    }

    let mut result = 0;
    for pattern in patterns.iter().filter(|p| design.starts_with(*p)) {
        result += path_count(&design[pattern.len()..], patterns, memo);
    }

    memo.insert(design.to_string(), result);
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_19_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.patterns.is_empty());
        assert!(!sol.designs.is_empty());
        assert_eq!(400, sol.designs.len());
        Ok(())
    }

    #[test]
    fn aoc2024_19_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "293");
        Ok(())
    }

    #[test]
    fn aoc2024_19_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "623924810770264");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2024_19> {
        AoC2024_19::new()
    }

    #[test]
    fn aoc2024_19_case_1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "6");
    }

    #[test]
    fn aoc2024_19_case_2() {
        let sol = make_test_solution();
        assert_eq!(sol.part_two(), "16");
    }

    fn make_test_solution() -> AoC2024_19 {
        let s = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        AoC2024_19::with_str(s)
    }
}
