use crate::solution::Solution;

use std::{fs::read_to_string, io};

type Value = usize;

pub struct AoC2017_10 {
    lengths: Vec<Value>,
}

impl AoC2017_10 {
    pub fn new() -> io::Result<Self> {
        let lengths = read_to_string("input/aoc2017_10")?
            .trim()
            .split(',')
            .map(|s| {
                s.parse::<Value>()
                    .expect("Integer values expected in the input")
            })
            .collect::<Vec<Value>>();
        Ok(Self { lengths })
    }
}

impl Solution for AoC2017_10 {
    fn part_one(&self) -> String {
        let arr = knot_hash(256, &self.lengths);
        (arr[0] * arr[1]).to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 10: Knot Hash".to_string()
    }
}

fn knot_hash(size: usize, length: &[usize]) -> Vec<usize> {
    let mut arr = vec![0usize; size]
        .iter_mut()
        .enumerate()
        .map(|(idx, _)| idx)
        .collect::<Vec<usize>>();
    let mut pos = 0usize;
    let mut skip_size = 0usize;
    for &len in length {
        for i in 0..len / 2 {
            let a = (pos + i) % size;
            let b = (pos + len - i - 1) % size;
            arr.swap(a, b);
        }
        pos += len + skip_size;
        skip_size += 1;
    }
    arr
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_10_input_load_test() -> io::Result<()> {
        let sol = AoC2017_10::new()?;
        assert!(!sol.lengths.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_10_example1() {
        let arr = knot_hash(5, &[3, 4, 1, 5]);
        assert_eq!(arr, [3, 4, 2, 1, 0]);
    }

    #[test]
    fn aoc2017_10_correctness() -> io::Result<()> {
        let sol = AoC2017_10::new()?;
        assert_eq!(sol.part_one(), "54675");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
