use crate::solution::Solution;
use crate::utils::*;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io;

type Int = u32;
type Point = Position2<usize>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    point: Point,
    direction: Direction,
    stripe: u8,
}

impl Node {
    fn new(point: Point, direction: Direction, stripe: u8) -> Self {
        Self {
            point,
            direction,
            stripe,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct QueueItem {
    weight: Int,
    node: Node,
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .weight
            .cmp(&self.weight)
            .then(self.node.cmp(&other.node))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn valid_points(point: Point, rows: usize, cols: usize) -> HashMap<Direction, Point> {
    let mut adj = HashMap::new();
    let Point { row: r, col: c } = point;
    if r > 0 {
        adj.insert(Direction::Up, Point::new(r - 1, c));
    }
    if r < rows - 1 {
        adj.insert(Direction::Down, Point::new(r + 1, c));
    }
    if c > 0 {
        adj.insert(Direction::Left, Point::new(r, c - 1));
    }
    if c < cols - 1 {
        adj.insert(Direction::Right, Point::new(r, c + 1));
    }
    adj
}

fn adjacent_pt1(map: &[Vec<Int>], node: Node) -> Vec<Node> {
    let rows = map.len();
    let cols = map[0].len();
    let mut adj = Vec::new();
    let stripe = node.stripe;
    for (direction, point) in valid_points(node.point, rows, cols) {
        if direction.is_reversed(&node.direction) {
            continue;
        }
        if direction != node.direction {
            adj.push(Node::new(point, direction, 1));
        } else if stripe < 3 {
            adj.push(Node::new(point, direction, 1 + stripe));
        }
    }
    adj
}

fn adjacent_pt2(map: &[Vec<Int>], node: Node) -> Vec<Node> {
    let rows = map.len();
    let cols = map[0].len();
    let mut adj = Vec::new();
    let stripe = node.stripe;
    for (direction, point) in valid_points(node.point, rows, cols) {
        if direction.is_reversed(&node.direction) {
            continue;
        }
        if direction != node.direction && stripe >= 4 {
            adj.push(Node::new(point, direction, 1));
        } else if direction == node.direction && stripe < 10 {
            adj.push(Node::new(point, direction, 1 + stripe));
        }
    }
    adj
}

type Adjacent = dyn Fn(&[Vec<Int>], Node) -> Vec<Node>;

fn dijkstra(map: &[Vec<Int>], start: Point, target: Point, adjacent: &Adjacent) -> Option<Int> {
    // init
    let mut weights: HashMap<Node, Int> = HashMap::new();
    let mut queue: BinaryHeap<QueueItem> = BinaryHeap::new();
    [
        Node::new(start, Direction::Down, 0),
        Node::new(start, Direction::Right, 0),
    ]
    .iter()
    .for_each(|node| {
        weights.insert(*node, 0);
        queue.push(QueueItem {
            node: *node,
            weight: 0,
        });
    });
    // main loop
    while let Some(item) = queue.pop() {
        let QueueItem { node, weight } = item;
        if node.point == target {
            return Some(weight);
        }
        for adj in adjacent(map, node) {
            let new_weight = weight + map[adj.point.row][adj.point.col];
            if let Some(old_weight) = weights.get(&adj) {
                if new_weight >= *old_weight {
                    continue;
                }
            }
            weights.insert(adj, new_weight);
            queue.push(QueueItem {
                node: adj,
                weight: new_weight,
            });
        }
    }
    None
}

pub struct AoC2023_17 {
    map: Vec<Vec<Int>>,
}

impl AoC2023_17 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_17")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let map = lines
            .iter()
            .map(|s| {
                s.chars()
                    .map(|ch| ch.to_digit(10).expect("Digit is expected"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { map }
    }

    fn start(&self) -> Point {
        Point::new(0, 0)
    }

    fn target(&self) -> Point {
        Point::new(self.map.len() - 1, self.map[0].len() - 1)
    }
}

impl Solution for AoC2023_17 {
    fn part_one(&self) -> String {
        let Some(val) = dijkstra(&self.map, self.start(), self.target(), &adjacent_pt1) else {
            return "Not found".to_string();
        };
        val.to_string()
    }

    fn part_two(&self) -> String {
        let Some(val) = dijkstra(&self.map, self.start(), self.target(), &adjacent_pt2) else {
            return "Not found".to_string();
        };
        val.to_string()
    }

    fn description(&self) -> String {
        "AoC 2023/Day 17: Clumsy Crucible".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_17_input_load_test() -> io::Result<()> {
        let sol = AoC2023_17::new()?;
        assert!(!sol.map.is_empty());
        Ok(())
    }

    fn puzzle() -> AoC2023_17 {
        let lines = [
            "2413432311323",
            "3215453535623",
            "3255245654254",
            "3446585845452",
            "4546657867536",
            "1438598798454",
            "4457876987766",
            "3637877979653",
            "4654967986887",
            "4564679986453",
            "1224686865563",
            "2546548887735",
            "4322674655533",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        AoC2023_17::with_lines(&lines)
    }

    #[test]
    fn aoc2023_17_ex1() {
        assert_eq!(puzzle().part_one(), "102")
    }

    #[test]
    fn aoc2023_17_ex2() {
        assert_eq!(puzzle().part_two(), "94")
    }

    #[test]
    fn aoc2023_17_correctness() -> io::Result<()> {
        let sol = AoC2023_17::new()?;
        assert_eq!(sol.part_one(), "1008");
        assert_eq!(sol.part_two(), "1210");
        Ok(())
    }
}
