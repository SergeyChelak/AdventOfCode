use crate::solution::Solution;
use crate::utils::*;

use std::io;

use std::collections::HashSet;

pub struct AoC2015_03 {
    input: Vec<char>,
}

impl AoC2015_03 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: read_file_as_chars("input/aoc2015_03")?,
        })
    }
}

impl Solution for AoC2015_03 {
    fn part_one(&self) -> String {
        let mut coord = (0i32, 0i32);
        let mut set: HashSet<(i32, i32)> = HashSet::new();
        set.insert(coord);
        for ch in self.input.iter() {
            let mut x = 0;
            let mut y = 0;
            match ch {
                '>' => x = 1,
                '<' => x = -1,
                '^' => y = 1,
                'v' => y = -1,
                _ => panic!("unexpected value {ch}"),
            };
            coord.0 += x;
            coord.1 += y;
            set.insert(coord);
        }
        set.len().to_string()
    }

    fn part_two(&self) -> String {
        let mut pos_santa = (0i32, 0i32);
        let mut pos_robo = pos_santa;
        let mut set: HashSet<(i32, i32)> = HashSet::new();
        set.insert(pos_santa);
        let mut is_santa = true;
        for ch in self.input.iter() {
            let mut coord = if is_santa {
                &mut pos_santa
            } else {
                &mut pos_robo
            };
            match ch {
                '>' => coord.0 += 1,
                '<' => coord.0 -= 1,
                '^' => coord.1 += 1,
                'v' => coord.1 -= 1,
                _ => panic!("unexpected value {ch}"),
            };
            set.insert(*coord);
            is_santa = !is_santa;
        }
        set.len().to_string()
    }

    fn description(&self) -> String {
        "AoC 2015/Day 3: Perfectly Spherical Houses in a Vacuum".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc2015_03_input_load_test() -> io::Result<()> {
        let sol = AoC2015_03::new()?;
        assert!(sol.input.len() > 0);
        Ok(())
    }

    #[test]
    fn aoc2015_03_pt1_case1() {
        let result = AoC2015_03 { input: vec!['>'] }.part_one();
        assert_eq!(result, "2");
    }

    #[test]
    fn aoc2015_03_pt1_case2() {
        let result = AoC2015_03 {
            input: vec!['^', '>', 'v', '<'],
        }
        .part_one();
        assert_eq!(result, "4");
    }

    #[test]
    fn aoc2015_03_pt1_case3() {
        let result = AoC2015_03 {
            input: vec!['^', 'v', '^', 'v', '^', 'v', '^', 'v', '^', 'v'],
        }
        .part_one();
        assert_eq!(result, "2");
    }

    #[test]
    fn aoc2015_03_correctness() -> io::Result<()> {
        let sol = AoC2015_03::new()?;
        assert_eq!(sol.part_one(), "2081".to_string());
        assert_eq!(sol.part_two(), "2341".to_string());
        Ok(())
    }
}
