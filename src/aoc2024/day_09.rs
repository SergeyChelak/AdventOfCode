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

    fn part_two(&self) -> String {
        let (mut blocks, mut info) = extract_with_info(&self.input);
        compact_whole_file(&mut blocks, &mut info);
        checksum(&blocks).to_string()
    }

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

#[derive(Debug, Copy, Clone, PartialEq)]
struct BlockInfo {
    block: Block,
    start: usize,
    len: usize,
}

impl BlockInfo {
    fn is_free(&self) -> bool {
        self.block == Block::Free
    }
}

fn extract(s: &str) -> Vec<Block> {
    extract_with_info(s).0
}

fn extract_with_info(s: &str) -> (Vec<Block>, Vec<BlockInfo>) {
    let mut blocks: Vec<Block> = Vec::new();
    let mut blocks_info: Vec<BlockInfo> = Vec::new();
    let mut id: FileID = 0;
    let mut is_file = true;
    for ch in s.trim().chars() {
        let val = ch.to_digit(10).expect("Only digit char are acceptable");
        let block = if is_file {
            Block::File(id)
        } else {
            Block::Free
        };
        {
            let info = BlockInfo {
                block,
                start: blocks.len(),
                len: val as usize,
            };
            blocks_info.push(info);
        }
        for _ in 0..val {
            blocks.push(block);
        }
        if is_file {
            id += 1;
        }
        is_file = !is_file;
    }
    (blocks, blocks_info)
}

fn compact_whole_file(blocks: &mut [Block], block_info: &mut Vec<BlockInfo>) {
    loop {
        // dump(&blocks);

        let mut matched: Option<(usize, usize)> = None;

        for (occupy_idx, len) in block_info
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, val)| !val.is_free())
            .map(|(idx, val)| (idx, val.len))
        {
            let pos = block_info
                .iter()
                .enumerate()
                .filter(|(idx, val)| val.is_free() && *idx < occupy_idx)
                .find(|(_, val)| val.len >= len);
            let Some((free_idx, _)) = pos else {
                continue;
            };
            matched = Some((free_idx, occupy_idx));
            break;
        }

        // no more movements
        let Some((free_idx, occupy_idx)) = matched else {
            break;
        };

        // update blocks
        {
            let occupy = block_info[occupy_idx];
            assert!(!occupy.is_free());
            let free = block_info[free_idx];
            assert!(free.is_free());
            for i in 0..occupy.len {
                blocks.swap(occupy.start + i, free.start + i);
            }
        }

        // update existing block info
        {
            block_info[free_idx].len -= block_info[occupy_idx].len;
            block_info[free_idx].start += block_info[occupy_idx].len;

            let element = BlockInfo {
                start: block_info[free_idx].start,
                ..block_info[occupy_idx]
            };
            block_info.insert(free_idx, element);
        }

        // merge free space
        // let mut ptr = occupy_idx + 1;
        // block_info[ptr].block = Block::Free;
        // loop {
        //     if ptr > 0 {
        //         let idx = ptr - 1;
        //         if block_info[idx].is_free() {
        //             block_info[idx].len += block_info[ptr].len;
        //             block_info.remove(ptr);
        //             ptr = idx;
        //             continue;
        //         }
        //     }
        //     if ptr < block_info.len() - 1 {
        //         let idx = ptr + 1;
        //         if block_info[idx].is_free() {
        //             block_info[ptr].len += block_info[idx].len;
        //             block_info.remove(idx);
        //             continue;
        //         }
        //     }
        //     break;
        // }
    }
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
        let sol = make_puzzle();
        assert_eq!("1928", sol.part_one());
    }

    #[test]
    fn aoc2024_09_case_2() {
        let sol = make_puzzle();
        assert_eq!("2858", sol.part_two());
    }

    fn make_puzzle() -> AoC2024_09 {
        AoC2024_09 {
            input: "2333133121414131402".to_string(),
        }
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
        assert_eq!(sol.part_two(), "6511178035564");
        Ok(())
    }
}
