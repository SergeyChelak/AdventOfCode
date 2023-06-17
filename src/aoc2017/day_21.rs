use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2017_21 {
    // place required fields here
}

impl AoC2017_21 {
    pub fn new() -> io::Result<Self> {
        _ = read_file_as_lines("input/aoc2017_20")?;
        Ok(Self {
            // initialize solution
        })
    }

    fn parse_rule(s: &str) {
        let (inp, out) = s.split_once(" => ").expect("Invalid pattern format");
    }
}

impl Solution for AoC2017_21 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 21: Fractal Art".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_21_input_load_test() -> io::Result<()> {
        let sol = AoC2017_21::new()?;
        Ok(())
    }

    #[test]
    fn aoc2017_21_correctness() -> io::Result<()> {
        let sol = AoC2017_21::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
