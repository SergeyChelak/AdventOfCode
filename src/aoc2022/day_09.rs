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

    fn simulate(&self, knots: usize) -> usize {
        assert!(knots > 1);
        let mut visited = HashSet::<Point>::new();
        visited.insert(Point::zero());
        let mut rope = vec![Point::zero(); knots];
        for movement in self.input.iter() {
            for _ in 0..movement.count {
                rope[0] = rope[0].moved_by(&movement.dir);
                for i in 1..knots {
                    rope[i] = next_tail_position(&rope[i - 1], &rope[i]);
                }
                visited.insert(rope[knots - 1]);
            }
        }
        visited.len()
    }
}

impl Solution for AoC2022_09 {
    fn part_one(&self) -> String {
        self.simulate(2).to_string()
    }

    fn part_two(&self) -> String {
        self.simulate(10).to_string()
    }

    fn description(&self) -> String {
        "Day 9: Rope Bridge".to_string()
    }
}

fn next_tail_position(head: &Point, t: &Point) -> Point {
    let mut tail = *t;
    let dx = head.x - tail.x;
    let dy = head.y - tail.y;

    // If the knot is further than 1 step in any direction
    if dx.abs() > 1 || dy.abs() > 1 {
        // Use signum to move 1 step (-1, 0, or 1) in the direction of the leader
        tail.x += dx.signum();
        tail.y += dy.signum();
    }

    tail
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
        assert_eq!(sol.part_one(), "5930");
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
        assert_eq!(sol.part_two(), "1");
    }

    #[test]
    fn aoc2022_09_case_2() {
        let input = ["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"];
        let sol = AoC2022_09::parse_lines(&input);
        assert_eq!(sol.part_two(), "36")
    }

    fn make_solution() -> io::Result<AoC2022_09> {
        AoC2022_09::new()
    }

    fn make_test_solution() -> AoC2022_09 {
        let input = ["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];
        AoC2022_09::parse_lines(&input)
    }
}
