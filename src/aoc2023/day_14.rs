use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2023_14 {
    input: Vec<Vec<char>>,
}

impl AoC2023_14 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_14")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let input = lines
            .iter()
            .map(|row| row.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2023_14 {
    fn part_one(&self) -> String {
        let map = slide_north(&self.input);
        total_load(&map).to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 14: Parabolic Reflector Dish".to_string()
    }
}

fn slide_north(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = input.clone();
    for row in 1..input.len() {
        for col in 0..input[row].len() {
            if result[row][col] != 'O' {
                continue;
            }
            let mut lift = 0;
            for k in 1..=row {
                if result[row - k][col] != '.' {
                    break;
                }
                lift = k;
            }
            if lift > 0 {
                result[row - lift][col] = 'O';
                result[row][col] = '.'
            }
        }
    }
    result
}

fn _dump(input: &[Vec<char>]) {
    for row in input {
        for ch in row {
            print!("{ch}");
        }
        println!();
    }
}

fn total_load(input: &[Vec<char>]) -> usize {
    let mut total = 0;
    for col in 0..input[0].len() {
        let len = input.len();
        for row in 0..len {
            if input[row][col] == 'O' {
                total += len - row;
            }
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_14_input_load_test() -> io::Result<()> {
        let sol = AoC2023_14::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_14_ex1() {
        let input = [
            "O....#....",
            "O.OO#....#",
            ".....##...",
            "OO.#O....O",
            ".O.....O#.",
            "O.#..O.#.#",
            "..O..#O..O",
            ".......O..",
            "#....###..",
            "#OO..#....",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let puzzle = AoC2023_14::with_lines(&input);
        assert_eq!(puzzle.part_one(), "136");
    }

    #[test]
    fn aoc2023_14_correctness() -> io::Result<()> {
        let sol = AoC2023_14::new()?;
        assert_eq!(sol.part_one(), "109385");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
