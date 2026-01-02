use crate::solution::Solution;

use std::io;

type Int = isize;

pub struct AoC2021_07 {
    input: Vec<Int>,
}

impl AoC2021_07 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2021_07")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        let input = data
            .trim()
            .split(',')
            .map(|s| s.parse::<Int>().expect("Invalid input"))
            .collect::<Vec<_>>();
        Self { input }
    }

    fn calculate(&self, consumption: impl Fn(Int, Int) -> usize) -> usize {
        let low = *self.input.iter().min().expect("Input shouldn't be empty");
        let high = *self.input.iter().max().expect("Input shouldn't be empty");

        let mut amount = usize::MAX;
        for pos in low..=high {
            let tmp = self
                .input
                .iter()
                .map(|x| consumption(*x, pos))
                .sum::<usize>();
            amount = amount.min(tmp);
        }
        amount
    }
}

impl Solution for AoC2021_07 {
    fn part_one(&self) -> String {
        self.calculate(|x, pos| x.abs_diff(pos)).to_string()
    }

    fn part_two(&self) -> String {
        self.calculate(|x, pos| {
            let n = x.abs_diff(pos);
            (n + 1) * n / 2
        })
        .to_string()
    }

    fn description(&self) -> String {
        "Day 7: The Treachery of Whales".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_07_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2021_07_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "344735");
        Ok(())
    }

    #[test]
    fn aoc2021_07_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "96798233");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2021_07> {
        AoC2021_07::new()
    }
}
