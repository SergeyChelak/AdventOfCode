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
            _ => panic!("Unexpected value '{s}'"),
        }
    }
}

struct Container {
    edge_a: Edge,
    edge_b: Edge,
    delta_edge: Option<Edge>,
}

fn distance(steps: &[Edge]) -> usize {
    let mut arr = [0usize; 6];
    steps.iter().for_each(|edge| {
        arr[*edge as usize] += 1;
    });
    let mut sum = arr.iter().sum();
    loop {
        [
            Container {
                edge_a: Edge::North,
                edge_b: Edge::South,
                delta_edge: None,
            },
            Container {
                edge_a: Edge::NorthEast,
                edge_b: Edge::SouthWest,
                delta_edge: None,
            },
            Container {
                edge_a: Edge::SouthEast,
                edge_b: Edge::NorthWest,
                delta_edge: None,
            },
            Container {
                edge_a: Edge::NorthWest,
                edge_b: Edge::South,
                delta_edge: Some(Edge::SouthWest),
            },
            Container {
                edge_a: Edge::SouthEast,
                edge_b: Edge::North,
                delta_edge: Some(Edge::NorthEast),
            },
            Container {
                edge_a: Edge::SouthWest,
                edge_b: Edge::North,
                delta_edge: Some(Edge::NorthWest),
            },
            Container {
                edge_a: Edge::NorthEast,
                edge_b: Edge::South,
                delta_edge: Some(Edge::SouthEast),
            },
        ]
        .iter()
        .for_each(|container| {
            let idx_a = container.edge_a as usize;
            let idx_b = container.edge_b as usize;

            let val = arr[idx_a].min(arr[idx_b]);
            arr[idx_a] -= val;
            arr[idx_b] -= val;
            if let Some(edge) = container.delta_edge {
                arr[edge as usize] += val;
            }
        });
        let new_sum = arr.iter().sum::<usize>();
        if new_sum == sum {
            break;
        }
        sum = new_sum;
    }
    sum
}

pub struct AoC2017_11 {
    steps: Vec<Edge>,
}

impl AoC2017_11 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2017_11")?;
        Ok(Self::with_input(&input))
    }

    fn with_input(s: &str) -> Self {
        let steps = s.trim().split(',').map(Edge::with_str).collect();
        Self { steps }
    }
}

impl Solution for AoC2017_11 {
    fn part_one(&self) -> String {
        distance(&self.steps).to_string()
    }

    fn part_two(&self) -> String {
        let mut max_steps = 0usize;
        for l in 1..=self.steps.len() {
            max_steps = max_steps.max(distance(&self.steps[0..l]));
        }
        max_steps.to_string()
    }

    fn description(&self) -> String {
        "AoC 2017/Day 11: Hex Ed".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_11_input_load_test() -> io::Result<()> {
        let sol = AoC2017_11::new()?;
        assert!(!sol.steps.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_11_correctness() -> io::Result<()> {
        let sol = AoC2017_11::new()?;
        assert_eq!(sol.part_one(), "773");
        assert_eq!(sol.part_two(), "1560");
        Ok(())
    }
}
