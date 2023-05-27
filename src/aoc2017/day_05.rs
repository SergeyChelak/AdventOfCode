use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2017_05 {
    jumps: Vec<i32>,
}

impl AoC2017_05 {
    pub fn new() -> io::Result<Self> {
        let jumps = read_file_as_lines("input/aoc2017_05")?
            .iter()
            .map(|s| s.parse::<i32>().expect("Integer value is expected"))
            .collect::<Vec<i32>>();
        Ok(Self {
            jumps
        })
    }
}

impl Solution for AoC2017_05 {
    fn part_one(&self) -> String {
        let mut ptr = 0usize;
        let mut jumps = self.jumps.clone();
        let len = jumps.len();
        let mut steps = 0;
        loop {
            let val = jumps[ptr];
            jumps[ptr] += 1;
            steps += 1;
            if val > 0 {
                ptr += val as usize;
                if ptr >= len {
                    break;
                }
            } else if val < 0 {
                let val = (-val) as usize;
                if val > ptr {
                    break;
                }
                ptr -= val;
            }
        }
        steps.to_string()
    }

    fn part_two(&self) -> String {
        let mut ptr = 0usize;
        let mut jumps = self.jumps.clone();
        let len = jumps.len();
        let mut steps = 0;
        loop {
            let val = jumps[ptr];
            if val > 2 {
                jumps[ptr] -= 1;
            } else {
                jumps[ptr] += 1;
            }
            steps += 1;
            if val > 0 {
                ptr += val as usize;
                if ptr >= len {
                    break;
                }
            } else if val < 0 {
                let val = (-val) as usize;
                if val > ptr {
                    break;
                }
                ptr -= val;
            }
        }
        steps.to_string()
    }

    fn description(&self) -> String {
        "AoC 2017/Day 5: A Maze of Twisty Trampolines, All Alike".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_05_input_load_test() -> io::Result<()> {
        let sol = AoC2017_05::new()?;
        assert!(!sol.jumps.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_05_example1() {
        let sol = AoC2017_05 {
            jumps: vec![0, 3,  0,  1,  -3]
        };
        assert_eq!(sol.part_one(), "5");
    }

    #[test]
    fn aoc2017_05_correctness() -> io::Result<()> {
        let sol = AoC2017_05::new()?;
        assert_eq!(sol.part_one(), "343467");
        assert_eq!(sol.part_two(), "24774780");
        Ok(())
    }
}