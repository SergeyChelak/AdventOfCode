use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2024_04 {
    // place required fields here
}

impl AoC2024_04 {
    pub fn new() -> io::Result<Self> {
        let _ = "input/aoc2024_04";
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2024_04 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_04_input_load_test() -> io::Result<()> {
        let sol = AoC2024_04::new()?;
        Ok(())
    }

    #[test]
    fn aoc2024_04_correctness() -> io::Result<()> {
        let sol = AoC2024_04::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
