use crate::solution::Solution;
use crate::utils::*;

use std::io;

struct LevelParams(usize, usize);

pub struct AoC2017_13 {
    input: Vec<LevelParams>,
}

impl AoC2017_13 {
    pub fn new() -> io::Result<Self> {
        let input = read_file_as_lines("input/aoc2017_13")?
            .iter()
            .map(|s| {
                let (depth, range) = s
                    .split_once(": ")
                    .expect("Input strings should be separated with colon");
                let depth = depth.parse::<usize>().expect("Depth should be integer");
                let range = range.parse::<usize>().expect("Range should be integer");
                LevelParams(depth, range)
            })
            .collect::<Vec<LevelParams>>();
        Ok(Self { input })
    }
}

impl Solution for AoC2017_13 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 13: Packet Scanners".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_13_input_load_test() -> io::Result<()> {
        let sol = AoC2017_13::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_13_correctness() -> io::Result<()> {
        let sol = AoC2017_13::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
