use crate::solution::Solution;
use crate::utils::*;

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

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 3: Gear Ratios".to_string()
    }
}

fn has_adjacent_symbol(matrix: &[Vec<char>], row: usize, col: usize) -> bool {
    let up = row > 0;
    let down = row < matrix.len() - 1;
    let left = col > 0;
    let right = col < matrix[row].len() - 1;

    let mut chars = Vec::with_capacity(8);
    if up {
        chars.push(matrix[row - 1][col]);
    }
    if down {
        chars.push(matrix[row + 1][col]);
    }
    if left {
        chars.push(matrix[row][col - 1]);
    }
    if right {
        chars.push(matrix[row][col + 1]);
    }
    if up && left {
        chars.push(matrix[row - 1][col - 1]);
    }
    if up && right {
        chars.push(matrix[row - 1][col + 1]);
    }
    if down && left {
        chars.push(matrix[row + 1][col - 1]);
    }
    if down && right {
        chars.push(matrix[row + 1][col + 1]);
    }
    chars
        .iter()
        .filter(|ch| !matches!(ch, '.' | '0'..='9'))
        .count()
        > 0
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
        let input = [
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
        .collect::<Vec<_>>();
        let sol = AoC2023_03 { input };
        assert_eq!(sol.part_one(), "4361");
    }

    #[test]
    fn aoc2023_03_correctness() -> io::Result<()> {
        let sol = AoC2023_03::new()?;
        assert_eq!(sol.part_one(), "527364");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
