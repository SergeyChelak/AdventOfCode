use crate::solution::Solution;

use std::fs::read_to_string;
use std::io;

pub struct AoC2023_13 {
    blocks: Vec<String>,
}

impl AoC2023_13 {
    pub fn new() -> io::Result<Self> {
        let contents = read_to_string("input/aoc2023_13")?;
        Ok(Self::with_chars(&contents))
    }

    fn with_chars(contents: &str) -> Self {
        let blocks = contents
            .split("\n\n")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        Self { blocks }
    }

    fn calculate(&self, reflection_line: &dyn Fn(&[Vec<char>]) -> usize) -> usize {
        let mut total_above = 0usize;
        let mut total_left = 0usize;
        for block in &self.blocks {
            let lines = block.split_whitespace().collect::<Vec<_>>();
            let chars = lines
                .iter()
                .map(|s| s.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();
            total_above += reflection_line(&chars);
            total_left += reflection_line(&transpose(&chars));
        }
        total_left + 100 * total_above
    }
}

impl Solution for AoC2023_13 {
    fn part_one(&self) -> String {
        self.calculate(&mirror_reflection_line).to_string()
    }

    fn part_two(&self) -> String {
        self.calculate(&smudge_reflection_line).to_string()
    }

    fn description(&self) -> String {
        "AoC 2023/Day 13: Point of Incidence".to_string()
    }
}

fn mirror_reflection_line(block: &[Vec<char>]) -> usize {
    for i in 1..block.len() {
        let above = block[..i].iter().rev().cloned().collect::<Vec<_>>();
        let below = &block[i..];
        let len = above.len().min(below.len());
        let above = &above[..len];
        let below = &below[..len];
        if above == below {
            return i;
        }
    }
    0
}

fn smudge_reflection_line(block: &[Vec<char>]) -> usize {
    for i in 1..block.len() {
        let above = block[..i].iter().rev().cloned().collect::<Vec<_>>();
        let below = &block[i..];

        let sum = above
            .iter()
            .zip(below.iter())
            .map(|(vec_a, vec_b)| {
                vec_a
                    .iter()
                    .zip(vec_b.iter())
                    .filter(|(a, b)| **a != **b)
                    .count()
            })
            .sum::<usize>();
        if sum == 1 {
            return i;
        }
    }
    0
}

#[allow(clippy::needless_range_loop)]
fn transpose(inp: &[Vec<char>]) -> Vec<Vec<char>> {
    let inp_rows = inp.len();
    let inp_cols = inp[0].len();
    let mut result = vec![vec!['\0'; inp_rows]; inp_cols];
    for row in 0..inp_rows {
        for col in 0..inp_cols {
            result[col][row] = inp[row][col];
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_13_input_load_test() -> io::Result<()> {
        let sol = AoC2023_13::new()?;
        assert!(!sol.blocks.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_13_ex1() {
        let input = [
            "#.##..##.",
            "..#.##.#.",
            "##......#",
            "##......#",
            "..#.##.#.",
            "..##..##.",
            "#.#.##.#.",
            "",
            "#...##..#",
            "#....#..#",
            "..##..###",
            "#####.##.",
            "#####.##.",
            "..##..###",
            "#....#..#",
        ]
        .join("\n");
        let puzzle = AoC2023_13::with_chars(&input);
        assert_eq!(puzzle.part_one(), "405")
    }

    #[test]
    fn aoc2023_13_correctness() -> io::Result<()> {
        let sol = AoC2023_13::new()?;
        assert_eq!(sol.part_one(), "34772");
        assert_eq!(sol.part_two(), "35554");
        Ok(())
    }
}
