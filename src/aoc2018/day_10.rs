use crate::solution::Solution;
use crate::utils::*;

use std::io;

type PointElem = Point2d<i32>;

struct Point {
    position: PointElem,
    speed: PointElem,
}

impl Point {
    fn from_str(s: &str) -> Self {
        let idx = s.find("velocity").expect("velocity parameter is not found");
        let position = Self::parse_parameter_value(&s[..idx]);
        let speed = Self::parse_parameter_value(&s[idx..]);
        Self { position, speed }
    }

    fn parse_parameter_value(s: &str) -> PointElem {
        let mut s = s.trim().split_once('=').expect("'=' not found").1;
        s = remove_first_and_last(s);
        PointElem::parse_csv(s).unwrap()
    }
}

pub struct AoC2018_10 {
    points: Vec<Point>,
}

impl AoC2018_10 {
    pub fn new() -> io::Result<Self> {
        let points = read_file_as_lines("input/aoc2018_10")?
            .iter()
            .map(|x| Point::from_str(x))
            .collect::<Vec<Point>>();
        Ok(Self { points })
    }
}

impl Solution for AoC2018_10 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 10: The Stars Align".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_10_input_load_test() -> io::Result<()> {
        let sol = AoC2018_10::new()?;
        assert!(!sol.points.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_10_correctness() -> io::Result<()> {
        let sol = AoC2018_10::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
