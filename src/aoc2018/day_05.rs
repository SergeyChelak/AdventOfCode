use crate::solution::Solution;
use crate::utils::*;

use std::io;

const REGISTER_DISTANCE: u8 = 32;

fn react(input: &[char]) -> Vec<char> {
    let mut polymers = input.to_owned();
    loop {
        let count = polymers.len();
        let mut buffer: Vec<char> = Vec::new();
        let mut idx = 0;
        while idx < count {
            let cur = polymers[idx];
            if idx < count - 1 {
                let next = polymers[idx + 1];
                if (cur as u8).abs_diff(next as u8) == REGISTER_DISTANCE {
                    idx += 2;
                    continue;
                };
            }
            buffer.push(cur);
            idx += 1;
        }
        if count == buffer.len() {
            break;
        }
        polymers = buffer;
    }
    polymers
}

pub struct AoC2018_05 {
    input: Vec<char>,
}

impl AoC2018_05 {
    pub fn new() -> io::Result<Self> {
        let input = read_file_as_chars("input/aoc2018_05")?
            .into_iter()
            .filter(|&x| x.is_alphabetic())
            .collect::<Vec<char>>();
        Ok(Self { input })
    }
}

impl Solution for AoC2018_05 {
    fn part_one(&self) -> String {
        react(&self.input).len().to_string()
    }

    fn part_two(&self) -> String {
        let mut shortest = usize::MAX;
        for ch in 'A'..='Z' {
            let polymer = self
                .input
                .iter()
                .filter(|&x| {
                    let dist = (ch as u8).abs_diff(*x as u8);
                    dist != 0 && dist != REGISTER_DISTANCE
                })
                .copied()
                .collect::<Vec<char>>();
            shortest = shortest.min(react(&polymer).len());
        }
        shortest.to_string()
    }

    fn description(&self) -> String {
        "AoC 2018/Day 5: Alchemical Reduction".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_05_input_load_test() -> io::Result<()> {
        let sol = AoC2018_05::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_05_correctness() -> io::Result<()> {
        let sol = AoC2018_05::new()?;
        assert_eq!(sol.part_one(), "9386");
        assert_eq!(sol.part_two(), "4876");
        Ok(())
    }

    #[test]
    fn aoc2018_05_example1() {
        let input = "dabAcCaCBAcCcaDA".chars().collect();
        let sol = AoC2018_05 { input };
        assert_eq!(sol.part_one(), "10")
    }
}
