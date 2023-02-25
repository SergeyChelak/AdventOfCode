use crate::solution::Solution;
use crate::file_utils::*;

use std::collections::HashSet;
use std::io;

pub struct AoC2015_05 {
    input: Vec<String>,
}

impl AoC2015_05 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: read_file_as_lines("input/aoc2015_05")?
        })
    }

    fn is_good_str(s: &str) -> bool {
        for sub_str in vec!["ab", "cd", "pq", "xy"] {
            if s.contains(sub_str) {
                return false;
            }
        }
        let mut vovel_count = 0;
        let vovels: HashSet<char> = vec!['a', 'e', 'i', 'o', 'u'].into_iter().collect();
        let mut prev_char = '\0';
        let mut has_rep = false;
        for ch in s.chars() {
            if vovels.contains(&ch) {
                vovel_count += 1;
            }
            has_rep = has_rep || prev_char == ch;
            prev_char = ch;
        }
        vovel_count > 2 && has_rep
    }
}

impl Solution for AoC2015_05 {
    fn part_one(&self) -> String {
        self.input.iter()
            .filter(|v| Self::is_good_str(*v))
            .count()
            .to_string()
    }

    // fn part_two(&self) -> String {
    // }

    // fn description(&self) -> String {
    // "".to_string()
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_05_input_load_test() -> io::Result<()> {
        let sol = AoC2015_05::new()?;
        assert!(sol.input.len() > 0);
        Ok(())
    }

    #[test]
    fn aoc2015_05_is_good_str_test() {
        assert!(AoC2015_05::is_good_str("ugknbfddgicrmopn"));
        assert!(AoC2015_05::is_good_str("aaa"));
        assert!(!AoC2015_05::is_good_str("jchzalrnumimnmhp"));
        assert!(!AoC2015_05::is_good_str("haegwjzuvuyypxyu"));
        assert!(!AoC2015_05::is_good_str("dvszwmarrgswjxmb"));
    }

    #[test]
    fn aoc2015_05_correctness() -> io::Result<()> {
        assert_eq!(AoC2015_05::new()?.part_one(), "255");
        Ok(())
    }
}