use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2015_08 {
    input: Vec<String>,
}

impl AoC2015_08 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: read_file_as_lines("input/aoc2015_08")?
        })
    }

    fn unescaped_len(s: &str) -> usize {
        let chars = s.chars().collect::<Vec<char>>();
        let mut len = chars.len();
        let count = len - 1;
        assert!(len > 1, "String is too short");
        if chars[0] == '\"' && chars[len-1] == '\"' {
            len -= 2;
        } else {
            panic!("String isn't wrapped with quotes")
        }
        let mut pos = 1usize;
        while pos < count {
            let ch = chars[pos];
            if ch == '\\' {
                if chars[pos + 1] == 'x' {
                    len -= 3;
                    pos += 3;
                } else {
                    len -= 1;
                    pos += 2;
                }
            } else {
                pos += 1;
            }
        }
        len
    }

    fn escaped_len(s: &str) -> usize {
        s.chars().fold(2usize, |acc, ch| {
            acc + match ch {
                '\"' | '\\' => 2,
                '\'' => 3,
                _ => 1,
            }
        })
    }
}

impl Solution for AoC2015_08 {
    fn part_one(&self) -> String {
        self.input.iter()
            .map(|s| s.len() - Self::unescaped_len(s))
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.input.iter()
            .map(|s| Self::escaped_len(s) - s.len())
            .sum::<usize>()
            .to_string()
    }

    fn description(&self) -> String {
    	"AoC 2015/Day 8: Matchsticks".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_08_input_load_test() -> io::Result<()> {
        let sol = AoC2015_08::new()?;
        assert_eq!(sol.input.len(), 300);
        Ok(())
    }

    #[test]
    fn aoc2015_08_unescaped_len() {
        assert_eq!(AoC2015_08::unescaped_len("\"\""), 0);
        assert_eq!(AoC2015_08::unescaped_len("\"abc\""), 3);
        assert_eq!(AoC2015_08::unescaped_len("\"aaa\\\"aaa\""), 7);
        assert_eq!(AoC2015_08::unescaped_len("\"\\x27\""), 1);
    }

    #[test]
    fn aoc2015_08_escaped_len() {
        assert_eq!(AoC2015_08::escaped_len("\"\""), 6);
        assert_eq!(AoC2015_08::escaped_len("\"abc\""), 9);
        assert_eq!(AoC2015_08::escaped_len("\"aaa\\\"aaa\""), 16);
        assert_eq!(AoC2015_08::escaped_len("\"\\x27\""), 11);
    }

    #[test]
    fn aoc2015_08_correctness() -> io::Result<()> {
        let sol = AoC2015_08::new()?;
        assert_eq!(sol.part_one(), "1350");
        assert_eq!(sol.part_two(), "2085");
        Ok(())
    }
}