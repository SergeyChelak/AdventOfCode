use crate::solution::Solution;
use crate::utils::*;

use std::io;
use std::num::ParseIntError;

struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
    used_percent: usize,
    avail: usize,
}

impl Node {
    fn parse(s: &str) -> Self {
        let tokens = s.split_whitespace().collect::<Vec<&str>>();
        println!("{:?}", tokens);
        let (x, y) = {
            let name = tokens[0].split('-').collect::<Vec<&str>>();
            let x = name[1][1..]
                .parse::<usize>()
                .expect("Node x value should be integer");
            let y = name[2][1..]
                .parse::<usize>()
                .expect("Node y value should be integer");
            (x, y)
        };
        let value_of = |index: usize| -> Result<usize, ParseIntError> {
            let len = tokens[index].len();
            tokens[index][0..len - 1].parse::<usize>()
        };

        let size = value_of(1).expect("Size should be integer value");
        let used = value_of(2).expect("Used should be integer value");
        let used_percent = value_of(4).expect("Percentage should be integer value");
        let avail = value_of(3).expect("Avail should be integer value");
        Self {
            x,
            y,
            size,
            used,
            used_percent,
            avail,
        }
    }
}

pub struct AoC2016_22 {
    nodes: Vec<Node>,
}

impl AoC2016_22 {
    pub fn new() -> io::Result<Self> {
        let nodes = read_file_as_lines("input/aoc2016_22")?[2..]
            .iter()
            .map(|s| Node::parse(s))
            .collect::<Vec<Node>>();
        Ok(Self { nodes })
    }
}

impl Solution for AoC2016_22 {
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
    fn aoc2016_22_input_load_test() -> io::Result<()> {
        let sol = AoC2016_22::new()?;
        assert!(!sol.nodes.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2016_22_parse() {
        let node = Node::parse("/dev/grid/node-x2-y5     87T   65T    22T   74%");
        assert_eq!(node.x, 2);
        assert_eq!(node.y, 5);
        assert_eq!(node.size, 87);
        assert_eq!(node.used, 65);
        assert_eq!(node.avail, 22);
        assert_eq!(node.used_percent, 74);
    }

    #[test]
    fn aoc2016_22_correctness() -> io::Result<()> {
        let sol = AoC2016_22::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
