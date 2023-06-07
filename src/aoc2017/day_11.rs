use crate::solution::Solution;

use std::fs::read_to_string;
use std::io;

enum Edge {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl Edge {
    fn with_str(s: &str) -> Self {
        match s {
            "n" => Self::North,
            "ne" => Self::NorthEast,
            "se" => Self::SouthEast,
            "s" => Self::South,
            "sw" => Self::SouthWest,
            "nw" => Self::NorthWest,
            _ => panic!("Unexpected value '{s}'")
        }
    }
}

pub struct AoC2017_11 {
    edges: Vec<Edge>,
}

impl AoC2017_11 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2017_11")?;
        Ok(Self::with_input(&input))
    }

    fn with_input(s: &str) -> Self {
        let edges = s.trim().split(',').map(|val| Edge::with_str(val)).collect();
        Self { edges }
    }
}

impl Solution for AoC2017_11 {
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
    fn aoc2017_11_input_load_test() -> io::Result<()> {
        let sol = AoC2017_11::new()?;
        assert!(!sol.edges.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_11_correctness() -> io::Result<()> {
        let sol = AoC2017_11::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
