use crate::solution::Solution;

use std::{fs::read_to_string, io};

const LAYER_WIDTH: usize = 25;
const LAYER_HEIGHT: usize = 6;

pub struct AoC2019_08 {
    input: Vec<char>,
}

impl AoC2019_08 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2019_08")?;
        Ok(Self::with_str(&input))
    }

    fn with_str(input: &str) -> Self {
        let input = input.trim().chars().collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2019_08 {
    fn part_one(&self) -> String {
        let mut zeros = usize::MAX;
        let mut value = 0usize;
        for arr in self.input.chunks(LAYER_HEIGHT * LAYER_WIDTH) {
            let mut usage = [0usize; 10];
            arr.iter()
                .map(|ch| ch.to_digit(10).expect("Non-digit value found"))
                .for_each(|digit| usage[digit as usize] += 1);
            if usage[0] < zeros {
                value = usage[1] * usage[2];
                zeros = usage[0];
            }
        }
        value.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 8: Space Image Format".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_08_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_08_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "2318");
        Ok(())
    }

    #[test]
    fn aoc2019_08_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_08> {
        AoC2019_08::new()
    }
}
