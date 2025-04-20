use crate::{solution::Solution, utils::Point2d};

use std::{collections::HashMap, fs::read_to_string, io};

use super::intcode_computer::*;

const TILE_BLOCK: Int = 2;

type Pixel = Point2d<Int>;

pub struct AoC2019_13 {
    input: Memory,
}

impl AoC2019_13 {
    pub fn new() -> io::Result<Self> {
        let line = read_to_string("input/aoc2019_13")?;
        Ok(Self::with_str(&line))
    }

    fn with_str(input: &str) -> Self {
        Self {
            input: parse_program(input),
        }
    }
}

impl Solution for AoC2019_13 {
    fn part_one(&self) -> String {
        let mut computer = IntcodeComputer::with_size(10 * 1024);
        computer.load_program(&self.input);
        let status = computer.run();
        assert!(matches!(status, ExecutionStatus::Halted));
        let mut display = HashMap::new();
        for chunk in computer.output().chunks(3) {
            let point = Pixel::new(chunk[0], chunk[1]);
            let tile = chunk[2];
            display.insert(point, tile);
        }
        display
            .iter()
            .filter(|(_, v)| **v == TILE_BLOCK)
            .count()
            .to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 13: Care Package".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_13_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_13_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "432");
        Ok(())
    }

    #[test]
    fn aoc2019_13_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_13> {
        AoC2019_13::new()
    }
}
