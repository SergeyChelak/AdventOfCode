use crate::solution::Solution;

use std::fs::read_to_string;
use std::io;

type Int = usize;

pub struct AoC2018_08 {
    input: Vec<Int>,
}

impl AoC2018_08 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2018_08")?
            .split_whitespace()
            .map(|x| x.parse::<Int>().expect("Non int value in the input"))
            .collect();

        Ok(Self { input })
    }
}

impl Solution for AoC2018_08 {
    fn part_one(&self) -> String {
        let mut sum = 0;
        sum_metadata(&self.input, &mut 0, &mut sum);
        sum.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 8: Memory Maneuver".to_string()
    }
}

fn sum_metadata(input: &[Int], pos: &mut usize, sum: &mut Int) {
    let child_count = input[*pos];
    let metadata_count = input[*pos + 1];
    *pos += 2;
    (0..child_count).for_each(|_| sum_metadata(input, pos, sum));
    (0..metadata_count).for_each(|_| {
        *sum += input[*pos];
        *pos += 1;
    });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_08_input_load_test() -> io::Result<()> {
        let sol = AoC2018_08::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_08_correctness() -> io::Result<()> {
        let sol = AoC2018_08::new()?;
        assert_eq!(sol.part_one(), "48260");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2018_08_example1() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"
            .split_whitespace()
            .map(|x| x.parse::<Int>().expect("Non int value in the input"))
            .collect::<Vec<Int>>();
        let mut sum = 0;
        sum_metadata(&input, &mut 0, &mut sum);
        assert_eq!(sum, 138);
    }
}
