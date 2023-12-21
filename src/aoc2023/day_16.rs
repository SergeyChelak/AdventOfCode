use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashSet, VecDeque};
use std::io;

impl Direction {
    fn turn(&self, ch: char) -> Vec<Self> {
        match (ch, self) {
            ('/', Direction::Left) => vec![Self::Down],
            ('/', Direction::Down) => vec![Self::Left],
            ('/', Direction::Right) => vec![Self::Up],
            ('/', Direction::Up) => vec![Self::Right],

            ('\\', Direction::Left) => vec![Self::Up],
            ('\\', Direction::Up) => vec![Self::Left],
            ('\\', Direction::Right) => vec![Self::Down],
            ('\\', Direction::Down) => vec![Self::Right],

            ('-', Direction::Up) | ('-', Direction::Down) => vec![Self::Left, Self::Right],
            ('|', Direction::Left) | ('|', Direction::Right) => vec![Self::Up, Self::Down],
            _ => vec![*self],
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Location {
    row: usize,
    col: usize,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Beam {
    dir: Direction,
    loc: Location,
}

pub struct AoC2023_16 {
    contraption: Vec<Vec<char>>,
}

impl AoC2023_16 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_16")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let contraption = lines
            .iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect();
        Self { contraption }
    }

    fn next(&self, beam: &Beam) -> Vec<Beam> {
        let (row, col) = (beam.loc.row, beam.loc.col);
        let ch = self.contraption[row][col];
        let mut res = Vec::new();
        for dir in beam.dir.turn(ch) {
            match dir {
                Direction::Up if row > 0 => res.push(Beam {
                    dir,
                    loc: Location { row: row - 1, col },
                }),
                Direction::Down if row < self.contraption.len() - 1 => res.push(Beam {
                    dir,
                    loc: Location { row: row + 1, col },
                }),
                Direction::Left if col > 0 => res.push(Beam {
                    dir,
                    loc: Location { row, col: col - 1 },
                }),
                Direction::Right if col < self.contraption[row].len() - 1 => res.push(Beam {
                    dir,
                    loc: Location { row, col: col + 1 },
                }),
                _ => {
                    // no op
                }
            }
        }
        res
    }

    fn energized(&self, start: Beam) -> usize {
        let mut energized = HashSet::from([start]);
        let mut beams = VecDeque::from([start]);
        while !beams.is_empty() {
            let beam = beams
                .pop_front()
                .expect("Beams must contain 1 item at least");
            for next in self.next(&beam) {
                if energized.contains(&next) {
                    continue;
                }
                energized.insert(next);
                beams.push_back(next);
            }
        }
        energized
            .iter()
            .map(|beam| beam.loc)
            .collect::<HashSet<_>>()
            .len()
    }
}

impl Solution for AoC2023_16 {
    fn part_one(&self) -> String {
        let start = Beam {
            dir: Direction::Right,
            loc: Location { row: 0, col: 0 },
        };
        self.energized(start).to_string()
    }

    fn part_two(&self) -> String {
        let mut directions = Vec::new();
        for row in 0..self.contraption.len() {
            let right = Beam {
                dir: Direction::Right,
                loc: Location { row, col: 0 },
            };

            let left = Beam {
                dir: Direction::Left,
                loc: Location {
                    row,
                    col: self.contraption[row].len() - 1,
                },
            };
            directions.push(left);
            directions.push(right);
        }
        for col in 0..self.contraption[0].len() {
            let down = Beam {
                dir: Direction::Down,
                loc: Location { row: 0, col },
            };

            let up = Beam {
                dir: Direction::Left,
                loc: Location {
                    row: self.contraption.len() - 1,
                    col,
                },
            };
            directions.push(up);
            directions.push(down);
        }
        directions
            .iter()
            .map(|beam| self.energized(*beam))
            .max()
            .expect("Expected at least 1 energized configuration")
            .to_string()
    }

    fn description(&self) -> String {
        "AoC 2023/Day 16: The Floor Will Be Lava".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_16_input_load_test() -> io::Result<()> {
        let sol = AoC2023_16::new()?;
        assert!(!sol.contraption.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_16_ex1() {
        assert_eq!(puzzle().part_one(), "46")
    }

    #[test]
    fn aoc2023_16_ex2() {
        assert_eq!(puzzle().part_two(), "51")
    }

    fn puzzle() -> AoC2023_16 {
        let lines = [
            r#".|...\...."#,
            r#"|.-.\....."#,
            r#".....|-..."#,
            r#"........|."#,
            r#".........."#,
            r#".........\"#,
            r#"..../.\\.."#,
            r#".-.-/..|.."#,
            r#".|....-|.\"#,
            r#"..//.|...."#,
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        AoC2023_16::with_lines(&lines)
    }

    #[test]
    fn aoc2023_16_correctness() -> io::Result<()> {
        let sol = AoC2023_16::new()?;
        assert_eq!(sol.part_one(), "7307");
        assert_eq!(sol.part_two(), "7635");
        Ok(())
    }
}
