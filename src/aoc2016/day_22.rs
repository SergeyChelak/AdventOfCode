use crate::solution::Solution;
use crate::utils::*;

use std::io;
use std::num::ParseIntError;

struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
}

impl Node {
    fn parse(s: &str) -> Self {
        let tokens = s.split_whitespace().collect::<Vec<&str>>();
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
        Self { x, y, size, used }
    }

    fn avail(&self) -> usize {
        self.size - self.used
    }
}

pub struct AoC2016_22 {
    nodes: Vec<Node>,
}

impl AoC2016_22 {
    pub fn new() -> io::Result<Self> {
        let lines = &read_file_as_lines("input/aoc2016_22")?[2..];
        Ok(Self::with_lines(lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let nodes = lines.iter().map(|s| Node::parse(s)).collect::<Vec<Node>>();
        Self { nodes }
    }
}

impl Solution for AoC2016_22 {
    fn part_one(&self) -> String {
        let mut count = 0usize;
        for i in 0..self.nodes.len() {
            if self.nodes[i].used == 0 {
                continue;
            }
            for j in 0..self.nodes.len() {
                if i != j && self.nodes[i].used <= self.nodes[j].avail() {
                    count += 1;
                }
            }
        }
        count.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2016/Day 22: Grid Computing".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_22_input_load_test() -> io::Result<()> {
        let sol = AoC2016_22::new()?;
        assert_eq!(sol.nodes.len(), 990);
        Ok(())
    }

    #[test]
    fn aoc2016_22_parse() {
        let node = Node::parse("/dev/grid/node-x2-y5     87T   65T    22T   74%");
        assert_eq!(node.x, 2);
        assert_eq!(node.y, 5);
        assert_eq!(node.size, 87);
        assert_eq!(node.used, 65);
    }

    #[test]
    fn aoc2016_22_correctness() -> io::Result<()> {
        let sol = AoC2016_22::new()?;
        assert_eq!(sol.part_one(), "960");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2016_22_fewest_steps() {
        let lines = [
            "/dev/grid/node-x0-y0   10T    8T     2T   80%",
            "/dev/grid/node-x0-y1   11T    6T     5T   54%",
            "/dev/grid/node-x0-y2   32T   28T     4T   87%",
            "/dev/grid/node-x1-y0    9T    7T     2T   77%",
            "/dev/grid/node-x1-y1    8T    0T     8T    0%",
            "/dev/grid/node-x1-y2   11T    7T     4T   63%",
            "/dev/grid/node-x2-y0   10T    6T     4T   60%",
            "/dev/grid/node-x2-y1    9T    8T     1T   88%",
            "/dev/grid/node-x2-y2    9T    6T     3T   66%",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        let sol = AoC2016_22::with_lines(&lines);
        assert_eq!(sol.part_two(), "7");
    }
}
