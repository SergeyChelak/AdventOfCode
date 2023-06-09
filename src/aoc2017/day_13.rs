use crate::solution::Solution;
use crate::utils::*;

use std::io;

#[derive(Clone, Copy, Debug)]
struct LevelParams {
    size: usize,
    ptr: usize,
    is_forward: bool,
}

impl LevelParams {
    fn new() -> Self {
        Self {
            size: 0,
            ptr: 0,
            is_forward: true,
        }
    }

    fn teak(&mut self) {
        if self.size == 0 {
            return;
        }
        if self.is_forward {
            self.ptr += 1;
        } else {
            self.ptr -= 1;
        }
        if self.ptr == 0 {
            self.is_forward = true;
        } else if self.ptr == self.size - 1 {
            self.is_forward = false;
        }
    }

    fn is_on_top(&self) -> bool {
        if self.size == 0 {
            return false;
        }
        self.ptr == 0
    }
}

pub struct AoC2017_13 {
    input: Vec<(usize, usize)>,
}

impl AoC2017_13 {
    pub fn new() -> io::Result<Self> {
        let input = read_file_as_lines("input/aoc2017_13")?
            .iter()
            .map(|s| {
                let (depth, range) = s
                    .split_once(": ")
                    .expect("Input strings should be separated with colon");
                let depth = depth.parse::<usize>().expect("Depth should be integer");
                let range = range.parse::<usize>().expect("Range should be integer");
                (depth, range)
            })
            .collect::<Vec<(usize, usize)>>();
        Ok(Self { input })
    }

    fn levels(&self) -> Vec<LevelParams> {
        let count = self
            .input
            .iter()
            .map(|x| x.0)
            .max()
            .expect("Input shouldn't be empty");
        let mut arr = vec![LevelParams::new(); count + 1];
        self.input.iter().for_each(|(idx, size)| {
            arr[*idx].size = *size;
        });
        arr
    }
}

impl Solution for AoC2017_13 {
    fn part_one(&self) -> String {
        let mut levels = self.levels();
        let mut severity = 0;
        for i in 0..levels.len() {
            if levels[i].is_on_top() {
                let size = levels[i].size;
                severity += i * size;
            }
            levels.iter_mut().for_each(|param| param.teak());
        }
        severity.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 13: Packet Scanners".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_13_input_load_test() -> io::Result<()> {
        let sol = AoC2017_13::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_13_example() {
        let sol = AoC2017_13 {
            input: vec![(0, 3), (1, 2), (4, 4), (6, 4)],
        };
        assert_eq!(sol.part_one(), "24")
    }

    #[test]
    fn aoc2017_13_correctness() -> io::Result<()> {
        let sol = AoC2017_13::new()?;
        assert_eq!(sol.part_one(), "748");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
