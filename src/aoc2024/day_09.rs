use crate::solution::Solution;

use std::fs::read_to_string;
use std::io;

pub struct AoC2024_09 {
    input: String,
}

impl AoC2024_09 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2024_09")?;
        Ok(Self { input })
    }
}

impl Solution for AoC2024_09 {
    fn part_one(&self) -> String {
        let mut blocks = extract(&self.input);
        compact(&mut blocks);
        checksum(&blocks).to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "2024/Day 9: Disk Fragmenter".to_string()
    }
}

type FileID = u32;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Block {
    Free,
    File(FileID),
}

fn extract(s: &str) -> Vec<Block> {
    let mut result = Vec::new();
    let mut id: FileID = 0;
    let mut is_file = true;
    for ch in s.trim().chars() {
        let val = ch.to_digit(10).expect("Only digit char are acceptable");
        let block = if is_file {
            Block::File(id)
        } else {
            Block::Free
        };
        for _ in 0..val {
            result.push(block);
        }
        if is_file {
            id += 1;
        }
        is_file = !is_file;
    }
    result
}

fn compact(arr: &mut [Block]) {
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left < right {
        if arr[left] == Block::Free && arr[right] != Block::Free {
            arr.swap(left, right);
            left += 1;
            right -= 1;
            continue;
        }
        if arr[left] != Block::Free {
            left += 1;
        }
        if arr[right] == Block::Free {
            right -= 1;
        }
    }
}

fn checksum(arr: &[Block]) -> usize {
    arr.iter()
        .enumerate()
        .map(|(index, val)| match val {
            Block::Free => 0,
            Block::File(id) => index * *id as usize,
        })
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_09_input_load_test() -> io::Result<()> {
        let sol = AoC2024_09::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2024_09_case_1() {
        let sol = AoC2024_09 {
            input: "2333133121414131402".to_string(),
        };
        assert_eq!("1928", sol.part_one());
    }

    #[test]
    fn aoc2024_09_extract() {
        let output = extract("12345");
        let arr = [
            Block::File(0),
            Block::Free,
            Block::Free,
            Block::File(1),
            Block::File(1),
            Block::File(1),
            Block::Free,
            Block::Free,
            Block::Free,
            Block::Free,
            Block::File(2),
            Block::File(2),
            Block::File(2),
            Block::File(2),
            Block::File(2),
        ];
        assert_eq!(output.len(), arr.len());
        output
            .iter()
            .zip(arr.iter())
            .for_each(|(a, b)| assert_eq!(a, b));
    }

    #[test]
    fn aoc2024_09_correctness() -> io::Result<()> {
        let sol = AoC2024_09::new()?;
        assert_eq!(sol.part_one(), "6471961544878");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
