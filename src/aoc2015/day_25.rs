use crate::solution::Solution;

use std::io;

pub struct AoC2015_25 {
    row: usize,
    col: usize,
}

impl AoC2015_25 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            row: 2947,
            col: 3029,
        })
    }
}

impl Solution for AoC2015_25 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2015/Day 25: Let It Snow".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_25_correctness() -> io::Result<()> {
        let sol = AoC2015_25::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}