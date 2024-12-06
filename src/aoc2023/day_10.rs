use crate::solution::Solution;
use crate::utils::*;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Pipe {
    NorthSouth, // | is a vertical pipe connecting north and south
    EastWest,   // - is a horizontal pipe connecting east and west
    NorthEast,  // L is a 90-degree bend connecting north and east
    NorthWest,  // J is a 90-degree bend connecting north and west
    SouthWest,  // 7 is a 90-degree bend connecting south and west
    SouthEast,  // F is a 90-degree bend connecting south and east
    Ground,
}

impl TryFrom<char> for Pipe {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Pipe::*;
        match value {
            '|' => Ok(NorthSouth),
            '-' => Ok(EastWest),
            'L' => Ok(NorthEast),
            'J' => Ok(NorthWest),
            '7' => Ok(SouthWest),
            'F' => Ok(SouthEast),
            '.' => Ok(Ground),
            _ => Err(()),
        }
    }
}

type Int = i32;
type Position = Point2d<Int>;

type Maze = HashMap<Position, Pipe>;

pub struct AoC2023_10 {
    maze: RefCell<Maze>,
    start: Position,
    rows: Int,
    cols: Int,
}

impl AoC2023_10 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_10")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let mut start: Option<Position> = None;
        let mut maze: Maze = HashMap::new();
        for (r, line) in lines.iter().enumerate() {
            for (c, ch) in line.chars().enumerate() {
                let pos = Position::new(r as Int, c as Int);
                if ch == 'S' {
                    start = Some(pos);
                } else {
                    let val = Pipe::try_from(ch).expect("Unknown pipe type");
                    maze.insert(pos, val);
                }
            }
        }
        let Some(start) = start else {
            panic!("Start position should be present")
        };
        let Some(start_value) = Self::calc_start_value(&maze, start) else {
            panic!("Failed to calculate start value")
        };
        maze.insert(start, start_value);
        Self {
            maze: RefCell::new(maze),
            start,
            rows: lines.len() as Int,
            cols: lines[0].len() as Int,
        }
    }

    fn calc_start_value(maze: &Maze, start: Position) -> Option<Pipe> {
        let up = maze.get(&start.up());
        let down = maze.get(&start.down());
        let left = maze.get(&start.left());
        let right = maze.get(&start.right());

        use Pipe::*;
        let allowed_up = HashSet::from([SouthWest, NorthSouth, SouthEast]);
        let allowed_down = HashSet::from([NorthWest, NorthSouth, NorthEast]);
        let allowed_left = HashSet::from([EastWest, NorthEast, SouthEast]);
        let allowed_right = HashSet::from([EastWest, SouthWest, NorthWest]);

        if let (Some(up), Some(down)) = (up, down) {
            if allowed_up.contains(up) && allowed_down.contains(down) {
                return Some(NorthSouth);
            }
        }
        if let (Some(left), Some(right)) = (left, right) {
            if allowed_left.contains(left) && allowed_right.contains(right) {
                return Some(EastWest);
            }
        }

        if let (Some(up), Some(left)) = (up, left) {
            if allowed_left.contains(left) && allowed_up.contains(up) {
                return Some(NorthWest);
            }
        }

        if let (Some(down), Some(left)) = (down, left) {
            if allowed_left.contains(left) && allowed_down.contains(down) {
                return Some(SouthWest);
            }
        }
        if let (Some(up), Some(right)) = (up, right) {
            if allowed_up.contains(up) && allowed_right.contains(right) {
                return Some(NorthEast);
            }
        }
        if let (Some(down), Some(right)) = (down, right) {
            if allowed_down.contains(down) && allowed_right.contains(right) {
                return Some(SouthEast);
            }
        }
        None
    }

    fn find_loop(&self) -> HashSet<Position> {
        let mut seen = HashSet::new();
        let mut deque: VecDeque<Position> = VecDeque::from([self.start]);
        while !deque.is_empty() {
            let pos = deque.pop_front().expect("Deque shouldn't be empty");
            let value = {
                let maze = self.maze.borrow();
                let Some(val) = maze.get(&pos) else {
                    panic!("Unexpected case (1)")
                };
                *val
            };
            use Pipe::*;
            let adjacent = match value {
                NorthSouth => [pos.up(), pos.down()],
                EastWest => [pos.left(), pos.right()],
                NorthEast => [pos.up(), pos.right()],
                NorthWest => [pos.up(), pos.left()],
                SouthWest => [pos.down(), pos.left()],
                SouthEast => [pos.down(), pos.right()],
                Ground => panic!("Ground is not expected value"),
            };
            adjacent.iter().for_each(|p| {
                if !seen.contains(p) {
                    seen.insert(*p);
                    deque.push_back(*p)
                }
            });
        }
        seen
    }
}

impl Solution for AoC2023_10 {
    fn part_one(&self) -> String {
        let path = self.find_loop();
        (path.len() / 2).to_string()
    }

    fn part_two(&self) -> String {
        let path = self.find_loop();
        // simplify maze
        {
            let mut maze = self.maze.borrow_mut();
            for row in 0..self.rows {
                for col in 0..self.cols {
                    let pos = Position::new(row, col);
                    if !path.contains(&pos) {
                        maze.insert(pos, Pipe::Ground);
                    }
                }
            }
        }
        let maze = self.maze.borrow();
        let mut outside: HashSet<Position> = HashSet::new();
        for row in 0..self.rows {
            let mut is_within = false;
            let mut edge_start = EdgeStart::Nope;
            for col in 0..self.cols {
                let pos = Position::new(row, col);
                let item = *maze.get(&pos).expect("Expected item (3)");
                match item {
                    Pipe::NorthSouth => {
                        // |
                        assert_eq!(edge_start, EdgeStart::Nope);
                        is_within = !is_within;
                    }
                    Pipe::EastWest => {
                        // -
                        assert_ne!(edge_start, EdgeStart::Nope);
                    }
                    Pipe::NorthEast => {
                        // L
                        assert_eq!(edge_start, EdgeStart::Nope);
                        edge_start = EdgeStart::North;
                    }
                    Pipe::SouthEast => {
                        // F
                        assert_eq!(edge_start, EdgeStart::Nope);
                        edge_start = EdgeStart::South;
                    }
                    Pipe::NorthWest => {
                        // J
                        assert_ne!(edge_start, EdgeStart::Nope);
                        if edge_start == EdgeStart::South {
                            is_within = !is_within;
                        }
                        edge_start = EdgeStart::Nope;
                    }
                    Pipe::SouthWest => {
                        // 7
                        assert_ne!(edge_start, EdgeStart::Nope);
                        if edge_start == EdgeStart::North {
                            is_within = !is_within;
                        }
                        edge_start = EdgeStart::Nope;
                    }
                    Pipe::Ground => {
                        if !is_within {
                            outside.insert(pos);
                        }
                    }
                }
            }
        }
        let result = self.rows * self.cols - outside.len() as Int - path.len() as Int;
        result.to_string()
    }

    fn description(&self) -> String {
        "AoC 2023/Day 10: Pipe Maze".to_string()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum EdgeStart {
    North,
    South,
    Nope,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_10_input_load_test() -> io::Result<()> {
        let sol = AoC2023_10::new()?;
        assert!(!sol.maze.borrow().is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_10_ex1_1() {
        #[rustfmt::skip]
        let inp = [
            ".....",
            ".S-7.",
            ".|.|.",
            ".L-J.",
            ".....",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let puzzle = AoC2023_10::with_lines(&inp);
        assert_eq!(puzzle.part_one(), "4");
    }

    #[test]
    fn aoc2023_10_ex2_1() {
        #[rustfmt::skip]
        let inp = [
            "...........",
            ".S-------7.",
            ".|F-----7|.",
            ".||.....||.",
            ".||.....||.",
            ".|L-7.F-J|.",
            ".|..|.|..|.",
            ".L--J.L--J.",
            "...........",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let puzzle = AoC2023_10::with_lines(&inp);
        assert_eq!(puzzle.part_two(), "4");
    }

    #[test]
    fn aoc2023_10_ex2_2() {
        #[rustfmt::skip]
        let inp = [
            "FF7FSF7F7F7F7F7F---7",
            "L|LJ||||||||||||F--J",
            "FL-7LJLJ||||||LJL-77",
            "F--JF--7||LJLJ7F7FJ-",
            "L---JF-JLJ.||-FJLJJ7",
            "|F|F-JF---7F7-L7L|7|",
            "|FFJF7L7F-JF7|JL---7",
            "7-L-JL7||F7|L7F-7F7|",
            "L.L7LFJ|||||FJL7||LJ",
            "L7JLJL-JLJLJL--JLJ.L",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let puzzle = AoC2023_10::with_lines(&inp);
        assert_eq!(puzzle.part_two(), "10");
    }

    #[test]
    fn aoc2023_10_correctness() -> io::Result<()> {
        let sol = AoC2023_10::new()?;
        assert_eq!(sol.part_one(), "6956");
        assert_eq!(sol.part_two(), "455");
        Ok(())
    }
}
