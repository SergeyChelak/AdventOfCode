use crate::solution::Solution;

use std::collections::HashSet;
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

type Location = [i32; 2];

fn calc_distance(pos: Option<Location>) -> i32 {
    if let Some(pos) = pos {
        pos
        .iter()
        .map(|x| x.abs())
        .sum()
    } else {
        0
    }
}

struct NavigationIterator<'a> {
    pole: Pole,
    route: &'a Vec<Maneuver>,
    position: usize,
    location: Location
}

impl<'a> NavigationIterator<'a> {
    fn new(pole: Pole, route: &'a Vec<Maneuver>) -> Self {
        Self {
            pole,
            route,
            position: 0,
            location: [0, 0]
        }
    }
}

impl<'a> Iterator for NavigationIterator<'a> {
    type Item = Location;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.route.len() {
            let coord = if matches!(self.pole, Pole::North | Pole::South) {
                0
            } else {
                1
            };
            let item = &self.route[self.position];
            let mut steps = match item {
                Maneuver::Left(s) => -*s,
                Maneuver::Right(s) => *s
            };
            if matches!(self.pole, Pole::South | Pole::East) {
                steps = -steps;
            }
            self.location[coord] += steps;
            self.pole = self.pole.next(item);
            self.position += 1;
            Some(self.location.clone())
        } else {
            None
        }
    }
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
        let pos = NavigationIterator::new(Pole::North, &self.input)
            .last();
        calc_distance(pos)
            .to_string()
    }

    fn part_two(&self) -> String {
        let mut locations: HashSet<[i32; 2]> = HashSet::new();
        let mut prev = [0i32, 0i32];
        for pos in NavigationIterator::new(Pole::North, &self.input) {
            let axe = if pos[0] == prev[0] { 1 } else { 0 };
            let step = if pos[axe] > prev[axe] { 1 } else { -1 };
            let mut coord = prev.clone();
            while coord[axe] != pos[axe] {
                coord[axe] += step;
                if locations.contains(&coord) {
                    return calc_distance(Some(coord)).to_string();
                }
                locations.insert(coord);
            }
            prev = pos;
        }
        "Not found".to_string()
    }

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
        assert_eq!(sol.part_two(), "136");
        Ok(())
    }

    #[test]
    fn aoc2016_01_path_case1() -> io::Result<()> {
        let input = vec![
            Maneuver::Right(2),
            Maneuver::Left(3)
        ];
        let sol = AoC2016_01 {
            input
        };
        assert_eq!(sol.part_one(), "5");
        Ok(())
    }

    #[test]
    fn aoc2016_01_path_case2() -> io::Result<()> {
        let input = vec![
            Maneuver::Right(2),
            Maneuver::Right(2),
            Maneuver::Right(2)
        ];
        let sol = AoC2016_01 {
            input
        };
        assert_eq!(sol.part_one(), "2");
        Ok(())
    }

    #[test]
    fn aoc2016_01_path_case3() -> io::Result<()> {
        let input = vec![
            Maneuver::Right(5),
            Maneuver::Left(5),
            Maneuver::Right(5),
            Maneuver::Right(3)
        ];
        let sol = AoC2016_01 {
            input
        };
        assert_eq!(sol.part_one(), "12");
        Ok(())
    }

    #[test]
    fn aoc2016_01_pt2() -> io::Result<()> {
        let input = vec![
            Maneuver::Right(8),
            Maneuver::Right(4),
            Maneuver::Right(4),
            Maneuver::Right(8)
        ];
        let sol = AoC2016_01 {
            input
        };
        assert_eq!(sol.part_two(), "4");
        Ok(())
    }
}