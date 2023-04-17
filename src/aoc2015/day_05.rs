use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

pub struct AoC2015_05 {
    input: Vec<String>,
}

impl AoC2015_05 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: read_file_as_lines("input/aoc2015_05")?,
        })
    }

    fn is_nice_str_pt1(s: &str) -> bool {
        for sub_str in &["ab", "cd", "pq", "xy"] {
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

    fn is_nice_str_pt2(s: &str) -> bool {
        let chars: Vec<char> = s.chars().collect();
        {
            let mut crit_2 = false;
            for i in 0..(chars.len() - 2) {
                crit_2 = crit_2 || chars[i] == chars[i + 2];
                if crit_2 {
                    break;
                }
            }
            if !crit_2 {
                return false;
            }
        }
        {
            for i in 0..(chars.len() - 2) {
                let pair = &s[i..=(i + 1)];
                let substr = &s[(i + 2)..];
                if substr.contains(pair) {
                    return true;
                }
            }
        }
        false
    }

    fn count_nice_strings<C>(&self, criteria: C) -> String
    where
        C: Fn(&str) -> bool,
    {
        self.input
            .iter()
            .filter(|v| criteria(v))
            .count()
            .to_string()
    }
}

impl Solution for AoC2015_05 {
    fn part_one(&self) -> String {
        self.count_nice_strings(Self::is_nice_str_pt1)
    }

    fn part_two(&self) -> String {
        self.count_nice_strings(Self::is_nice_str_pt2)
    }

    fn description(&self) -> String {
        "AoC 2015/Day 5: Doesn't He Have Intern-Elves For This?".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_05_input_load_test() -> io::Result<()> {
        let sol = AoC2015_05::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2015_05_is_nice_str_p1_test() {
        assert!(AoC2015_05::is_nice_str_pt1("ugknbfddgicrmopn"));
        assert!(AoC2015_05::is_nice_str_pt1("aaa"));
        assert!(!AoC2015_05::is_nice_str_pt1("jchzalrnumimnmhp"));
        assert!(!AoC2015_05::is_nice_str_pt1("haegwjzuvuyypxyu"));
        assert!(!AoC2015_05::is_nice_str_pt1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn aoc2015_05_is_nice_str_p2_test() {
        assert!(AoC2015_05::is_nice_str_pt2("qjhvhtzxzqqjkmpb"));
        assert!(AoC2015_05::is_nice_str_pt2("xxyxx"));
        assert!(!AoC2015_05::is_nice_str_pt2("uurcxstgmygtbstg"));
        assert!(!AoC2015_05::is_nice_str_pt2("ieodomkazucvgmuy"));
    }

    #[test]
    fn aoc2015_05_correctness() -> io::Result<()> {
        let sol = AoC2015_05::new()?;
        assert_eq!(sol.part_one(), "255");
        assert_eq!(sol.part_two(), "55");
        Ok(())
    }
}
