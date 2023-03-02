use crate::solution::Solution;

use std::io;

fn increment(s: &str) -> String {
    todo!()
}

fn is_valid(s: &str) -> bool {
    todo!()
}

pub struct AoC2015_11 {
    input: String
}

impl AoC2015_11 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: "vzbxkghb".to_string()
        })
    }
}

impl Solution for AoC2015_11 {
    fn part_one(&self) -> String {
        let mut pass = self.input.clone();
        loop {
            pass = increment(&pass);
            if is_valid(&pass) {
                break pass;
            }
        }
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
    	"AoC 2015/Day 11: Corporate Policy".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_11_increment() {
        assert_eq!(increment("xx"), "xy");
        assert_eq!(increment("xy"), "xz");
        assert_eq!(increment("xz"), "ya");
        assert_eq!(increment("ya"), "yb");
    }

    #[test]
    fn aoc2015_11_is_valid_password() {
        assert!(!is_valid("hijklmmn"));
        assert!(!is_valid("abbceffg"));
        assert!(!is_valid("abbcegjk"));
        assert!(is_valid("abcdffaa"));
        assert!(is_valid("ghjaabcc"));
    }

    #[test]
    fn aoc2015_11_correctness() -> io::Result<()> {
        let sol = AoC2015_11::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}