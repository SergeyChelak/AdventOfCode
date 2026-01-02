use crate::solution::Solution;

use std::{collections::HashMap, io};

type Int = isize;

pub struct AoC2021_06 {
    input: Vec<Int>,
}

impl AoC2021_06 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2021_06")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        let input = data
            .trim()
            .split(',')
            .map(|s| s.parse::<Int>().expect("Input must be numbers"))
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2021_06 {
    fn part_one(&self) -> String {
        calculate(&self.input, 80).to_string()
    }

    fn part_two(&self) -> String {
        calculate(&self.input, 256).to_string()
    }

    fn description(&self) -> String {
        "Day 6: Lanternfish".to_string()
    }
}

fn calculate(input: &[Int], days: Int) -> Int {
    let mut cache = HashMap::<Int, Int>::new();
    let mut acc = 0;
    for counter in input.iter() {
        if let Some(count) = cache.get(counter) {
            acc += *count;
            continue;
        }
        // +1 means including self
        let amount = count(*counter, days, &mut cache) + 1;
        cache.insert(*counter, amount);
        acc += amount;
    }
    acc
}

fn count(val: Int, days: Int, memo: &mut HashMap<Int, Int>) -> Int {
    if let Some(cached) = memo.get(&val) {
        return *cached;
    }
    if days < val {
        return 0;
    }
    let mut acc = 0;
    for day in (val + 1..=days).step_by(7) {
        acc += 1 + count(day + 8, days, memo);
    }
    memo.insert(val, acc);
    acc
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_06_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2021_06_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "374927");
        Ok(())
    }

    #[test]
    fn aoc2021_06_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "1687617803407");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2021_06> {
        AoC2021_06::new()
    }
}
