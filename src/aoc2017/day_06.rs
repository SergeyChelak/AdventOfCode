use crate::solution::Solution;

use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;

pub struct AoC2017_06 {
    banks: Vec<usize>,
}

impl AoC2017_06 {
    pub fn new() -> io::Result<Self> {
        let banks = read_to_string("input/aoc2017_06")?
            .split_whitespace()
            .map(|s| s.parse::<usize>().expect("Int value is expected"))
            .collect();
        Ok(Self { banks })
    }

    fn redistribute_steps(&self) -> (usize, usize) {
        let mut map = HashMap::new();
        let mut banks = self.banks.clone();
        let len = banks.len();
        let mut steps = 0usize;
        loop {
            map.insert(banks.clone(), steps);
            steps += 1;
            let mut idx = 0;
            for i in 1..len {
                if banks[i] > banks[idx] {
                    idx = i;
                }
            }
            let val = banks[idx];
            banks[idx] = 0;
            for i in 1..=val {
                banks[(idx + i) % len] += 1;
            }
            if let Some(val) = map.get(&banks) {
                break (steps, steps - *val);
            }
        }
    }
}

impl Solution for AoC2017_06 {
    fn part_one(&self) -> String {
        self.redistribute_steps().0.to_string()
    }

    fn part_two(&self) -> String {
        self.redistribute_steps().1.to_string()
    }

    fn description(&self) -> String {
        "AoC 2017/Day 6: Memory Reallocation".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_06_input_load_test() -> io::Result<()> {
        let sol = AoC2017_06::new()?;
        assert!(!sol.banks.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_06_example1() {
        let sol = AoC2017_06 {
            banks: vec![0, 2, 7, 0],
        };
        assert_eq!(sol.part_one(), "5");
        assert_eq!(sol.part_two(), "4");
    }

    #[test]
    fn aoc2017_06_correctness() -> io::Result<()> {
        let sol = AoC2017_06::new()?;
        assert_eq!(sol.part_one(), "3156");
        assert_eq!(sol.part_two(), "1610");
        Ok(())
    }
}
