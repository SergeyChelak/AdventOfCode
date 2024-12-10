use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2024_11 {
    // place required fields here
}

impl AoC2024_11 {
    pub fn new() -> io::Result<Self> {
        let _ = "input/aoc2024_11";
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2024_11 {
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
    fn aoc2024_11_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        Ok(())
    }

    #[test]
    fn aoc2024_11_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "");
        Ok(())
    }

    #[test]
    fn aoc2024_11_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2024_11> {
        AoC2024_11::new()
    }
}
