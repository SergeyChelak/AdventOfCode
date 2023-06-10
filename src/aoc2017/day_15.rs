use crate::solution::Solution;

use std::io;

pub struct AoC2017_15 {
    //
}

impl AoC2017_15 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            //
        })
    }
}

impl Solution for AoC2017_15 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 15: Dueling Generators".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_15_correctness() -> io::Result<()> {
        let sol = AoC2017_15::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}