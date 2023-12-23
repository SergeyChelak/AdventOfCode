use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(usize, usize);

const MAP_PATH: char = '.';
// const MAP_FOREST: char = '#';
const MAP_SLOPE_UP: char = '^';
const MAP_SLOPE_DOWN: char = 'v';
const MAP_SLOPE_LEFT: char = '<';
const MAP_SLOPE_RIGHT: char = '>';

pub struct AoC2023_23 {
    maze: Vec<Vec<char>>,
}

impl AoC2023_23 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_23")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let maze = lines
            .iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self { maze }
    }

    fn path_position(&self, row: usize) -> Option<Position> {
        let Some(col) = self.maze[row].iter().position(|x| *x == MAP_PATH) else {
            return None;
        };
        Some(Position(row, col))
    }

    fn position_start(&self) -> Position {
        self.path_position(0).expect("Start position not found")
    }

    fn position_end(&self) -> Position {
        let row = self.maze.len() - 1;
        self.path_position(row).expect("End position not found")
    }
}

impl Solution for AoC2023_23 {
    fn part_one(&self) -> String {
        let mut max = 0usize;
        let start = self.position_start();
        let target = self.position_end();
        bt_search(&self.maze, &target, start, &mut HashSet::new(), 0, &mut max);
        max.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 23: A Long Walk".to_string()
    }
}

fn bt_search(
    map: &[Vec<char>],
    target: &Position,
    pos: Position,
    seen: &mut HashSet<Position>,
    acc: usize,
    max: &mut usize,
) {
    if pos == *target {
        *max = acc.max(*max);
        return;
    }
    let Position(row, col) = pos;
    let rows = map.len();
    let cols = map[rows - 1].len();
    let mut possible: HashMap<Direction, Position> = HashMap::new();
    if row > 0 {
        possible.insert(Direction::Up, Position(row - 1, col));
    }
    if row < rows - 1 {
        possible.insert(Direction::Down, Position(row + 1, col));
    }
    if col > 0 {
        possible.insert(Direction::Left, Position(row, col - 1));
    }
    if col < cols - 1 {
        possible.insert(Direction::Right, Position(row, col + 1));
    }
    let valid_directions = match map[row][col] {
        MAP_PATH => vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ],
        MAP_SLOPE_UP => vec![Direction::Up],
        MAP_SLOPE_DOWN => vec![Direction::Down],
        MAP_SLOPE_LEFT => vec![Direction::Left],
        MAP_SLOPE_RIGHT => vec![Direction::Right],
        _ => vec![],
    };
    for dir in valid_directions {
        let Some(next) = possible.get(&dir) else {
            continue;
        };
        if seen.contains(next) {
            continue;
        }
        seen.insert(*next);
        bt_search(map, target, *next, seen, acc + 1, max);
        seen.remove(next);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_23_input_load_test() -> io::Result<()> {
        let sol = AoC2023_23::new()?;
        assert!(!sol.maze.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_23_ex1() {
        let input = [
            "#.#####################",
            "#.......#########...###",
            "#######.#########.#.###",
            "###.....#.>.>.###.#.###",
            "###v#####.#v#.###.#.###",
            "###.>...#.#.#.....#...#",
            "###v###.#.#.#########.#",
            "###...#.#.#.......#...#",
            "#####.#.#.#######.#.###",
            "#.....#.#.#.......#...#",
            "#.#####.#.#.#########v#",
            "#.#...#...#...###...>.#",
            "#.#.#v#######v###.###v#",
            "#...#.>.#...>.>.#.###.#",
            "#####v#.#.###v#.#.###.#",
            "#.....#...#...#.#.#...#",
            "#.#########.###.#.#.###",
            "#...###...#...#...#.###",
            "###.###.#.###v#####v###",
            "#...#...#.#.>.>.#.>.###",
            "#.###.###.#.###.#.#v###",
            "#.....###...###...#...#",
            "#####################.#",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let puzzle = AoC2023_23::with_lines(&input);
        assert_eq!(puzzle.part_one(), "94");
    }

    #[test]
    fn aoc2023_23_correctness() -> io::Result<()> {
        let sol = AoC2023_23::new()?;
        assert_eq!(sol.part_one(), "2442");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
