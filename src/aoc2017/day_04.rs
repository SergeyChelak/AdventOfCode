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

    fn part_two(&self) -> String {
        self.phrases
            .iter()
            .filter(|s| has_no_anagrams(s))
            .map(|_| 1)
            .sum::<usize>()
            .to_string()
    }

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

fn has_no_anagrams(s: &str) -> bool {
    let words = s.split_whitespace().collect::<Vec<&str>>();
    let len = words.len();
    for i in 0..len - 1 {
        for j in i + 1..len {
            if is_anagram(words[i], words[j]) {
                return false;
            }
        }
    }
    true
}

fn is_anagram(w1: &str, w2: &str) -> bool {
    let f1 = footprint(w1);
    let f2 = footprint(w2);
    for (&a, &b) in f1.iter().zip(f2.iter()) {
        if a != b {
            return false;
        }
    }
    true
}

fn footprint(word: &str) -> [u32; 26] {
    let mut arr = [0; 26];
    word.chars().for_each(|ch| {
        let index = ch as u8 - b'a';
        arr[index as usize] += 1;
    });
    arr
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
        assert_eq!(sol.part_two(), "265");
        Ok(())
    }
}
