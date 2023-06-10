use crate::solution::Solution;

use std::io;

type Value = u64;

struct Generator {
    value: Value,
    factor: Value,
}

impl Generator {
    fn gen_a(value: Value) -> Self {
        Self {
            value,
            factor: 16807,
        }
    }

    fn gen_b(value: Value) -> Self {
        Self {
            value,
            factor: 48271,
        }
    }

    fn next(&mut self) {
        self.value = self.value * self.factor % 2147483647;
    }
}

pub struct AoC2017_15 {
    start_a: Value,
    start_b: Value,
}

impl AoC2017_15 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            start_a: 703,
            start_b: 516,
        })
    }
}

impl Solution for AoC2017_15 {
    fn part_one(&self) -> String {
        let mut gen_a = Generator::gen_a(self.start_a);
        let mut gen_b = Generator::gen_b(self.start_b);
        let mut count = 0;
        for _ in 0..40_000_000 {
            let val_a = gen_a.value & 0xffff;
            let val_b = gen_b.value & 0xffff;
            if val_a == val_b {
                count += 1;
            }
            gen_a.next();
            gen_b.next();
        }
        count.to_string()
    }

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
    fn aoc2017_15_example1() {
        let sol = AoC2017_15 {
            start_a: 65,
            start_b: 8921,
        };
        assert_eq!(sol.part_one(), "588")
    }

    #[test]
    fn aoc2017_15_correctness() -> io::Result<()> {
        let sol = AoC2017_15::new()?;
        assert_eq!(sol.part_one(), "594");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
