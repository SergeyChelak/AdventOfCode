use regex::Regex;

use crate::solution::Solution;

use std::fs::read_to_string;
use std::io;

pub struct AoC2024_03 {
    input: String,
}

impl AoC2024_03 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2024_03")?;
        Ok(Self { input })
    }
}

impl Solution for AoC2024_03 {
    fn part_one(&self) -> String {
        Regex::new(r"mul\(\d*,\d*\)")
            .expect("regex format isn't valid")
            .find_iter(&self.input)
            .map(|s| mul(s.as_str()))
            .sum::<i32>()
            .to_string()
    }

    fn part_two(&self) -> String {
        Regex::new(r"mul\(\d*,\d*\)|do\(\)|don't\(\)")
            .expect("regex format isn't valid")
            .find_iter(&self.input)
            .map(|s| s.as_str())
            .fold((0, true), |acc, value| {
                let sum = acc.0;
                let doable = acc.1;
                if value == "don't()" {
                    return (sum, false);
                }
                if value == "do()" {
                    return (sum, true);
                }
                if doable {
                    return (sum + mul(value), true);
                }
                acc
            })
            .0
            .to_string()
    }

    fn description(&self) -> String {
        "2024/Day 3: Mull It Over".to_string()
    }
}

fn mul(s: &str) -> i32 {
    let (a, b) = s[4..s.len() - 1]
        .split_once(',')
        .expect("Invalid instruction {s}");
    let a = a.parse::<i32>().expect("first argument isn't numeric");
    let b = b.parse::<i32>().expect("second argument isn't numeric");
    a * b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_03_input_load_test() -> io::Result<()> {
        let sol = AoC2024_03::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2024_03_correctness() -> io::Result<()> {
        let sol = AoC2024_03::new()?;
        assert_eq!(sol.part_one(), "171183089");
        assert_eq!(sol.part_two(), "63866497");
        Ok(())
    }

    #[test]
    fn aoc2024_03_case_1() {
        let sol = AoC2024_03 {
            input: r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
                .to_string(),
        };
        assert_eq!(sol.part_one(), "161");
    }

    #[test]
    fn aoc2024_03_case_2() {
        let sol = AoC2024_03 {
            input: r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
                .to_string(),
        };
        assert_eq!(sol.part_two(), "48");
    }
}
