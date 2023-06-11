use crate::solution::Solution;

use std::io;

pub struct AoC2017_17 {
    steps_count: usize,
}

impl AoC2017_17 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            steps_count: 344
        })
    }
}

impl Solution for AoC2017_17 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 17: Spinlock".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_17_correctness() -> io::Result<()> {
        let sol = AoC2017_17::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}