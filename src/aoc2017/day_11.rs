use crate::solution::Solution;

use std::fs::read_to_string;
use std::io;

#[derive(Clone, Copy)]
enum Edge {
    North = 0,
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

    fn inverse(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::SouthWest => Self::NorthEast,
            Self::NorthEast => Self::SouthWest,
            Self::NorthWest => Self::SouthEast,
            Self::SouthEast => Self::NorthWest,
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
    fn part_one(&self) -> String {
        let mut arr = [0usize; 6];
        self.edges.iter().for_each(|edge| {
            arr[*edge as usize] += 1;
        });
        [Edge::North, Edge::NorthEast, Edge::SouthEast].iter()
            .for_each(|edge| {
                let idx = *edge as usize;
                let idx_inv = edge.inverse() as usize;
                let val = arr[idx].min(arr[idx_inv]);
                arr[idx] -= val;
                arr[idx_inv] -= val;
            });
        let dx = arr[Edge::NorthWest as usize].min(arr[Edge::South as usize]);
        arr[Edge::SouthWest as usize] += dx;
        arr[Edge::South as usize] -= dx;
        arr[Edge::NorthWest as usize] -= dx;

        println!("dx = {dx}, {arr:?}");
        arr.iter().sum::<usize>().to_string()
    }

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
        assert_eq!(sol.part_one(), "773");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
