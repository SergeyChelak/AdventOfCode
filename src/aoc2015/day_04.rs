use crate::solution::Solution;

use std::io;
use md5;

pub struct AoC2015_04 {
    input: String,
}

impl AoC2015_04 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: "bgvyzdsv".to_string()
        })
    }

    fn search(&self, prefix: &str) -> String {        
        for i in 1..usize::MAX {
            let input = format!("{}{i}", self.input);
            let hash = format!("{:x}", md5::compute(input));
            if hash.starts_with(prefix) {
                return i.to_string()
            }
        }
        "Not found".to_string()
    }
}

impl Solution for AoC2015_04 {
    fn part_one(&self) -> String {
        self.search("00000")
    }

    fn part_two(&self) -> String {
        self.search("000000")
    }

    fn description(&self) -> String {
        "AoC 2015/Day 4: The Ideal Stocking Stuffer".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_04_correctness() -> io::Result<()> {
        assert_eq!(AoC2015_04::new()?.part_one(), "254575");
        assert_eq!(AoC2015_04::new()?.part_two(), "1038736");
        Ok(())
    }
}