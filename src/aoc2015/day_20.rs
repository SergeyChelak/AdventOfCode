use crate::solution::Solution;

use std::io;

pub struct AoC2015_20 {
    input: usize,
}

impl AoC2015_20 {
    pub fn new() -> io::Result<Self> {
        Ok(Self { input: 34000000 })
    }
}

impl Solution for AoC2015_20 {
    fn part_one(&self) -> String {
        let mut max_houses = self.input / 10;
        let mut gifts = vec![0usize; max_houses];
        let mut elf = 1usize;
        while elf < max_houses {
            for i in (elf..max_houses).step_by(elf) {
                gifts[i] += elf * 10;
                if gifts[i] > self.input {
                    max_houses = i;
                    break;
                }
            }
            elf += 1;
        }
        gifts
            .iter()
            .position(|&val| val >= self.input)
            .unwrap()
            .to_string()
    }

    fn part_two(&self) -> String {
        let mut max_houses = self.input / 10;
        let mut gifts = vec![0usize; max_houses];
        let mut elf = 1usize;
        while elf < max_houses {
            let max = max_houses.min(50 * elf);
            for i in (elf..max).step_by(elf) {
                gifts[i] += elf * 11;
                if gifts[i] > self.input {
                    max_houses = i;
                    break;
                }
            }
            elf += 1;
        }
        gifts
            .iter()
            .position(|&val| val >= self.input)
            .unwrap()
            .to_string()
    }

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
        assert_eq!(sol.part_two(), "831600");
        Ok(())
    }
}
