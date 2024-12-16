use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
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
        calc_lower_cost(&self.map, start, end)
            .map(|x| x.to_string())
            .unwrap_or("Path not found".to_string())
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "2024/Day 16: Reindeer Maze".to_string()
    }
}

const WALL: char = '#';
// const EMPTY: char = '.';
const START: char = 'S';
const END: char = 'E';

const STEP_COST: usize = 1;
const TURN_COST: usize = 1000;

type Position = Position2<usize>;

fn get_first_position(map: &[Vec<char>], element: char) -> Option<Position> {
    for (row, arr) in map.iter().enumerate() {
        for (col, val) in arr.iter().enumerate() {
            if *val == element {
                return Some(Position { row, col });
            }
        }
    }
    None
}

fn calc_lower_cost(map: &[Vec<char>], start: Position, end: Position) -> Option<usize> {
    let mut queue = Direction::all()
        .iter()
        .map(|dir| (start, *dir))
        .collect::<Vec<_>>();

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

        for next in Direction::all().iter().map(|dir| {
            let mut next = p;
            use Direction::*;
            match dir {
                Up => next.row -= 1,
                Down => next.row += 1,
                Left => next.col -= 1,
                Right => next.col += 1,
            }
            (next, *dir)
        }) {
            let (n_pos, n_dir) = next;
            if map[n_pos.row][n_pos.col] == WALL {
                continue;
            }
            let next_cost = cost
                + if is_vertical == n_dir.is_vertical() {
                    1usize
                } else {
                    1001
                };

            let mut is_better = true;
            if let Some(old_cost) = table.get(&next) {
                is_better = *old_cost > next_cost;
            }

            if is_better {
                table.insert(next, next_cost);
                queue.push(next);
            }
        }
    }
    ///////
    Direction::all()
        .iter()
        .map(|dir| (end, *dir))
        .filter_map(|tuple| table.get(&tuple))
        .copied()
        .min()
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
        assert_eq!(sol.part_two(), "");
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
        assert_eq!("11048", puzzle.part_one());
    }

    fn make_solution() -> io::Result<AoC2024_16> {
        AoC2024_16::new()
    }
}
