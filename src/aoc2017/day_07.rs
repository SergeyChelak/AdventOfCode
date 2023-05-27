use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

struct Node {
    name: String,
    weight: u32,
    children: Vec<String>,
}

impl Node {
    fn parse(s: &str) -> Self {
        let parts = s.split(" -> ").collect::<Vec<&str>>();
        let (name, weight) = parts[0]
            .split_once(" ")
            .expect("Node should contain name and weight");
        let name = name.to_string();
        let weight = weight[1..weight.len() - 1]
            .parse::<u32>()
            .expect("Node weight should be integer");
        let children = if parts.len() > 1 {
            parts[1]
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        } else {
            vec![]
        };
        Self {
            name,
            weight,
            children,
        }
    }
}

pub struct AoC2017_07 {
    nodes: Vec<Node>,
}

impl AoC2017_07 {
    pub fn new() -> io::Result<Self> {
        let nodes = read_file_as_lines("input/aoc2017_07")?
            .iter()
            .map(|s| Node::parse(s))
            .collect::<Vec<Node>>();
        Ok(Self { nodes })
    }
}

impl Solution for AoC2017_07 {
    fn part_one(&self) -> String {
        let siblings: HashSet<String> =
            HashSet::from_iter(self.nodes.iter().flat_map(|node| &node.children).cloned());
        
        self.nodes.iter()
            .filter(|node| !node.children.is_empty())
            .map(|node| node.name.clone())
            .filter(|name| !siblings.contains(name))
            .collect::<Vec<String>>()
            .first()
            .expect("Root node not found")
            .clone()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 7: Recursive Circus".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_07_input_load_test() -> io::Result<()> {
        let sol = AoC2017_07::new()?;
        assert!(!sol.nodes.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_07_correctness() -> io::Result<()> {
        let sol = AoC2017_07::new()?;
        assert_eq!(sol.part_one(), "aapssr");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
