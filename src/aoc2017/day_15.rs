use crate::solution::Solution;

use std::io;

type Value = u64;

struct Generator {
    value: Value,
    factor: Value,
    filter: Value,
}

impl Generator {
    fn gen_a(value: Value) -> Self {
        Self {
            value,
            factor: 16807,
            filter: 4,
        }
    }

    fn gen_b(value: Value) -> Self {
        Self {
            value,
            factor: 48271,
            filter: 8,
        }
    }

    fn next(&mut self) {
        self.value = self.value * self.factor % 2147483647;
    }

    fn next_filtered(&mut self) {
        loop {
            self.next();
            if self.value % self.filter == 0 {
                break;
            }
        }
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

    fn count_matches(&self, max_pairs: usize, next: &impl Fn(&mut Generator)) -> i32 {
        let mut gen_a = Generator::gen_a(self.start_a);
        let mut gen_b = Generator::gen_b(self.start_b);
        let mut count = 0;
        for _ in 0..max_pairs {
            let val_a = gen_a.value & 0xffff;
            let val_b = gen_b.value & 0xffff;
            if val_a == val_b {
                count += 1;
            }
            next(&mut gen_a);
            next(&mut gen_b);
        }
        count
    }
}

impl Solution for AoC2017_15 {
    fn part_one(&self) -> String {
        self.count_matches(40_000_000, &|gen: &mut Generator| gen.next())
            .to_string()
    }

    fn part_two(&self) -> String {
        self.count_matches(5_000_000, &|gen: &mut Generator| gen.next_filtered())
            .to_string()
    }

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
        assert_eq!(sol.part_two(), "328");
        Ok(())
    }
}
