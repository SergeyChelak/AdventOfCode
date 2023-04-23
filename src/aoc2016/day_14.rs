use crate::solution::Solution;

use std::io;

struct HashInfo {
    symbol: char,
    reps: usize,
    index: usize,
}

fn hash(salt: &str, index: usize) -> String {
    let input = format!("{}{index}", salt);
    format!("{:x}", md5::compute(input))
}

fn stretched_hash(salt: &str, index: usize) -> String {
    let mut input = hash(salt, index);
    for _ in 0..2016 {
        input = format!("{:x}", md5::compute(input));
    }
    input
}

type FnHasher = dyn Fn(&str, usize) -> String;

struct HashGenerator {
    salt: String,
    hasher: &'static FnHasher,
    _index: usize,
}

impl HashGenerator {
    fn with_salt(salt: &str, hasher: &'static FnHasher) -> Self {
        Self {
            salt: salt.to_string(),
            hasher,
            _index: 0,
        }
    }
}

impl Iterator for HashGenerator {
    type Item = HashInfo;

    fn next(&mut self) -> Option<Self::Item> {
        for index in self._index..usize::MAX {
            let hash = (self.hasher)(&self.salt, index);
            let (reps, ch) = find_char_sequence(&hash);
            if reps > 2 {
                self._index = index + 1;
                return Some(HashInfo {
                    symbol: ch,
                    reps,
                    index,
                });
            }
        }
        None
    }
}

fn find_char_sequence(s: &str) -> (usize, char) {
    let mut count = 0;
    let mut prev = '\0';
    let mut res_count = 0;
    let mut res_char = '\0';
    for ch in s.chars() {
        if ch == prev {
            count += 1;
            if res_count < count {
                res_count = count;
                res_char = prev;
            }
        } else {
            // Only consider the first triplet in a hash
            if res_count == 3 {
                break;
            }
            count = 1;
        }
        prev = ch;
    }
    (res_count, res_char)
}

fn find_key_index(salt: &str, hasher: &'static FnHasher) -> Option<usize> {
    let distance = 1001;
    let mut generator = HashGenerator::with_salt(salt, hasher);
    let mut hashes: Vec<HashInfo> = Vec::with_capacity(2 * distance);
    if let Some(info) = generator.next() {
        hashes.push(info);
    }
    // look for the pairs
    let mut count = 0usize;
    let mut number = 0usize;
    loop {
        let symbol = hashes[number].symbol;
        let index = hashes[number].index;
        // fill hashes
        for info in generator.by_ref() {
            let info_idx = info.index;
            hashes.push(info);
            if info_idx - index > distance {
                break;
            }
        }
        let current_hash = &hashes[number];
        for info in hashes.iter().skip(number + 1) {
            assert_ne!(info.index, current_hash.index);
            if info.symbol == symbol && info.reps > 4 && info.index - index <= distance {
                count += 1;
                break;
            }
        }
        if count == 64 {
            break Some(current_hash.index);
        }
        number += 1;
    }
}

pub struct AoC2016_14 {
    salt: String,
}

impl AoC2016_14 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            salt: "ihaygndm".to_string(),
        })
    }
}

impl Solution for AoC2016_14 {
    fn part_one(&self) -> String {
        find_key_index(&self.salt, &hash)
            .expect("Correspoding hash index not found")
            .to_string()
    }

    fn part_two(&self) -> String {
        find_key_index(&self.salt, &stretched_hash)
            .expect("Correspoding hash index not found")
            .to_string()
    }

    fn description(&self) -> String {
        "AoC 2016/Day 14: One-Time Pad".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_14_correctness() -> io::Result<()> {
        let sol = AoC2016_14::new()?;
        assert_eq!(sol.part_one(), "15035");
        assert_eq!(sol.part_two(), "19968");
        Ok(())
    }

    #[test]
    fn aoc2016_14_find_char_sequence() {
        assert_eq!(find_char_sequence("cc38887a5"), (3, '8'));
        assert_eq!(find_char_sequence("abbccc"), (3, 'c'));
        assert_eq!(find_char_sequence("aaaabbbccd"), (4, 'a'));
    }

    #[test]
    fn aoc2016_14_example() {
        let index = find_key_index("abc", &hash);
        assert_eq!(index.unwrap(), 22728);
    }

    #[test]
    fn aoc2016_14_calc_stretched_hash() {
        let hash = stretched_hash("abc", 0);
        assert_eq!(hash, "a107ff634856bb300138cac6568c0f24");
    }
}
