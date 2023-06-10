use crate::solution::Solution;

use super::knot_hash::*;
use std::{fs::read_to_string, io};

pub struct AoC2017_10 {
    input: String,
}

impl AoC2017_10 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2017_10")?.trim().to_string();
        Ok(Self { input })
    }
}

impl Solution for AoC2017_10 {
    fn part_one(&self) -> String {
        let inp = self
            .input
            .split(',')
            .map(|s| {
                s.parse::<u8>()
                    .expect("Integer values expected in the input")
            })
            .collect::<Vec<u8>>();
        let arr = tie_knot(256, 1, &inp);
        (arr[0] as u32 * arr[1] as u32).to_string()
    }

    fn part_two(&self) -> String {
        self.input.knot_hash()
    }

    fn description(&self) -> String {
        "AoC 2017/Day 10: Knot Hash".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_10_input_load_test() -> io::Result<()> {
        let sol = AoC2017_10::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_10_correctness() -> io::Result<()> {
        let sol = AoC2017_10::new()?;
        assert_eq!(sol.part_one(), "54675");
        assert_eq!(sol.part_two(), "a7af2706aa9a09cf5d848c1e6605dd2a");
        Ok(())
    }
}
