use crate::solution::Solution;

use std::io;

pub struct AoC2017_17 {
    steps_count: usize,
}

impl AoC2017_17 {
    pub fn new() -> io::Result<Self> {
        Ok(Self { steps_count: 344 })
    }
}

impl Solution for AoC2017_17 {
    fn part_one(&self) -> String {
        let mut buffer = Vec::new();
        buffer.push(0);
        let mut pos = 0usize;
        for val in 1..=2017 {
            let index = (pos + self.steps_count) % buffer.len() + 1;
            buffer.insert(index, val);
            pos = index;
        }
        buffer[pos + 1].to_string()
    }

    fn part_two(&self) -> String {
        let mut len = 1;
        let mut pos = 0usize;
        let mut result = 0;
        for val in 1..=50000000 {
            let index = (pos + self.steps_count) % len + 1;
            pos = index;
            len += 1;
            if pos == 1 {
                result = val;
            }
        }
        result.to_string()
    }

    fn description(&self) -> String {
        "AoC 2017/Day 17: Spinlock".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_17_example1() {
        let sol = AoC2017_17 { steps_count: 3 };
        assert_eq!(sol.part_one(), "638");
    }

    #[test]
    fn aoc2017_17_correctness() -> io::Result<()> {
        let sol = AoC2017_17::new()?;
        assert_eq!(sol.part_one(), "996");
        assert_eq!(sol.part_two(), "1898341");
        Ok(())
    }
}
