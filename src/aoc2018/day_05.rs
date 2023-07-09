use crate::solution::Solution;
use crate::utils::*;

use std::io;

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
        let mut polymers = self.input.clone();
        loop {
            let count = polymers.len();
            let mut buffer: Vec<char> = Vec::new();
            let mut idx = 0;
            while idx < count {
                let cur = polymers[idx];
                if idx < count - 1 {
                    let next = polymers[idx + 1];
                    if (cur as u8).abs_diff(next as u8) == 32 {
                        idx += 2;
                        continue;
                    };
                }
                buffer.push(cur);
                idx += 1;
            }
            if count == buffer.len() {
                break count;
            }
            polymers = buffer;
        }
        .to_string()
    }

    // fn part_two(&self) -> String {
    // }

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
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2018_05_example1() {
        let input = "dabAcCaCBAcCcaDA".chars().collect();
        let sol = AoC2018_05 { input };
        assert_eq!(sol.part_one(), "10")
    }
}
