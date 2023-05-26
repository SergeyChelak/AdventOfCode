use crate::solution::Solution;

use std::fs::read_to_string;
use std::io;

pub struct AoC2017_01 {
    digits: Vec<u32>,
}

impl AoC2017_01 {
    pub fn new() -> io::Result<Self> {
        let str = read_to_string("input/aoc2017_01")?;
        Ok(Self::with_str(&str))
    }

    fn with_str(s: &str) -> Self {
        let digits = s
            .chars()
            .filter_map(|ch| ch.to_digit(10))
            .collect::<Vec<u32>>();
        Self { digits }
    }
}

impl Solution for AoC2017_01 {
    fn part_one(&self) -> String {
        let mut sum = 0usize;
        let len = self.digits.len();
        for (i, _) in self.digits.iter().enumerate() {
            if self.digits[i] == self.digits[(i + 1) % len] {
                sum += self.digits[i] as usize;
            }
        }
        sum.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 1: Inverse Captcha".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_01_input_load_test() -> io::Result<()> {
        let sol = AoC2017_01::new()?;
        assert!(!sol.digits.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_01_part1() {
        let sol = AoC2017_01::with_str("1122");
        assert_eq!(sol.part_one(), "3");
        let sol = AoC2017_01::with_str("1111");
        assert_eq!(sol.part_one(), "4");
        let sol = AoC2017_01::with_str("1234");
        assert_eq!(sol.part_one(), "0");
        let sol = AoC2017_01::with_str("91212129");
        assert_eq!(sol.part_one(), "9");
    }

    #[test]
    fn aoc2017_01_correctness() -> io::Result<()> {
        let sol = AoC2017_01::new()?;
        assert_eq!(sol.part_one(), "1047");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
