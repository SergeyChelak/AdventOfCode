use crate::solution::Solution;
use crate::utils::*;

use std::io;

const DIM: usize = 4;
type Int = i32;
type Coordinate = [Int; DIM];

pub struct AoC2018_25 {
    points: Vec<Coordinate>,
}

impl AoC2018_25 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2018_25")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let points = lines.iter().map(|s| parse_coordinate(s.as_str())).collect();
        Self { points }
    }
}

impl Solution for AoC2018_25 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 25: Four-Dimensional Adventure".to_string()
    }
}

fn distance(a: &Coordinate, b: &Coordinate) -> Int {
    a.iter().zip(b.iter()).map(|(a, b)| (a - b).abs()).sum()
}

fn parse_coordinate(s: &str) -> Coordinate {
    let coords = s
        .split(',')
        .map(|x| x.parse::<Int>().expect("Non integer coordinate value"))
        .collect::<Vec<Int>>();
    coords.try_into().expect("Invalid coordinate size")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_25_input_load_test() -> io::Result<()> {
        let sol = AoC2018_25::new()?;
        assert_eq!(sol.points.len(), 1498);
        Ok(())
    }

    #[test]
    fn aoc2018_25_correctness() -> io::Result<()> {
        let sol = AoC2018_25::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
