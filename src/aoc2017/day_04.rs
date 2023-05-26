use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

pub struct AoC2017_04 {
    phrases: Vec<String>,
}

impl AoC2017_04 {
    pub fn new() -> io::Result<Self> {
        let phrases = read_file_as_lines("input/aoc2017_04")?;
        Ok(Self { phrases })
    }
}

impl Solution for AoC2017_04 {
    fn part_one(&self) -> String {
        self.phrases
            .iter()
            .filter(|s| has_duplicates(s))
            .map(|_| 1)
            .sum::<usize>()
            .to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 4: High-Entropy Passphrases".to_string()
    }
}

fn has_duplicates(s: &str) -> bool {
    let words = s.split_whitespace().collect::<Vec<&str>>();
    let mut set: HashSet<&str> = HashSet::new();
    for word in words {
        if set.contains(word) {
            return false;
        }
        set.insert(word);
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_04_input_load_test() -> io::Result<()> {
        let sol = AoC2017_04::new()?;
        assert!(!sol.phrases.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_04_correctness() -> io::Result<()> {
        let sol = AoC2017_04::new()?;
        assert_eq!(sol.part_one(), "383");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
