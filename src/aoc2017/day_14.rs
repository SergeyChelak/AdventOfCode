use crate::solution::Solution;

use std::io;

use super::knot_hash::KnotHashable;

pub struct AoC2017_14 {
    input: String,
}

impl AoC2017_14 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: "oundnydw".to_string(),
        })
    }
}

impl Solution for AoC2017_14 {
    fn part_one(&self) -> String {
        (0..128)
            .fold(0, |acc, i| {
                acc + format!("{}-{i}", self.input)
                    .knot_hash()
                    .chars()
                    .filter_map(|ch| ch.to_digit(16))
                    .map(|x| {
                        (0..4)
                            .map(|offset| x & (0x1 << offset))
                            .map(|val| if val == 0 { 0 } else { 1 })
                            .sum::<u32>()
                    })
                    .sum::<u32>()
            })
            .to_string()
    }

    fn part_two(&self) -> String {
        let mut matrix = make_matrix(&self.input);
        let mut regions = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 0 {
                    continue;
                }
                fill_region(&mut matrix, i, j);
                regions += 1;
            }
        }
        regions.to_string()
    }

    fn description(&self) -> String {
        "AoC 2017/Day 14: Disk Defragmentation".to_string()
    }
}

type Matrix = Vec<Vec<i32>>;

fn make_matrix(input: &str) -> Matrix {
    (0..128)
        .map(|i| format!("{}-{i}", input))
        .map(|s| s.knot_hash())
        .map(|hash| hash2binary(&hash))
        .collect()
}

fn fill_region(matrix: &mut Matrix, row: usize, col: usize) {
    if matrix[row][col] == 0 {
        return;
    }
    matrix[row][col] = 0;
    if row > 0 {
        fill_region(matrix, row - 1, col);
    }
    if row < matrix.len() - 1 {
        fill_region(matrix, row + 1, col);
    }
    if col > 0 {
        fill_region(matrix, row, col - 1);
    }
    if col < matrix[row].len() - 1 {
        fill_region(matrix, row, col + 1);
    }
}

fn hash2binary(s: &str) -> Vec<i32> {
    let mut arr = Vec::new();
    s.chars().filter_map(|ch| ch.to_digit(16)).for_each(|x| {
        for offset in (0..4).rev() {
            let val = if x & (1 << offset) > 0 { 1 } else { 0 };
            arr.push(val);
        }
    });
    arr
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_14_example1() {
        let sol = AoC2017_14 {
            input: "flqrgnkx".to_string(),
        };
        assert_eq!(sol.part_one(), "8108");
    }

    #[test]
    fn aoc2017_14_example2() {
        let sol = AoC2017_14 {
            input: "flqrgnkx".to_string(),
        };
        assert_eq!(sol.part_two(), "1242");
    }

    #[test]
    fn aoc2017_14_correctness() -> io::Result<()> {
        let sol = AoC2017_14::new()?;
        assert_eq!(sol.part_one(), "8106");
        assert_eq!(sol.part_two(), "1164");
        Ok(())
    }
}
