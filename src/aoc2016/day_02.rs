use crate::solution::Solution;
use crate::utils::*;

use std::io;

fn get_number(from: i32, s: &str) -> i32 {
    let from = from - 1;
    let pos = (from / 3, from % 3);
    let (r, c) = s.chars()
        .fold(pos, |(r, c), ch| {
            match ch {
                'L' => (r, 0.max(c - 1)),
                'R' => (r, 2.min(c + 1)),
                'U' => (0.max(r - 1), c),
                'D' => (2.min(r + 1), c),
                _ => panic!("Unexpected char {ch}")
            }
        });
    r * 3 + c + 1
}

pub struct AoC2016_02 {
    lines: Vec<String>
}

impl AoC2016_02 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2016_02")?;
        Ok(Self {
            lines
        })
    }
}

impl Solution for AoC2016_02 {
    fn part_one(&self) -> String {
        let mut prev = 5;
        let mut output = String::new();
        for line in &self.lines {
            let val = get_number(prev, line);
            prev = val;
            output.push_str(&val.to_string());
        }
        output
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2016/Day 2: Bathroom Security".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_02_input_load_test() -> io::Result<()> {
        let sol = AoC2016_02::new()?;
        assert!(sol.lines.len() > 0);
        Ok(())
    }

    #[test]
    fn aoc2016_02_correctness() -> io::Result<()> {
        let sol = AoC2016_02::new()?;
        assert_eq!(sol.part_one(), "53255");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2016_02_demo_case() {
        assert_eq!(get_number(5, "ULL"), 1);
        assert_eq!(get_number(1, "RRDDD"), 9);
        assert_eq!(get_number(9, "LURDL"), 8);
        assert_eq!(get_number(8, "UUUUD"), 5);
    }
}