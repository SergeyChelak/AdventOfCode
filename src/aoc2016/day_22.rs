use crate::solution::Solution;
use crate::utils::*;

use std::collections::hash_map::DefaultHasher;
use std::collections::{HashSet, HashMap};
use std::hash::{Hash, Hasher};
use std::io;
use std::num::ParseIntError;

#[derive(Default, Copy, Clone, Hash)]
struct StorageInfo {
    is_target: bool,
    size: usize,
    used: usize,
}

impl StorageInfo {
    fn avail(&self) -> usize {
        self.size - self.used
    }

    fn is_empty(&self) -> bool {
        self.used == 0
    }

    /// Check if node would fit on other node
    fn is_fit(&self, other: &Self) -> bool {
        self.used <= other.avail()
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

// find zero node location
// build node's grid
// mark target node
// reduce node's grid to movable/empty/target/permanent
// search with hashing empty position + target data position

// fn make_grid(nodes: &[Node]) -> Grid {
//     let max_x = 1 + nodes
//         .iter()
//         .map(|node| node.x)
//         .max()
//         .expect("Max X should be computable");
//     let max_y = 1 + nodes
//         .iter()
//         .map(|node| node.y)
//         .max()
//         .expect("Max Y should be computable");
//     let mut grid = vec![vec![StorageInfo::default(); max_x]; max_y];
//     nodes.iter().for_each(|node| {
//         grid[node.y][node.x] = node.node;
//     });
//     grid[0][max_x - 1].is_target = true;
//     grid
// }

// fn grid_hash(grid: &Grid) -> u64 {
//     let mut hasher = DefaultHasher::new();
//     grid.hash(&mut hasher);
//     hasher.finish()
// }

// fn _print_grid(grid: &Grid) {
//     for row in grid {
//         for item in row {
//             print!("{}/{}\t", item.used, item.size);
//             if item.is_target {
//                 print!("#");
//             }
//         }
//         println!();
//     }
// }

pub struct AoC2016_22 {
    nodes: HashMap<Position, StorageInfo>,
}

impl AoC2016_22 {
    pub fn new() -> io::Result<Self> {
        let lines = &read_file_as_lines("input/aoc2016_22")?[2..];
        Ok(Self::with_lines(lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let mut nodes = HashMap::new();
        lines
            .iter()
            .for_each(|s| {
                let (pos, info) = Self::parse_node_info(s);
                nodes.insert(pos, info);
            });
        Self { nodes }
    }

    fn parse_node_info(s: &str) -> (Position, StorageInfo) {
        let tokens = s.split_whitespace().collect::<Vec<&str>>();
        let position = {
            let name = tokens[0].split('-').collect::<Vec<&str>>();
            let x = name[1][1..]
                .parse::<usize>()
                .expect("Node x value should be integer");
            let y = name[2][1..]
                .parse::<usize>()
                .expect("Node y value should be integer");
            Position { x, y }
        };
        let value_of = |index: usize| -> Result<usize, ParseIntError> {
        let len = tokens[index].len();
            tokens[index][0..len - 1].parse::<usize>()
        };

        let size = value_of(1).expect("Size should be integer value");
        let used = value_of(2).expect("Used should be integer value");
        let info = StorageInfo {
            is_target: false,
            size,
            used,
        };
        (position, info)
    }
}

impl Solution for AoC2016_22 {
    fn part_one(&self) -> String {
        let mut count = 0usize;
        for (i, (_, info_i)) in self.nodes.iter().enumerate() {
            if info_i.is_empty() {
                continue;
            }
            for (j, (_, info_j)) in self.nodes.iter().enumerate() {
                if i != j && info_i.is_fit(&info_j) {
                    count += 1;
                }
            }
        }
        count.to_string()
    }

    fn part_two(&self) -> String {
        "".to_string()
    }

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
        let (pos, info) = AoC2016_22::parse_node_info("/dev/grid/node-x2-y5     87T   65T    22T   74%");
        assert_eq!(pos.x, 2);
        assert_eq!(pos.y, 5);
        assert_eq!(info.size, 87);
        assert_eq!(info.used, 65);
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
