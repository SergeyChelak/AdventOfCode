use crate::solution::Solution;

use std::io;

pub struct AoC2015_20 {
    input: usize,
}

impl AoC2015_20 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: 34000000
        })
    }
}

impl Solution for AoC2015_20 {
    fn part_one(&self) -> String {
        let mut num = 0usize;
        loop {
            let mut count = 0usize;
            num += 1;
            for i in 1..=num {
                if num % i == 0 {
                    count += i * 10;
                }
            }
            println!("{count}");
            if count >= self.input {
                break;
            }
        };
        num.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2015/Day 20: Infinite Elves and Infinite Houses".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_20_correctness() -> io::Result<()> {
        let sol = AoC2015_20::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}