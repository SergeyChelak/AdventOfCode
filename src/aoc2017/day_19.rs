use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

type Matrix = Vec<Vec<char>>;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Direction {
    Vertical,
    Horizontal,
    Any,
}

impl Direction {
    fn from_char(ch: char) -> Option<Self> {
        match ch {
            '-' => Some(Self::Horizontal),
            '|' => Some(Self::Vertical),
            '+' => Some(Self::Any),
            _ => None,
        }
    }

    fn invert(&self) -> Self {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
            _ => Self::Any,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Location {
    row: usize,
    col: usize,
}

pub struct AoC2017_19 {
    maze: Matrix,
}

impl AoC2017_19 {
    pub fn new() -> io::Result<Self> {
        let maze = read_file_as_lines("input/aoc2017_19")?
            .iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Matrix>();
        Ok(Self { maze })
    }

    fn input_location(&self) -> Location {
        let col = self.maze[0]
            .iter()
            .enumerate()
            .find(|(_, ch)| !ch.is_whitespace())
            .expect("Input should be present at the top row")
            .0;
        Location { row: 0, col }
    }

    fn get(&self, location: &Location) -> char {
        self.maze[location.row][location.col]
    }

    fn get_direction(&self, location: &Location) -> Option<Direction> {
        let ch = self.maze[location.row][location.col];
        Direction::from_char(ch)
    }
}

#[derive(Default, Hash)]
struct VisitKind {
    horizontal: bool,
    vertical: bool,
}

impl Solution for AoC2017_19 {
    fn part_one(&self) -> String {
        let mut loc = self.input_location();
        let mut direction = self.get_direction(&loc).expect("Direction not determined");
        assert_ne!(direction, Direction::Any, "Invalid direction");
        let mut prev_steps: HashMap<Location, VisitKind> = HashMap::new();
        let mut output: Vec<char> = Vec::new();
        loop {
            let ch = self.get(&loc);
            assert!(!ch.is_whitespace());
            if ch.is_alphabetic() {
                output.push(ch);
            }
            let mut step_direction = direction;
            if ch == '+' {
                direction = direction.invert();
                step_direction = Direction::Any;
            }
            let mut kind = prev_steps.entry(loc.clone()).or_default();
            match step_direction {
                Direction::Horizontal => kind.horizontal = true,
                Direction::Vertical => kind.vertical = true,
                _ => {
                    kind.horizontal = true;
                    kind.vertical = true;
                }
            }
            let is_allowed = |adj: &Location| {
                if self.get(&adj).is_whitespace() {
                    return false;
                }
                if let Some(step_kind) = prev_steps.get(&adj) {
                    match step_direction {
                        Direction::Horizontal => !step_kind.horizontal,
                        Direction::Vertical => !step_kind.vertical,
                        _ => false,
                    }
                } else {
                    true
                }
            };
            let next = match direction {
                Direction::Horizontal => {
                    let block = |loc: &Location| {
                        if loc.col > 0 {
                            let next = Location {
                                col: loc.col - 1,
                                ..*loc
                            };
                            if is_allowed(&next) {
                                return Some(next);
                            }
                        }
                        if loc.col < self.maze[loc.row].len() - 1 {
                            let next = Location {
                                col: loc.col + 1,
                                ..*loc
                            };
                            if is_allowed(&next) {
                                return Some(next);
                            }
                        }
                        None
                    };
                    block(&loc)
                }
                Direction::Vertical => {
                    let block = |loc: &Location| {
                        if loc.row > 0 {
                            let next = Location {
                                row: loc.row - 1,
                                ..*loc
                            };
                            if is_allowed(&next) {
                                return Some(next);
                            }
                        }
                        if loc.row < self.maze.len() - 1 {
                            let next = Location {
                                row: loc.row + 1,
                                ..*loc
                            };
                            if is_allowed(&next) {
                                return Some(next);
                            }
                        }
                        None
                    };
                    block(&loc)
                }
                _ => panic!("Unexpected direction"),
            };
            if let Some(next) = next {
                loc = next;
            } else {
                break;
            }
        }
        output.into_iter().collect()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 19: A Series of Tubes".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_19_input_load_test() -> io::Result<()> {
        let sol = AoC2017_19::new()?;
        assert!(!sol.maze.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_19_example1() {
        let maze = r#"        |          
        |  +--+    
        A  |  C    
    F---|----E|--+ 
        |  |  |  D 
        +B-+  +--+     
        "#
        .split('\n')
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Matrix>();
        let sol = AoC2017_19 { maze };
        assert_eq!(sol.part_one(), "ABCDEF")
    }

    #[test]
    fn aoc2017_19_correctness() -> io::Result<()> {
        let sol = AoC2017_19::new()?;
        assert_eq!(sol.part_one(), "AYRPVMEGQ");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
