use crate::solution::Solution;

use std::io;

pub struct AoC2016_14 {
    salt: String
}

impl AoC2016_14 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            salt: "ihaygndm".to_string()
        })
    }
}

impl Solution for AoC2016_14 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

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
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}