use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

type Int = i32;
type Point = Point2d<Int>;

#[derive(Debug, Clone)]
struct Movement {
    dir: Direction,
    count: usize,
}

impl From<&str> for Movement {
    fn from(value: &str) -> Self {
        let (dir, steps) = value.split_once(' ').expect("Invalid movement format");
        let dir = match dir {
            "D" => Direction::Down,
            "U" => Direction::Up,
            "R" => Direction::Right,
            "L" => Direction::Left,
            _ => unreachable!(),
        };

        let count = steps.parse::<usize>().expect("Steps must be integer value");

        Self { dir, count }
    }
}

pub struct AoC2022_09 {
    input: Vec<Movement>,
}

impl AoC2022_09 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2022_09")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(Movement::from)
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2022_09 {
    fn part_one(&self) -> String {
        let mut visited = HashSet::<Point>::new();
        let mut head = Point::zero();
        let mut tail = Point::zero();
        visited.insert(tail);
        for movement in self.input.iter() {
            for _ in 0..movement.count {
                head = head.moved_by(&movement.dir);
                let dx = tail.x - head.x;
                let dy = tail.y - head.y;

                if dx.abs() == 2 {
                    tail.x += -dx.signum();
                    if dy.abs() == 1 {
                        tail.y += -dy.signum();
                    }
                }
                if dy.abs() == 2 {
                    tail.y += -dy.signum();
                    if dx.abs() == 1 {
                        tail.x += -dx.signum();
                    }
                }
                visited.insert(tail);
            }
        }
        visited.len().to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 9: Rope Bridge".to_string()
    }
}

fn _debug_print(h: &Point, t: &Point) {
    for y in -10..10 {
        for x in -10..10 {
            let point = Point::new(x, y);
            if *h == point {
                print!("H");
            } else if *t == point {
                print!("T");
            } else if point == Point::zero() {
                print!("s");
            } else {
                print!(".")
            }
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_09_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2022_09_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "");
        Ok(())
    }

    #[test]
    fn aoc2022_09_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2022_09_case_1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "13");
    }

    fn make_solution() -> io::Result<AoC2022_09> {
        AoC2022_09::new()
    }

    fn make_test_solution() -> AoC2022_09 {
        let input = ["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];
        AoC2022_09::parse_lines(&input)
    }
}
