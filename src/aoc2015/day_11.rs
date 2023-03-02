use crate::solution::Solution;

use std::io;

fn increment(s: &str) -> String {
    let offset = 'a' as u8;
    let mut carry = 1;
    let mut chars = s.chars().collect::<Vec<char>>();
    for ch in chars.iter_mut().rev() {
        let val = *ch as u8 - offset + carry;
        carry = val / 26;
        *ch = (val % 26 + offset) as char;
        if carry == 0 {
            break;
        }
    }
    chars.into_iter().collect()
}

fn is_valid(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<char>>();
    let has_forbidden_chars = ['i', 'o', 'l']
        .iter()
        .fold(false, |acc, ch| acc || chars.contains(ch));
    if has_forbidden_chars {
        return false;
    }

    let mut has_sequence = false;
    for i in 0..chars.len() - 2 {
        let a = chars[i] as u8 as i32;
        let b = chars[i + 1] as u8 as i32;
        let c = chars[i + 2] as u8 as i32;
        if b - a == 1 && c - b == 1 {
            has_sequence = true;
            break;
        }
    }
    if !has_sequence {
        return false;
    }
    
    let mut prev = '\0';
    let mut count = 0;
    let mut is_group = false;
    for ch in chars {
        if ch == prev {
            if !is_group {
                count += 1;
            }
            is_group = true;
        } else {
            is_group = false;
        }
        prev = ch;
    }
    count > 1
}

fn next_valid_password(s: &str) -> String {
    let mut pass = s.to_string();
    loop {
        pass = increment(&pass);
        if is_valid(&pass) {
            break pass;
        }
    }
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
        next_valid_password(&self.input)
    }

    fn part_two(&self) -> String {
        next_valid_password(&next_valid_password(&self.input))
    }

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
        assert_eq!(sol.part_one(), "vzbxxyzz");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}