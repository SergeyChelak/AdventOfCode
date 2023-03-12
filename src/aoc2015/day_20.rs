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

fn gifts(n: usize) -> usize {
    let mut count = 10usize;
    for i in 2..=n {
        if n % i == 0 {
            count += i * 10;
        }
    }
    count
}

impl Solution for AoC2015_20 {
    fn part_one(&self) -> String {
        let mut num = 1usize;
        loop {
            let amount = gifts(num);
            println!("# {num} gifts: {amount}");
            if amount > self.input {
                break;
            } else {
                let k = self.input / amount;
                if k > 10 {
                    num *= 2;
                } else {
                    num += 1;
                }
            }
        }
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
        assert_eq!(sol.part_one(), "786240");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}