use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

pub struct AoC2023_03 {
    input: Vec<Vec<char>>,
}

impl AoC2023_03 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_03")?;
        let input = lines
            .iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Ok(Self { input })
    }
}

impl Solution for AoC2023_03 {
    fn part_one(&self) -> String {
        let mut total = 0;
        for (row, arr) in self.input.iter().enumerate() {
            let mut acc = 0;
            let mut has_symbol = false;
            for (col, val) in arr.iter().enumerate() {
                if val.is_numeric() {
                    acc = acc * 10 + val.to_digit(10).expect("should be a digit");
                    has_symbol |= has_adjacent_symbol(&self.input, row, col);
                    continue;
                }
                if acc > 0 {
                    if has_symbol {
                        total += acc;
                    }
                    acc = 0;
                    has_symbol = false;
                }
            }
            if has_symbol && acc > 0 {
                total += acc;
            }
        }
        total.to_string()
    }

    fn part_two(&self) -> String {
        let mut total = 0;
        for (row, arr) in self.input.iter().enumerate() {
            for (col, val) in arr.iter().enumerate() {
                if *val != '*' {
                    continue;
                }
                let positions = adjacent_digits(&self.input, row, col);
                if positions.len() == 2 {
                    total += positions[0] * positions[1];
                }
            }
        }
        total.to_string()
    }

    fn description(&self) -> String {
        "AoC 2023/Day 3: Gear Ratios".to_string()
    }
}

fn has_adjacent_symbol(matrix: &[Vec<char>], row: usize, col: usize) -> bool {
    adjacent_positions(matrix, row, col)
        .iter()
        .map(|(r, c)| matrix[*r][*c])
        .filter(|ch| !matches!(ch, '.' | '0'..='9'))
        .count()
        > 0
}

fn adjacent_digits(matrix: &[Vec<char>], row: usize, col: usize) -> Vec<u32> {
    adjacent_positions(matrix, row, col)
        .into_iter()
        .filter(|(r, c)| matrix[*r][*c].is_numeric())
        .map(|(r, c)| (r, calc_digit_range(&matrix[r], c)))
        .collect::<HashSet<_>>()
        .iter()
        .map(|(r, (begin, end))| {
            String::from_iter(&matrix[*r][*begin..=*end])
                .parse::<u32>()
                .expect("Failed to parse adjacent number")
        })
        .collect::<Vec<u32>>()
}

fn adjacent_positions(matrix: &[Vec<char>], row: usize, col: usize) -> Vec<(usize, usize)> {
    let up = row > 0;
    let down = row < matrix.len() - 1;
    let left = col > 0;
    let right = col < matrix[row].len() - 1;

    let mut positions = Vec::with_capacity(8);
    if up {
        positions.push((row - 1, col));
    }
    if down {
        positions.push((row + 1, col));
    }
    if left {
        positions.push((row, col - 1));
    }
    if right {
        positions.push((row, col + 1));
    }
    if up && left {
        positions.push((row - 1, col - 1));
    }
    if up && right {
        positions.push((row - 1, col + 1));
    }
    if down && left {
        positions.push((row + 1, col - 1));
    }
    if down && right {
        positions.push((row + 1, col + 1));
    }
    positions
}

fn calc_digit_range(input: &[char], pos: usize) -> (usize, usize) {
    let mut begin = pos;
    while begin > 0 {
        if input[begin - 1].is_numeric() {
            begin -= 1;
        } else {
            break;
        }
    }
    let mut end = pos;
    while end < input.len() - 1 {
        if input[end + 1].is_numeric() {
            end += 1;
        } else {
            break;
        }
    }
    (begin, end)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_03_input_load_test() -> io::Result<()> {
        let sol = AoC2023_03::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_03_ex1() {
        let sol = AoC2023_03 { input: input() };
        assert_eq!(sol.part_one(), "4361");
    }

    #[test]
    fn aoc2023_03_ex2() {
        let sol = AoC2023_03 { input: input() };
        assert_eq!(sol.part_two(), "467835");
    }

    fn input() -> Vec<Vec<char>> {
        [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
    }

    #[test]
    fn aoc2023_03_correctness() -> io::Result<()> {
        let sol = AoC2023_03::new()?;
        assert_eq!(sol.part_one(), "527364");
        assert_eq!(sol.part_two(), "79026871");
        Ok(())
    }
}
