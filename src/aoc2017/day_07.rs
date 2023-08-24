use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

struct Item {
    name: String,
    weight: u32,
    children: Vec<String>,
}

impl Item {
    fn parse(s: &str) -> Self {
        let parts = s.split(" -> ").collect::<Vec<&str>>();
        let (name, weight) = parts[0]
            .split_once(' ')
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

struct Node {
    weight: u32,
    children: Vec<Node>,
}

impl Node {
    fn corrected_weight(&self, w: &mut u32) -> u32 {
        if *w != u32::MAX {
            return 0;
        }
        let weights = self
            .children
            .iter()
            .map(|child| child.corrected_weight(w))
            .collect::<Vec<u32>>();
        if *w != u32::MAX {
            return 0;
        }
        let sum = weights.iter().sum::<u32>();
        if !weights.is_empty() {
            let (min, max) = (
                *weights.iter().min().unwrap(),
                *weights.iter().max().unwrap(),
            );
            let delta = max - min;
            if delta > 0 {
                let (val, is_inc) = if sum - delta == weights.len() as u32 * min {
                    (max, false)
                } else {
                    (min, true)
                };
                for (i, weight) in weights.iter().enumerate() {
                    if *weight == val {
                        *w = if is_inc {
                            self.children[i].weight + delta
                        } else {
                            self.children[i].weight - delta
                        };
                        return 0;
                    }
                }
            }
        }
        self.weight + sum
    }
}

pub struct AoC2017_07 {
    nodes: Vec<Item>,
}

impl AoC2017_07 {
    pub fn new() -> io::Result<Self> {
        let nodes = read_file_as_lines("input/aoc2017_07")?
            .iter()
            .map(|s| Item::parse(s))
            .collect::<Vec<Item>>();
        Ok(Self { nodes })
    }

    fn tree_with_root(&self, name: &str) -> Option<Node> {
        let item = self.nodes.iter().find(|node| node.name == name)?;
        let children: Vec<Node> = item
            .children
            .iter()
            .filter_map(|child| self.tree_with_root(child))
            .collect();
        Some(Node {
            weight: item.weight,
            children,
        })
    }

    fn root_node_name(&self) -> String {
        let siblings: HashSet<String> =
            HashSet::from_iter(self.nodes.iter().flat_map(|node| &node.children).cloned());

        self.nodes
            .iter()
            .filter(|node| !node.children.is_empty())
            .map(|node| node.name.clone())
            .filter(|name| !siblings.contains(name))
            .take(1)
            .collect::<String>()
    }
}

impl Solution for AoC2017_07 {
    fn part_one(&self) -> String {
        self.root_node_name()
    }

    fn part_two(&self) -> String {
        let root = self.root_node_name();
        let Some(tree) = self.tree_with_root(&root) else {
            return "Not found".to_string();
        };
        let mut weight = u32::MAX;
        _ = tree.corrected_weight(&mut weight);
        weight.to_string()
    }

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
        assert_eq!(sol.part_two(), "1458");
        Ok(())
    }
}
