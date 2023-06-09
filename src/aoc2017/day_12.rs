use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

type Graph = HashMap<usize, usize>;

pub struct AoC2017_12 {
    graph: Graph,
}

impl AoC2017_12 {
    pub fn new() -> io::Result<Self> {
        let mut graph = Graph::new();
        read_file_as_lines("input/aoc2017_12")?
            .iter()
            .map(|s| Self::parse_line(s))
            .for_each(|(node, connections)| {
                connections.iter().for_each(|&elem| {
                    graph.insert(node, elem);
                    graph.insert(elem, node);
                })
            });
        Ok(Self { graph })
    }

    fn parse_line(s: &str) -> (usize, Vec<usize>) {
        let (node, connections) = s
            .split_once(" <-> ")
            .expect("Items should be separated with  <->");
        let node = node
            .parse::<usize>()
            .expect("Integer expected as node number");
        let connections = connections
            .split(", ")
            .map(|x| {
                x.parse::<usize>()
                    .expect("Connected node number integer number")
            })
            .collect::<Vec<usize>>();
        (node, connections)
    }
}

impl Solution for AoC2017_12 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 12: Digital Plumber".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_12_input_load_test() -> io::Result<()> {
        let sol = AoC2017_12::new()?;
        assert!(!sol.graph.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_12_correctness() -> io::Result<()> {
        let sol = AoC2017_12::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
