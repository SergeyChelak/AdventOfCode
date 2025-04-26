use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

type Position = Point2d<usize>;

pub struct AoC2024_06 {
    input: Vec2<char>,
    start_position: Position,
}

impl AoC2024_06 {
    pub fn new() -> io::Result<Self> {
        let arr = read_file_as_lines("input/aoc2024_06")?;
        Ok(Self::with_strings(&arr))
    }

    fn with_strings<T: AsRef<str>>(arr: &[T]) -> Self {
        let mut input = Vec::with_capacity(arr.len());
        let mut start: Option<Position> = None;

        for (row, s) in arr.iter().enumerate() {
            let chars = s.as_ref().chars().collect::<Vec<_>>();
            if let Some(col) = chars.iter().position(|x| *x == '^') {
                start = Some(Position { y: row, x: col });
            }
            input.push(chars);
        }

        Self {
            input,
            start_position: start.expect("Initial position not found"),
        }
    }
}

impl Solution for AoC2024_06 {
    fn part_one(&self) -> String {
        let mut visited = HashSet::<Position>::new();
        let mut direction = Direction::Up;
        let mut position = self.start_position;
        loop {
            visited.insert(position);
            let Some(next) = next_position(&self.input, position, direction) else {
                break;
            };
            if self.input[next.y][next.x] == '#' {
                direction = direction.turn_right();
                continue;
            }
            position = next;
        }
        visited.len().to_string()
    }

    fn part_two(&self) -> String {
        let mut total = 0;
        let mut matrix = self.input.clone();
        for row in 0..matrix.len() {
            for col in 0..matrix[row].len() {
                if matrix[row][col] != '.' {
                    continue;
                }
                matrix[row][col] = '#';
                if is_stuck(&matrix, self.start_position, Direction::Up) {
                    total += 1;
                }
                matrix[row][col] = '.';
            }
        }
        total.to_string()
    }

    fn description(&self) -> String {
        "2024/Day 6: Guard Gallivant".to_string()
    }
}

fn is_stuck(matrix: &[Vec<char>], mut position: Position, mut direction: Direction) -> bool {
    let mut visited = HashSet::<Visit>::new();
    loop {
        let visit = Visit {
            position,
            direction,
        };
        if visited.contains(&visit) {
            return true;
        }
        visited.insert(visit);
        let Some(next) = next_position(matrix, position, direction) else {
            return false;
        };
        if matrix[next.y][next.x] == '#' {
            direction = direction.turn_right();
            continue;
        }
        position = next;
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Visit {
    position: Position,
    direction: Direction,
}

fn next_position(
    matrix: &[Vec<char>],
    position: Position,
    direction: Direction,
) -> Option<Position> {
    let Position { y: row, x: col } = position;
    let next = match direction {
        Direction::Up if row > 0 => Position::new(row - 1, col),
        Direction::Down if row < matrix.len() - 1 => Position::new(row + 1, col),
        Direction::Left if col > 0 => Position::new(row, col - 1),
        Direction::Right if col < matrix[row].len() - 1 => Position::new(row, col + 1),
        _ => return None,
    };
    Some(next)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_06_input_load_test() -> io::Result<()> {
        let sol = AoC2024_06::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2024_06_case_1() {
        let puzzle = make_puzzle();
        assert_eq!("41", puzzle.part_one());
    }

    #[test]
    fn aoc2024_06_case_2() {
        let puzzle = make_puzzle();
        assert_eq!("6", puzzle.part_two());
    }

    fn make_puzzle() -> AoC2024_06 {
        AoC2024_06::with_strings(&[
            "....#.....",
            ".........#",
            "..........",
            "..#.......",
            ".......#..",
            "..........",
            ".#..^.....",
            "........#.",
            "#.........",
            "......#...",
        ])
    }

    #[test]
    fn aoc2024_06_correctness() -> io::Result<()> {
        let sol = AoC2024_06::new()?;
        assert_eq!(sol.part_one(), "5239");
        assert_eq!(sol.part_two(), "1753");
        Ok(())
    }
}
