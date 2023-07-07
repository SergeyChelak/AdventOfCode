use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2018_03 {
    // place required fields here
}

impl AoC2018_03 {
    pub fn new() -> io::Result<Self> {
        _ = read_file_as_lines("input/aoc2018_03")?;
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2018_03 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 3: No Matter How You Slice It".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_03_input_load_test() -> io::Result<()> {
        let sol = AoC2018_03::new()?;
        Ok(())
    }

    #[test]
    fn aoc2018_03_correctness() -> io::Result<()> {
        let sol = AoC2018_03::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}