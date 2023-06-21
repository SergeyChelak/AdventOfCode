use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

type PointScalar = isize;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    row: PointScalar,
    col: PointScalar,
}

impl Point {
    fn forward(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.row -= 1,
            Direction::Left => self.col -= 1,
            Direction::Down => self.row += 1,
            Direction::Right => self.col += 1,
        }
    }
}

pub struct AoC2017_22 {
    center: Point,
    infected: HashSet<Point>,
}

impl AoC2017_22 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2017_22")?;
        Ok(Self::from_lines(&lines))
    }

    fn from_lines(lines: &[String]) -> Self {
        let mut infected = HashSet::new();
        lines.iter().enumerate().for_each(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .for_each(|(col, _)| {
                    let point = Point {
                        row: row as PointScalar,
                        col: col as PointScalar,
                    };
                    infected.insert(point);
                });
        });
        let center = {
            let rows = lines.len() as PointScalar;
            let cols = lines[0].chars().count() as PointScalar;
            Point {
                row: rows / 2,
                col: cols / 2,
            }
        };
        Self { center, infected }
    }

    fn contaminate(&self, bursts: usize) -> usize {
        let mut state = self.infected.clone();
        let mut ptr = self.center;
        let mut dir = Direction::Up;
        let mut count = 0;
        for _ in 0..bursts {
            let is_infected = state.contains(&ptr);
            dir = if is_infected {
                state.remove(&ptr);
                dir.turn_right()
            } else {
                count += 1;
                state.insert(ptr);
                dir.turn_left()
            };
            ptr.forward(&dir);
        }
        count
    }
}

impl Solution for AoC2017_22 {
    fn part_one(&self) -> String {
        self.contaminate(10_000).to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 22: Sporifica Virus".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_22_input_load_test() -> io::Result<()> {
        let sol = AoC2017_22::new()?;
        assert!(!sol.infected.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_22_example1() {
        let lines = ["..#", "#..", "..."]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let s = AoC2017_22::from_lines(&lines);
        assert_eq!(s.contaminate(70), 41);
        assert_eq!(s.contaminate(10_000), 5587);
    }

    #[test]
    fn aoc2017_22_correctness() -> io::Result<()> {
        let sol = AoC2017_22::new()?;
        assert_eq!(sol.part_one(), "5406");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
