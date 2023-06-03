use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2017_09 {
    chars: Vec<char>,
}

impl AoC2017_09 {
    pub fn new() -> io::Result<Self> {
        let chars = read_file_as_bytes("input/aoc2017_09")?
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<char>>();
        Ok(Self { chars })
    }
}

impl Solution for AoC2017_09 {
    fn part_one(&self) -> String {
        scores(&self.chars).0.to_string()
    }

    fn part_two(&self) -> String {
        scores(&self.chars).1.to_string()
    }

    fn description(&self) -> String {
        "AoC 2017/Day 9: Stream Processing".to_string()
    }
}

fn scores(chars: &[char]) -> (usize, usize) {
    let mut scores = 0;
    let mut garbage_len = 0;
    let mut depth = 0;
    // flags
    let mut is_garbage = false;
    let mut is_skip_next = false;
    for ch in chars {
        if is_skip_next {
            is_skip_next = false;
            continue;
        }
        match *ch {
            '!' => is_skip_next = true,
            '>' if is_garbage => is_garbage = false,
            '<' => {
                if is_garbage {
                    garbage_len += 1;
                } else {
                    is_garbage = true;
                }
            }
            '{' if !is_garbage => depth += 1,
            '}' if !is_garbage => {
                scores += depth;
                depth -= 1;
            }
            _ => {
                if is_garbage {
                    garbage_len += 1;
                }
            }
        };
    }
    (scores, garbage_len)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_09_input_load_test() -> io::Result<()> {
        let sol = AoC2017_09::new()?;
        assert!(!sol.chars.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_09_example1() {
        assert_eq!(scores_from_str("{}"), 1);
        assert_eq!(scores_from_str("{{{}}}"), 6);
        assert_eq!(scores_from_str("{{},{}}"), 5);
        assert_eq!(scores_from_str("{{{},{},{{}}}}"), 16);
        assert_eq!(scores_from_str("{<a>,<a>,<a>,<a>}"), 1);
        assert_eq!(scores_from_str("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
        assert_eq!(scores_from_str("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
        assert_eq!(scores_from_str("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }

    #[test]
    fn aoc2017_09_example2() {
        assert_eq!(garbage_len_from_str("<>,"), 0);
        assert_eq!(garbage_len_from_str("<random characters>"), 17);
        assert_eq!(garbage_len_from_str("<<<<>"), 3);
        assert_eq!(garbage_len_from_str("<{!>}>"), 2);
        assert_eq!(garbage_len_from_str("<!!>"), 0);
        assert_eq!(garbage_len_from_str("<!!!>>"), 0);
        assert_eq!(garbage_len_from_str("<{o\"i!a,<{i<a>"), 10);
    }

    #[test]
    fn aoc2017_09_correctness() -> io::Result<()> {
        let sol = AoC2017_09::new()?;
        assert_eq!(sol.part_one(), "7616");
        assert_eq!(sol.part_two(), "3838");
        Ok(())
    }

    fn scores_from_str(s: &str) -> usize {
        let chars = s.chars().collect::<Vec<char>>();
        scores(&chars).0
    }

    fn garbage_len_from_str(s: &str) -> usize {
        let chars = s.chars().collect::<Vec<char>>();
        scores(&chars).1
    }
}
