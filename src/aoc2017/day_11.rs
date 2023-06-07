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

    // fn inverse(&self) -> Self {
    //     match self {
    //         Self::North => Self::South,
    //         Self::South => Self::North,
    //         Self::SouthWest => Self::NorthEast,
    //         Self::NorthEast => Self::SouthWest,
    //         Self::NorthWest => Self::SouthEast,
    //         Self::SouthEast => Self::NorthWest,
    //     }
    // }
}

struct Container {
    edge_a: Edge,
    edge_b: Edge,
    delta_edge: Option<Edge>,
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
        sum.to_string()
    }

    // fn part_two(&self) -> String {
    // }

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
