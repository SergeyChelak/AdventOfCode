use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = i32;
type Point = Point2d<Int>;

pub struct AoC2018_06 {
    points: Vec<Point>,
}

impl AoC2018_06 {
    pub fn new() -> io::Result<Self> {
        let points = read_file_as_lines("input/aoc2018_06")?
            .iter()
            .map(|s| {
                Point::parse_csv(s)
                    .unwrap_or_else(|err| panic!("Failed to parse coordinate from '{s}' string, error: {err:?}"))
            })
            .collect::<Vec<Point>>();
        Ok(Self { points })
    }
}

impl Solution for AoC2018_06 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 6: Chronal Coordinates".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_06_input_load_test() -> io::Result<()> {
        let sol = AoC2018_06::new()?;
        assert!(!sol.points.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_06_correctness() -> io::Result<()> {
        let sol = AoC2018_06::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
