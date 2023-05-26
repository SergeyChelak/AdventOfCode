use crate::solution::Solution;

use std::io;

pub struct AoC2017_03 {
    input: u32,
}

impl AoC2017_03 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: 277678
        })
    }
}

impl Solution for AoC2017_03 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 3: Spiral Memory".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_03_correctness() -> io::Result<()> {
        let sol = AoC2017_03::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}