use crate::solution::Solution;

use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;

pub struct AoC2024_19 {
    pattern: Vec<String>,
    design: Vec<String>,
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

        Self { pattern, design }
    }
}

impl Solution for AoC2024_19 {
    fn part_one(&self) -> String {
        let mut available = HashMap::new();
        for pattern in &self.pattern {
            available.insert(pattern.clone(), true);
        }

        let mut total = 0usize;
        for design in &self.design {
            if is_possible(design, &mut available) {
                total += 1;
            }
        }

        total.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 19: Linen Layout".to_string()
    }
}

fn is_possible(design: &str, available: &mut HashMap<String, bool>) -> bool {
    if let Some(val) = available.get(design) {
        return *val;
    }

    for i in 1..design.len() {
        let prefix = &design[..i];
        let suffix = &design[i..];
        // println!("p: {}, s: {}", prefix, suffix);
        if is_possible(prefix, available) && is_possible(suffix, available) {
            available.insert(design.to_string(), true);
            return true;
        }
    }

    available.insert(design.to_string(), false);
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_19_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.pattern.is_empty());
        assert!(!sol.design.is_empty());
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
        assert_eq!(sol.part_two(), "");
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
