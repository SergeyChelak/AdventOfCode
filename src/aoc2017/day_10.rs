use crate::solution::Solution;

use std::{fs::read_to_string, io};

pub struct AoC2017_10 {
    input: String,
}

impl AoC2017_10 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2017_10")?.trim().to_string();
        Ok(Self { input })
    }
}

impl Solution for AoC2017_10 {
    fn part_one(&self) -> String {
        let inp = self
            .input
            .split(',')
            .map(|s| {
                s.parse::<u8>()
                    .expect("Integer values expected in the input")
            })
            .collect::<Vec<u8>>();
        let arr = knot_hash(256, 1, &inp);
        (arr[0] as u32 * arr[1] as u32).to_string()
    }

    fn part_two(&self) -> String {
        hash(&self.input)
    }

    fn description(&self) -> String {
        "AoC 2017/Day 10: Knot Hash".to_string()
    }
}

fn knot_hash(size: usize, rounds: usize, input: &[u8]) -> Vec<u8> {
    let mut arr = vec![0u8; size]
        .iter_mut()
        .enumerate()
        .map(|(idx, _)| idx as u8)
        .collect::<Vec<u8>>();
    let mut pos = 0usize;
    let mut skip_size = 0usize;
    for _ in 0..rounds {
        for len in input {
            let len = *len as usize;
            for i in 0..len / 2 {
                let a = (pos + i) % size;
                let b = (pos + len - i - 1) % size;
                arr.swap(a, b);
            }
            pos += len + skip_size;
            skip_size += 1;
        }
    }
    arr
}

fn hash(s: &str) -> String {
    let mut input = s.bytes().collect::<Vec<u8>>();
    let mut offset = vec![17u8, 31, 73, 47, 23];
    input.append(&mut offset);
    knot_hash(256, 64, &input)
        .chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, val| acc ^ val))
        .map(|b| format!("{b:02x}"))
        .collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_10_input_load_test() -> io::Result<()> {
        let sol = AoC2017_10::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_10_example1() {
        let arr = knot_hash(5, 1, &[3, 4, 1, 5]);
        assert_eq!(arr, [3, 4, 2, 1, 0]);
    }

    #[test]
    fn aoc2017_10_example2() {
        assert_eq!(hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }

    #[test]
    fn aoc2017_10_correctness() -> io::Result<()> {
        let sol = AoC2017_10::new()?;
        assert_eq!(sol.part_one(), "54675");
        assert_eq!(sol.part_two(), "a7af2706aa9a09cf5d848c1e6605dd2a");
        Ok(())
    }
}
