use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io;

pub struct AoC2024_16 {
    map: Vec2<char>,
}

impl AoC2024_16 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2024_16")?;
        Ok(Self::with_str(&input))
    }

    fn with_str(input: &str) -> Self {
        let map = input
            .split('\n')
            // .map(|s| s.trim())
            // .filter(|s| !s.is_empty())
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self { map }
    }
}

impl Solution for AoC2024_16 {
    fn part_one(&self) -> String {
        let start = get_first_position(&self.map, START).expect("Start position not found");
        let end = get_first_position(&self.map, END).expect("End position not found");
        calc_lower_cost(&self.map, start, end, &mut HashMap::new())
            .map(|x| x.0.to_string())
            .unwrap_or("Path not found".to_string())
    }

    fn part_two(&self) -> String {
        let start = get_first_position(&self.map, START).expect("Start position not found");
        let end = get_first_position(&self.map, END).expect("End position not found");
        let mut path_map = HashMap::new();
        let Some((_, dir)) = calc_lower_cost(&self.map, start, end, &mut path_map) else {
            return 0.to_string();
        };

        let mut nodes = path_map.get(&(end, dir)).expect("????").clone();
        let mut path = HashSet::new();

        while !nodes.is_empty() {
            let mut next = HashSet::new();
            for node in &nodes {
                path.insert(node.0);
                let Some(adj) = path_map.get(node) else {
                    continue;
                };
                for elem in adj {
                    next.insert(*elem);
                }
            }
            nodes = next;
        }

        (1 + path.len()).to_string()
    }

    fn description(&self) -> String {
        "2024/Day 16: Reindeer Maze".to_string()
    }
}

const WALL: char = '#';
const START: char = 'S';
const END: char = 'E';

const STEP_COST: usize = 1;
const TURN_COST: usize = 1000;

type Position = Point2d<usize>;
type Node = (Position, Direction);

fn calc_lower_cost(
    map: &[Vec<char>],
    start: Position,
    end: Position,
    path_map: &mut HashMap<Node, HashSet<Node>>,
) -> Option<(usize, Direction)> {
    let mut queue = vec![(start, Direction::Right)];
    let mut table = HashMap::new();
    for item in &queue {
        table.insert(*item, 0usize);
    }
    ///////
    while let Some(elem) = queue.pop() {
        let (p, dir) = elem;
        if p == end {
            continue;
        }

        let cost = *table.get(&elem).expect("cost not found: bug in code (1)");

        let is_vertical = dir.is_vertical();

        for next in Direction::all().iter().map(|dir| (p.moved_by(dir), *dir)) {
            let (n_pos, n_dir) = next;
            if map[n_pos.y][n_pos.x] == WALL {
                continue;
            }
            let next_cost = cost
                + if is_vertical == n_dir.is_vertical() {
                    STEP_COST
                } else {
                    STEP_COST + TURN_COST
                };

            let mut is_better = true;
            let mut is_equal = false;
            if let Some(old_cost) = table.get(&next) {
                is_better = *old_cost > next_cost;
                is_equal = *old_cost == next_cost;
            }

            let path_entry = path_map.entry(next).or_default();
            if is_better {
                table.insert(next, next_cost);
                queue.push(next);
                path_entry.clear();
                path_entry.insert(elem);
            } else if is_equal {
                path_entry.insert(elem);
            }
        }
    }
    ///////
    let mut result: Option<(usize, Direction)> = None;
    for dir in Direction::all() {
        let pos = (end, dir);
        let Some(val) = table.get(&pos) else {
            continue;
        };
        let mut is_better = true;
        if let Some(cur) = result {
            is_better = cur.0 > *val
        }
        if is_better {
            result = Some((*val, dir));
        }
    }
    result
    // Direction::all()
    //     .iter()
    //     .map(|dir| (end, *dir))
    //     .filter_map(|tuple| table.get(&tuple))
    //     .copied()
    //     .inspect(|x| println!("{x}"))
    //     .min()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_16_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.map.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2024_16_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "105496");
        Ok(())
    }

    #[test]
    fn aoc2024_16_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "524");
        Ok(())
    }

    #[test]
    fn aoc2024_16_case_1() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        let puzzle = AoC2024_16::with_str(input);
        assert_eq!("7036", puzzle.part_one());
        assert_eq!("45", puzzle.part_two());
    }

    #[test]
    fn aoc2024_16_case_2() {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        let puzzle = AoC2024_16::with_str(input);
        // assert_eq!("11048", puzzle.part_one());
        assert_eq!("64", puzzle.part_two());
    }

    fn make_solution() -> io::Result<AoC2024_16> {
        AoC2024_16::new()
    }
}
