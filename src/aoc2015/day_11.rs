use crate::solution::Solution;

use std::io;

pub struct AoC2015_11 {
    input: String
}

impl AoC2015_11 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: "vzbxkghb".to_string()
        })
    }
}

impl Solution for AoC2015_11 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
    	"AoC 2015/Day 11: Corporate Policy".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_11_correctness() -> io::Result<()> {
        let sol = AoC2015_11::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}