use crate::solution::Solution;
use crate::utils::*;

use std::io;

fn profile(s: &str) -> (usize, usize) {
    let mut char_count = [0usize; 26];
    s.chars().map(|ch| ch as u8 - b'a').for_each(|byte| {
        char_count[byte as usize] += 1;
    });
    let mut twice_rep = 0;
    let mut triple_rep = 0;
    char_count.iter().for_each(|x| match x {
        2 => twice_rep = 1,
        3 => triple_rep = 1,
        _ => {}
    });
    (twice_rep, triple_rep)
}

pub struct AoC2018_02 {
    input: Vec<String>,
}

impl AoC2018_02 {
    pub fn new() -> io::Result<Self> {
        let input = read_file_as_lines("input/aoc2018_02")?;
        Ok(Self { input })
    }
}

impl Solution for AoC2018_02 {
    fn part_one(&self) -> String {
        let count = self
            .input
            .iter()
            .map(|s| profile(s))
            .fold((0usize, 0usize), |acc, x| (acc.0 + x.0, acc.1 + x.1));
        (count.0 * count.1).to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 2: Inventory Management System".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_02_input_load_test() -> io::Result<()> {
        let sol = AoC2018_02::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_02_correctness() -> io::Result<()> {
        let sol = AoC2018_02::new()?;
        assert_eq!(sol.part_one(), "5976");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
