use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io;
use std::num::ParseIntError;

#[derive(Copy, Clone)]
struct StorageInfo {
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

type Coordinate = Point2d<usize>;

impl Coordinate {
    fn safe_left(&self) -> Option<Self> {
        if self.x > 0 {
            Some(Self {
                x: self.x - 1,
                y: self.y,
            })
        } else {
            None
        }
    }

    fn safe_right(&self) -> Option<Self> {
        Some(Self {
            x: self.x + 1,
            y: self.y,
        })
    }

    fn safe_up(&self) -> Option<Self> {
        if self.y > 0 {
            Some(Self {
                x: self.x,
                y: self.y - 1,
            })
        } else {
            None
        }
    }

    fn safe_down(&self) -> Option<Self> {
        Some(Self {
            x: self.x,
            y: self.y + 1,
        })
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct SearchState {
    empty: Coordinate,
    target: Coordinate,
}

type Grid = HashMap<Coordinate, StorageInfo>;

fn make_search_state(grid: &Grid) -> SearchState {
    let target = {
        let max_x = grid
            .iter()
            .map(|(pos, _)| *pos)
            .max_by(|&a, &b| a.x.cmp(&b.x))
            .expect("Max X should be computable")
            .x;
        Coordinate { x: max_x, y: 0 }
    };

    let empty = *grid
        .iter()
        .find(|(_, info)| info.is_empty())
        .expect("Empty node should be present")
        .0;

    SearchState { empty, target }
}

fn bfs_fewest_steps(grid: &Grid) -> usize {
    let mut visited: HashSet<SearchState> = HashSet::new();
    let mut states = Vec::new();
    let state = make_search_state(grid);
    states.push(state);
    let empty = grid
        .get(&state.empty)
        .expect("Empty node should be present");
    let grid = grid
        .iter()
        .filter(|(_, info)| info.is_fit(empty))
        .map(|(&pos, &info)| (pos, info))
        .collect::<Grid>();
    let mut steps = 0;
    'outer: while !states.is_empty() {
        steps += 1;
        let mut next_states = Vec::new();
        for state in states {
            let adjacent = adjacent_nodes(&grid, &state.empty);
            for pos in adjacent {
                let target = if pos == state.target {
                    state.empty
                } else {
                    state.target
                };
                let state = SearchState { empty: pos, target };
                if visited.contains(&state) {
                    continue;
                }
                if target.x == 0 && target.y == 0 {
                    break 'outer;
                }
                visited.insert(state);
                next_states.push(state);
            }
        }
        states = next_states;
    }
    steps
}

fn adjacent_nodes(grid: &Grid, position: &Coordinate) -> Vec<Coordinate> {
    vec![
        position.safe_left(),
        position.safe_right(),
        position.safe_up(),
        position.safe_down(),
    ]
    .into_iter()
    .filter_map(|opt| {
        if let Some(pos) = opt {
            if grid.contains_key(&pos) {
                return Some(pos);
            }
        }
        None
    })
    .collect()
}

pub struct AoC2016_22 {
    grid: Grid,
}

impl AoC2016_22 {
    pub fn new() -> io::Result<Self> {
        let lines = &read_file_as_lines("input/aoc2016_22")?[2..];
        Ok(Self::with_lines(lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let mut grid = HashMap::new();
        lines.iter().for_each(|s| {
            let (pos, info) = Self::parse_node_info(s);
            grid.insert(pos, info);
        });
        Self { grid }
    }

    fn parse_node_info(s: &str) -> (Coordinate, StorageInfo) {
        let tokens = s.split_whitespace().collect::<Vec<&str>>();
        let position = {
            let name = tokens[0].split('-').collect::<Vec<&str>>();
            let x = name[1][1..]
                .parse::<usize>()
                .expect("Node x value should be integer");
            let y = name[2][1..]
                .parse::<usize>()
                .expect("Node y value should be integer");
            Coordinate { x, y }
        };
        let value_of = |index: usize| -> Result<usize, ParseIntError> {
            let len = tokens[index].len();
            tokens[index][0..len - 1].parse::<usize>()
        };

        let size = value_of(1).expect("Size should be integer value");
        let used = value_of(2).expect("Used should be integer value");
        let info = StorageInfo { size, used };
        (position, info)
    }
}

impl Solution for AoC2016_22 {
    fn part_one(&self) -> String {
        let mut count = 0usize;
        for (i, (_, info_i)) in self.grid.iter().enumerate() {
            if info_i.is_empty() {
                continue;
            }
            for (j, (_, info_j)) in self.grid.iter().enumerate() {
                if i != j && info_i.is_fit(info_j) {
                    count += 1;
                }
            }
        }
        count.to_string()
    }

    fn part_two(&self) -> String {
        bfs_fewest_steps(&self.grid).to_string()
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
        assert_eq!(sol.grid.len(), 990);
        Ok(())
    }

    #[test]
    fn aoc2016_22_parse() {
        let (pos, info) =
            AoC2016_22::parse_node_info("/dev/grid/node-x2-y5     87T   65T    22T   74%");
        assert_eq!(pos.x, 2);
        assert_eq!(pos.y, 5);
        assert_eq!(info.size, 87);
        assert_eq!(info.used, 65);
    }

    #[test]
    fn aoc2016_22_correctness() -> io::Result<()> {
        let sol = AoC2016_22::new()?;
        assert_eq!(sol.part_one(), "960");
        assert_eq!(sol.part_two(), "225");
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
