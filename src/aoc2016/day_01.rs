use crate::solution::Solution;

use std::fs::read_to_string;
use std::io;

enum Maneuver {
    Left(i32),
    Right(i32)
}

impl Maneuver {
    fn with_str(s: &str) -> Self {
        let dir = &s[..=0];
        let steps = s[1..].parse::<i32>().expect("Incorrect input format");
        match dir {
            "L" => Maneuver::Left(steps),
            "R" => Maneuver::Right(steps),
            _ => panic!("unexpected direction {}", dir)
        }
    }
}

enum Pole {
    North,
    South,
    East,
    West
}

impl Pole {
    fn next(&self, maneuver: &Maneuver) -> Self {
        let is_left = if let Maneuver::Left(_) = maneuver {
            true
        } else {
            false
        };
        match self {
            Pole::North => if is_left {
                Pole::West
            } else {
                Pole::East
            },
            Pole::West => if is_left {
                Pole::South
            } else {
                Pole::North
            },
            Pole::East => if is_left {
                Pole::North
            } else {
                Pole::South
            },
            Pole::South => if is_left {
                Pole::East
            } else {
                Pole::West
            }
        }
    }
}

fn calc_distance(items: &Vec<Maneuver>) -> i32 {
    let mut location: [i32; 2] = [0, 0];
    let mut pole = Pole::North;    
    for item in items {
        let coord = if matches!(pole, Pole::North | Pole::South) {
            0
        } else {
            1
        };
        let mut steps = match item {
            Maneuver::Left(s) => -*s,
            Maneuver::Right(s) => *s
        };
        if matches!(pole, Pole::South | Pole::East) {
            steps = -steps;
        }
        location[coord] += steps;
        pole = pole.next(item);
    }
    location
        .iter()
        .map(|x| x.abs())
        .sum()
}

pub struct AoC2016_01 {
    input: Vec<Maneuver>
}

impl AoC2016_01 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: Self::parse_input("input/aoc2016_01")?
        })
    }

    fn parse_input(file: &str) -> io::Result<Vec<Maneuver>> {
        Ok(read_to_string(file)?
            .trim()
            .split(", ")
            .map(|token| Maneuver::with_str(token))
            .collect::<Vec<Maneuver>>()
        )
    }
}

impl Solution for AoC2016_01 {
    fn part_one(&self) -> String {
        calc_distance(&self.input)
            .to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2016/Day 1: No Time for a Taxicab".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_01_input_load_test() -> io::Result<()> {
        let sol = AoC2016_01::new()?;
        assert!(sol.input.len() > 0);
        Ok(())
    }

    #[test]
    fn aoc2016_01_correctness() -> io::Result<()> {
        let sol = AoC2016_01::new()?;
        assert_eq!(sol.part_one(), "209");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2016_01_path_case1() {
        let input = vec![
            Maneuver::Right(2),
            Maneuver::Left(3)
        ];
        assert_eq!(calc_distance(&input), 5)
    }

    #[test]
    fn aoc2016_01_path_case2() {
        let input = vec![
            Maneuver::Right(2),
            Maneuver::Right(2),
            Maneuver::Right(2)
        ];
        assert_eq!(calc_distance(&input), 2)
    }

    #[test]
    fn aoc2016_01_path_case3() {
        let input = vec![
            Maneuver::Right(5),
            Maneuver::Left(5),
            Maneuver::Right(5),
            Maneuver::Right(3)
        ];
        assert_eq!(calc_distance(&input), 12)
    }
}