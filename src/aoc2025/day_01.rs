use crate::solution::Solution;
use crate::utils::*;

use std::io;

enum RotateDirection {
    Left,
    Right,
}

struct Rotation {
    direction: RotateDirection,
    count: usize,
}

const DIAL_SIZE: usize = 100;

impl From<&str> for Rotation {
    fn from(value: &str) -> Self {
        let dir = &value[..1];
        let direction = if dir == "L" {
            RotateDirection::Left
        } else {
            assert!(dir == "R");
            RotateDirection::Right
        };
        let count = value[1..]
            .parse::<usize>()
            .expect("Rotations count must be integer");

        Self { direction, count }
    }
}

pub struct AoC2025_01 {
    input: Vec<Rotation>,
}

impl AoC2025_01 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2025_01")?;
        Ok(Self::parse(&lines))
    }

    fn parse<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|r| r.as_ref())
            .map(Rotation::from)
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2025_01 {
    fn part_one(&self) -> String {
        let mut point = 50usize;
        let mut password = 0usize;
        for rot in &self.input {
            let count = rot.count % DIAL_SIZE;
            point = match rot.direction {
                RotateDirection::Left => point + DIAL_SIZE - count,
                RotateDirection::Right => point + count,
            } % DIAL_SIZE;
            if point == 0 {
                password += 1;
            }
        }
        password.to_string()
    }

    fn part_two(&self) -> String {
        let mut point = 50usize;
        let mut password = 0usize;
        for rot in &self.input {
            let count = rot.count % DIAL_SIZE;
            password += rot.count / DIAL_SIZE;
            let new_point = match rot.direction {
                RotateDirection::Left => point + DIAL_SIZE - count,
                RotateDirection::Right => point + count,
            } % DIAL_SIZE;

            match rot.direction {
                _ if new_point == 0 => password += 1,
                _ if new_point == point => password += 1,
                _ if point == 0 => {}
                RotateDirection::Left if new_point > point => password += 1,
                RotateDirection::Right if new_point < point => password += 1,
                _ => {}
            }
            point = new_point;
        }
        password.to_string()
    }

    fn description(&self) -> String {
        "Day 1: Secret Entrance".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2025_01_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    fn make_test_puzzle() -> AoC2025_01 {
        let input = [
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];
        AoC2025_01::parse(&input)
    }

    #[test]
    fn aoc2025_01_case_1() {
        let sol = make_test_puzzle();
        assert_eq!(sol.part_one(), "3");
    }

    #[test]
    fn aoc2025_01_case_2() {
        let sol = make_test_puzzle();
        assert_eq!(sol.part_two(), "6");
    }

    #[test]
    fn aoc2025_01_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "1074");
        Ok(())
    }

    #[test]
    fn aoc2025_01_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "6254");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2025_01> {
        AoC2025_01::new()
    }
}
