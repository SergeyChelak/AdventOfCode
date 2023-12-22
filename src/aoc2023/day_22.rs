use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = i32;

#[derive(Debug, Clone)]
struct Point3d {
    x: Int,
    y: Int,
    z: Int,
}

impl Point3d {
    fn from_csv(s: &str) -> Self {
        let values = s
            .split(',')
            .map(|x| {
                x.parse::<Int>()
                    .expect("Integer value as coordinate is expected")
            })
            .collect::<Vec<_>>();
        assert_eq!(values.len(), 3, "Incorrect number of coordinates");
        Self {
            x: values[0],
            y: values[1],
            z: values[2],
        }
    }
}

#[derive(Debug, Clone)]
struct Brick {
    a: Point3d,
    b: Point3d,
}

pub struct AoC2023_22 {
    bricks: Vec<Brick>,
}

impl AoC2023_22 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_22")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let bricks = lines
            .iter()
            .map(|s| s.split_once('~').expect("Delimiter not found"))
            .map(|(a, b)| Brick {
                a: Point3d::from_csv(a),
                b: Point3d::from_csv(b),
            })
            .collect::<Vec<_>>();
        Self { bricks }
    }
}

impl Solution for AoC2023_22 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 22: Sand Slabs".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_22_input_load_test() -> io::Result<()> {
        let sol = AoC2023_22::new()?;
        assert!(!sol.bricks.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_22_correctness() -> io::Result<()> {
        let sol = AoC2023_22::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
