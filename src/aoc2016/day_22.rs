use crate::solution::Solution;
use crate::utils::*;

use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io;
use std::num::ParseIntError;

#[derive(Default, Copy, Clone, Hash)]
struct Node {
    is_target: bool,
    size: usize,
    used: usize,
}

impl Node {
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

struct InputItem {
    x: usize,
    y: usize,
    node: Node,
}

impl InputItem {
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
        let usage = Node {
            is_target: false,
            size,
            used,
        };
        Self { x, y, node: usage }
    }
}

type Grid = Vec<Vec<Node>>;
type GridSize = (usize, usize);

fn make_grid(nodes: &[InputItem]) -> (Grid, GridSize) {
    let max_x = 1 + nodes
        .iter()
        .map(|node| node.x)
        .max()
        .expect("Max X should be computable");
    let max_y = 1 + nodes
        .iter()
        .map(|node| node.y)
        .max()
        .expect("Max Y should be computable");
    let mut grid = vec![vec![Node::default(); max_x]; max_y];
    nodes.iter().for_each(|node| {
        grid[node.y][node.x] = node.node;
    });
    (grid, (max_y, max_x))
}

fn grid_hash(grid: &Grid) -> u64 {
    let mut hasher = DefaultHasher::new();
    grid.hash(&mut hasher);
    hasher.finish()
}

fn find_fewest_steps(initial: &Grid) -> usize {
    let mut steps = 0usize;
    let mut visited = HashSet::new();
    let mut grids = Vec::new();
    {
        if initial[0][0].is_target {
            return 0;
        }
        let hash = grid_hash(&initial);
        visited.insert(hash);
        grids.push(initial.clone());
    }
    'main: while !grids.is_empty() {
        steps += 1;
        println!("Step = {steps}");
        let mut next_grids = Vec::new();
        for grid in grids {
            for i in 0..grid.len() {
                for j in 0..grid[i].len() {
                    let mut positions = Vec::with_capacity(4);
                    if i > 0 {
                        positions.push((i - 1, j));
                    }
                    if i < grid.len() - 1 {
                        positions.push((i + 1, j));
                    }
                    if j > 0 {
                        positions.push((i, j - 1));
                    }
                    if j < grid[i].len() - 1 {
                        positions.push((i, j + 1));
                    }
                    for (x, y) in positions {
                        if grid[i][j].is_fit(&grid[x][y]) {
                            let mut next = grid.clone();
                            next[x][y].used += next[i][j].used;
                            next[i][j].used = 0;
                            next[x][y].is_target = next[i][j].is_target;
                            next[i][j].is_target = false;
                            if next[0][0].is_target {
                                break 'main;
                            }
                            let hash = grid_hash(&next);
                            if visited.contains(&hash) {
                                continue;
                            }
                            visited.insert(hash);
                            next_grids.push(next);
                        }
                    }
                }
            }
        }
        grids = next_grids;
    }
    steps
}

fn _print_grid(grid: &Grid) {
    for row in grid {
        for item in row {
            print!("{}/{}\t", item.used, item.size);
            if item.is_target {
                print!("#");
            }
        }
        println!();
    }
}

pub struct AoC2016_22 {
    nodes: Vec<InputItem>,
}

impl AoC2016_22 {
    pub fn new() -> io::Result<Self> {
        let lines = &read_file_as_lines("input/aoc2016_22")?[2..];
        Ok(Self::with_lines(lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let nodes = lines
            .iter()
            .map(|s| InputItem::parse(s))
            .collect::<Vec<InputItem>>();
        Self { nodes }
    }
}

impl Solution for AoC2016_22 {
    fn part_one(&self) -> String {
        let mut count = 0usize;
        for i in 0..self.nodes.len() {
            if self.nodes[i].node.is_empty() {
                continue;
            }
            for j in 0..self.nodes.len() {
                if i != j && self.nodes[i].node.is_fit(&self.nodes[j].node) {
                    count += 1;
                }
            }
        }
        count.to_string()
    }

    fn part_two(&self) -> String {
        let (mut grid, size) = make_grid(&self.nodes);
        println!("Size = {size:?}");
        grid[0][size.1 - 1].is_target = true;
        // print_grid(&grid);
        find_fewest_steps(&grid).to_string()
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
        let node = InputItem::parse("/dev/grid/node-x2-y5     87T   65T    22T   74%");
        assert_eq!(node.x, 2);
        assert_eq!(node.y, 5);
        assert_eq!(node.node.size, 87);
        assert_eq!(node.node.used, 65);
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
