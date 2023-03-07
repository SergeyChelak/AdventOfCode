use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2015_17 {
    values: Vec<i32>,
}

impl AoC2015_17 {
    pub fn new() -> io::Result<Self> {
        let values = read_file_as_lines("input/aoc2015_17")?
            .iter()
            .map(|s| s.parse::<i32>().ok().expect("non integer value found"))
            .collect::<Vec<i32>>();
        Ok(Self {
            values
        })
    }
}

impl Solution for AoC2015_17 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2015/Day 17: No Such Thing as Too Much".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_17_input_load_test() -> io::Result<()> {
        let sol = AoC2015_17::new()?;
        assert_eq!(sol.values.len(), 20);
        Ok(())
    }

    #[test]
    fn aoc2015_17_correctness() -> io::Result<()> {
        let sol = AoC2015_17::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}