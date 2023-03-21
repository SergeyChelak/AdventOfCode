use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2015_24 {
    input: Vec<i32>
}

impl AoC2015_24 {
    pub fn new() -> io::Result<Self> {
        let input = read_file_as_lines("input/aoc2015_24")?
            .iter()
            .map(|s| s.parse::<i32>().expect("Non numerical value found in input"))
            .collect::<Vec<i32>>();
        Ok(Self {
            input
        })
    }
}

impl Solution for AoC2015_24 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2015/Day 24: It Hangs in the Balance".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_24_input_load_test() -> io::Result<()> {
        let sol = AoC2015_24::new()?;
        assert_eq!(sol.input.len(), 29);
        Ok(())
    }

    #[test]
    fn aoc2015_24_correctness() -> io::Result<()> {
        let sol = AoC2015_24::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}