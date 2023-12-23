use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(usize, usize);

const MAP_PATH: char = '.';
const MAP_FOREST: char = '#';
const MAP_SLOPE_UP: char = '^';
const MAP_SLOPE_DOWN: char = 'v';
const MAP_SLOPE_LEFT: char = '<';
const MAP_SLOPE_RIGHT: char = '>';

type DirectionProvider = dyn Fn(char) -> Vec<Direction>;
type Graph = HashMap<Position, Vec<(Position, usize)>>;

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

    fn find_longest_path(&self, provider: &DirectionProvider) -> usize {
        let branch_points = self.branch_points(provider);
        // downsize complexity
        let mut graph = Graph::new();
        for point in &branch_points {
            let mut stack = vec![(*point, 0)];
            let mut seen = HashSet::from([*point]);
            while let Some((other, steps)) = stack.pop() {
                if steps != 0 && branch_points.contains(&other) {
                    let entry = graph.entry(*point).or_insert(Vec::new());
                    entry.push((other, steps));
                    continue;
                }
                for next in self.adjacent(other, provider) {
                    if seen.contains(&next) {
                        continue;
                    }
                    seen.insert(next);
                    stack.push((next, 1 + steps));
                }
            }
        }
        bfs(
            &graph,
            self.position_end(),
            self.position_start(),
            &mut HashSet::new(),
        )
        .expect("Not found")
    }

    fn branch_points(&self, provider: &DirectionProvider) -> Vec<Position> {
        let mut result = vec![self.position_start(), self.position_end()];
        for (r, row) in self.maze.iter().enumerate() {
            for (c, _) in row.iter().enumerate() {
                let pos = Position(r, c);
                let adj = self.adjacent(pos, provider);
                if adj.len() > 2 {
                    result.push(pos);
                }
            }
        }
        result
    }

    fn adjacent(&self, pos: Position, provider: &DirectionProvider) -> Vec<Position> {
        let Position(row, col) = pos;
        let rows = self.maze.len();
        let cols = self.maze[rows - 1].len();
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
        provider(self.maze[row][col])
            .iter()
            .filter_map(|dir| possible.get(dir))
            .filter(|Position(r, c)| self.maze[*r][*c] != MAP_FOREST)
            .copied()
            .collect::<Vec<_>>()
    }
}

impl Solution for AoC2023_23 {
    fn part_one(&self) -> String {
        let provider = |ch: char| -> Vec<Direction> {
            match ch {
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
            }
        };
        self.find_longest_path(&provider).to_string()
    }

    fn part_two(&self) -> String {
        let provider = |ch: char| -> Vec<Direction> {
            match ch {
                MAP_FOREST => vec![],
                _ => vec![
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ],
            }
        };
        self.find_longest_path(&provider).to_string()
    }

    fn description(&self) -> String {
        "AoC 2023/Day 23: A Long Walk".to_string()
    }
}

fn bfs(
    graph: &Graph,
    target: Position,
    pos: Position,
    seen: &mut HashSet<Position>,
) -> Option<usize> {
    if pos == target {
        return Some(0);
    }
    let Some(linked) = graph.get(&pos) else {
        return None;
    };
    let mut res = None;
    seen.insert(pos);
    for (next, steps) in linked {
        if seen.contains(next) {
            continue;
        }
        if let Some(val) = bfs(graph, target, *next, seen) {
            let val = steps + val;
            res = Some(res.unwrap_or(0).max(val));
        }
    }
    seen.remove(&pos);
    res
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
        assert_eq!(puzzle().part_one(), "94");
    }

    #[test]
    fn aoc2023_23_ex2() {
        assert_eq!(puzzle().part_two(), "154");
    }

    fn puzzle() -> AoC2023_23 {
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
        AoC2023_23::with_lines(&input)
    }

    #[test]
    fn aoc2023_23_correctness() -> io::Result<()> {
        let sol = AoC2023_23::new()?;
        assert_eq!(sol.part_one(), "2442");
        assert_eq!(sol.part_two(), "6898");
        Ok(())
    }
}
