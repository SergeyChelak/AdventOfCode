use crate::solution::Solution;

use std::io;

pub struct AoC2017_14 {
    input: String
}

impl AoC2017_14 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: "oundnydw".to_string()
        })
    }
}

impl Solution for AoC2017_14 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 14: Disk Defragmentation".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_14_correctness() -> io::Result<()> {
        let sol = AoC2017_14::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}