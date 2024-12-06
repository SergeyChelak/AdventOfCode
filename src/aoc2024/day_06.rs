use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

type Position = Position2<usize>;

pub struct AoC2024_06 {
    input: Vec<Vec<char>>,
    start_position: Position,
}

impl AoC2024_06 {
    pub fn new() -> io::Result<Self> {
        let arr = read_file_as_lines("input/aoc2024_06")?;
        Ok(Self::with_strings(&arr))
    }

    fn with_strings(arr: &[String]) -> Self {
        let mut input = Vec::with_capacity(arr.len());
        let mut start: Option<Position> = None;

        for (row, s) in arr.iter().enumerate() {
            let chars = s.chars().collect::<Vec<_>>();
            if let Some(col) = chars.iter().position(|x| *x == '^') {
                start = Some(Position { row, col });
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
            let Position { row, col } = position;
            let next = match direction {
                Direction::Up if row > 0 => Position::new(row - 1, col),
                Direction::Down if row < self.input.len() - 1 => Position::new(row + 1, col),
                Direction::Left if col > 0 => Position::new(row, col - 1),
                Direction::Right if col < self.input[row].len() - 1 => Position::new(row, col + 1),
                _ => break,
            };
            if self.input[next.row][next.col] == '#' {
                direction = direction.turn_right();
                continue;
            }
            position = next;
        }
        visited.len().to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "2024/Day 6: Guard Gallivant".to_string()
    }
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
        let puzzle = AoC2024_06::with_strings(
            &[
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
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
        );
        assert_eq!("41", puzzle.part_one());
    }

    #[test]
    fn aoc2024_06_correctness() -> io::Result<()> {
        let sol = AoC2024_06::new()?;
        assert_eq!(sol.part_one(), "5239");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
