use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2023_15 {
    // place required fields here
}

impl AoC2023_15 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2023_15 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_15_input_load_test() -> io::Result<()> {
        let sol = AoC2023_15::new()?;
        Ok(())
    }

    #[test]
    fn aoc2023_15_correctness() -> io::Result<()> {
        let sol = AoC2023_15::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
