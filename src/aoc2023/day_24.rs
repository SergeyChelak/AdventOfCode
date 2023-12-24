use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = i64;

#[derive(Debug, Copy, Clone)]
struct Vector3d {
    x: Int,
    y: Int,
    z: Int,
}

impl From<&str> for Vector3d {
    fn from(value: &str) -> Self {
        let values = value
            .split(", ")
            .map(|s| s.parse::<Int>().expect("Numeric value is expected"))
            .collect::<Vec<_>>();
        assert!(value.len() > 2);
        Self {
            x: values[0],
            y: values[1],
            z: values[2],
        }
    }
}

struct Hailstone {
    position: Vector3d,
    velocity: Vector3d,
}

impl From<&str> for Hailstone {
    fn from(value: &str) -> Self {
        let (position, velocity) = value
            .split_once(" @ ")
            .expect("Failed to parse hailstone data");
        Self {
            position: Vector3d::from(position),
            velocity: Vector3d::from(velocity),
        }
    }
}

pub struct AoC2023_24 {
    input: Vec<Hailstone>,
}

impl AoC2023_24 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_24")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let input = lines
            .iter()
            .map(|s| Hailstone::from(s.as_str()))
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2023_24 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 24: Never Tell Me The Odds".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_24_input_load_test() -> io::Result<()> {
        let sol = AoC2023_24::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_24_correctness() -> io::Result<()> {
        let sol = AoC2023_24::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
