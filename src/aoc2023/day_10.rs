use crate::solution::Solution;
use crate::utils::*;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
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

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Position(Int, Int);

impl Position {
    fn left(&self) -> Self {
        Self(self.0, self.1 - 1)
    }

    fn right(&self) -> Self {
        Self(self.0, self.1 + 1)
    }

    fn up(&self) -> Self {
        Self(self.0 - 1, self.1)
    }

    fn down(&self) -> Self {
        Self(self.0 + 1, self.1)
    }
}

type Maze = HashMap<Position, Pipe>;

pub struct AoC2023_10 {
    maze: RefCell<Maze>,
    start: Position,
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
                let pos = Position(r as Int, c as Int);
                if ch == 'S' {
                    start = Some(pos);
                } else {
                    let val = Pipe::try_from(ch).expect("Unknown pipe type");
                    maze.insert(pos, val);
                }
            }
        }
        Self {
            maze: RefCell::new(maze),
            start: start.expect("Start position should be present"),
        }
    }

    fn possible_start_values(&self) -> Vec<Pipe> {
        // NorthSouth, // | is a vertical pipe connecting north and south
        // EastWest,   // - is a horizontal pipe connecting east and west
        // NorthEast,  // L is a 90-degree bend connecting north and east
        // NorthWest,  // J is a 90-degree bend connecting north and west
        // SouthWest,  // 7 is a 90-degree bend connecting south and west
        // SouthEast,  // F is a 90-degree bend connecting south and east

        let mut possible = Vec::new();
        let maze = self.maze.borrow();

        let up = maze.get(&self.start.up());
        let down = maze.get(&self.start.down());
        let left = maze.get(&self.start.left());
        let right = maze.get(&self.start.right());

        println!(
            "Adjacent cells:\nUP = {:?}\nDOWN = {:?}\nLEFT = {:?}\nRIGHT = {:?}",
            up, down, left, right
        );

        use Pipe::*;
        let allowed_up = HashSet::from([SouthWest, NorthSouth, SouthEast]);
        let allowed_down = HashSet::from([NorthWest, NorthSouth, NorthEast]);
        let allowed_left = HashSet::from([EastWest, NorthEast, SouthEast]);
        let allowed_right = HashSet::from([EastWest, SouthWest, NorthWest]);

        if let (Some(up), Some(down)) = (up, down) {
            if allowed_up.contains(up) && allowed_down.contains(down) {
                possible.push(NorthSouth);
            }
        }
        if let (Some(left), Some(right)) = (left, right) {
            if allowed_left.contains(left) && allowed_right.contains(right) {
                possible.push(EastWest);
            }
        }

        if let (Some(up), Some(left)) = (up, left) {
            if allowed_left.contains(left) && allowed_up.contains(up) {
                possible.push(NorthWest);
            }
        }

        if let (Some(down), Some(left)) = (down, left) {
            if allowed_left.contains(left) && allowed_down.contains(down) {
                possible.push(SouthWest);
            }
        }
        if let (Some(up), Some(right)) = (up, right) {
            if allowed_up.contains(up) && allowed_right.contains(right) {
                possible.push(NorthEast);
            }
        }
        if let (Some(down), Some(right)) = (down, right) {
            if allowed_down.contains(down) && allowed_right.contains(right) {
                possible.push(SouthEast);
            }
        }
        possible
    }

    fn find_loop_len(
        &self,
        pos: Position,
        reached: &mut HashSet<Position>,
        step: usize,
        result: &mut Option<usize>,
    ) {
        if result.is_some() {
            return;
        }
        let maze = self.maze.borrow();
        let Some(val) = maze.get(&pos) else {
            return;
        };
        if reached.contains(&pos) {
            if pos == self.start && step > 2 {
                *result = Some(step);
            }
            return;
        }
        reached.insert(pos);
        use Pipe::*;
        let adjacent = match val {
            NorthSouth => [pos.up(), pos.down()], // | is a vertical pipe connecting north and south
            EastWest => [pos.left(), pos.right()], // - is a horizontal pipe connecting east and west
            NorthEast => [pos.up(), pos.right()], // L is a 90-degree bend connecting north and east
            NorthWest => [pos.up(), pos.left()],  // J is a 90-degree bend connecting north and west
            SouthWest => [pos.down(), pos.left()], // 7 is a 90-degree bend connecting south and west
            SouthEast => [pos.down(), pos.right()], // F is a 90-degree bend connecting south and east
            Ground => panic!("Ground is not expected value"),
        };
        for next in adjacent {
            self.find_loop_len(next, reached, step + 1, result);
        }
    }
}

impl Solution for AoC2023_10 {
    fn part_one(&self) -> String {
        for pipe in self.possible_start_values() {
            println!("Trying with {pipe:?}");
            {
                let mut maze = self.maze.borrow_mut();
                maze.insert(self.start, pipe);
            }
            let mut result: Option<usize> = None;
            self.find_loop_len(self.start, &mut HashSet::new(), 0, &mut result);
            if let Some(val) = result {
                return (val / 2).to_string();
            }
        }
        "Not found".to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 10: Pipe Maze".to_string()
    }
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
    fn aoc2023_10_correctness() -> io::Result<()> {
        let sol = AoC2023_10::new()?;
        assert_eq!(sol.part_one(), "6956");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
