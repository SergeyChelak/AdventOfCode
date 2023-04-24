use crate::solution::Solution;

use std::io;

pub struct AoC2016_19 {
    elves: usize,
}

impl AoC2016_19 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            elves: 3012210,
        })
    }
}

impl Solution for AoC2016_19 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2016/Day 19: An Elephant Named Joseph".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_19_correctness() -> io::Result<()> {
        let sol = AoC2016_19::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}