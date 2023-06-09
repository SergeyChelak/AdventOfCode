use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
use std::io;

type Graph = Vec<Vec<bool>>;

fn connections_count(graph: &Graph, node: usize) -> usize {
    let mut count = 0;
    let mut visited: HashSet<usize> = HashSet::new();
    let mut nodes = vec![node];    
    while !nodes.is_empty() {
        count += nodes.len();
        let mut next = Vec::new();
        for n in nodes {
            visited.insert(n);
            for (i, val) in graph[n].iter().enumerate() {
                if !val || visited.contains(&i) {
                    continue;
                }
                next.push(i);
            }
        }
        nodes = next;
    }
    count
}

pub struct AoC2017_12 {
    graph: Graph,
}

impl AoC2017_12 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2017_12")?;
        let count = lines.len();
        let mut graph = vec![vec![false; count]; count];
        lines
            .iter()
            .map(|s| Self::parse_line(s))
            .for_each(|(node, connections)| {
                connections.iter().for_each(|&elem| {
                    graph[node][elem] = true;
                    graph[elem][node] = true;
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
    fn part_one(&self) -> String {
        connections_count(&self.graph, 0).to_string()
    }

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
        assert_eq!(sol.part_one(), "378");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
