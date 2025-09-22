use crate::solution::Solution;
// use crate::utils::*;

use std::io;

pub struct AoC2020_04 {
    //
}

impl AoC2020_04 {
    pub fn new() -> io::Result<Self> {
        // let lines = read_file_as_lines("input/aoc2020_04")?;
        // let input = std::fs::read_to_string("input/aoc2020_04")?;
        Ok(Self {
            // do init
        })
    }
}

impl Solution for AoC2020_04 {
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
    fn aoc2020_04_input_load_test() -> io::Result<()> {
        // let sol = make_solution()?;
        Ok(())
    }

    #[test]
    fn aoc2020_04_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "");
        Ok(())
    }

    #[test]
    fn aoc2020_04_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_04> {
        AoC2020_04::new()
    }
}
