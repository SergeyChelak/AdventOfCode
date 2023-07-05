use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

type Int = i64;

pub struct AoC2018_01 {
    input: Vec<Int>,
}

impl AoC2018_01 {
    pub fn new() -> io::Result<Self> {
        let (input, _): (Vec<_>, Vec<_>) = read_file_as_lines("input/aoc2018_01")?
            .iter()
            .map(|s| s.parse::<Int>())
            .partition(Result::is_ok);
        let input = input.into_iter().map(Result::unwrap).collect();
        Ok(Self { input })
    }
}

impl Solution for AoC2018_01 {
    fn part_one(&self) -> String {
        self.input.iter().sum::<Int>().to_string()
    }

    fn part_two(&self) -> String {
        let mut set: HashSet<Int> = HashSet::new();
        let mut freq = 0;
        set.insert(0);
        'main: loop {
            for x in &self.input {
                // print!("{freq} + {x}");
                freq += x;
                if set.contains(&freq) {
                    break 'main;
                }
                // println!(" = {freq}");
                set.insert(freq);
            }
        }
        freq.to_string()
    }

    fn description(&self) -> String {
        "AoC 2018/Day 1: Chronal Calibration".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_01_input_load_test() -> io::Result<()> {
        let sol = AoC2018_01::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_01_correctness() -> io::Result<()> {
        let sol = AoC2018_01::new()?;
        assert_eq!(sol.part_one(), "543");
        assert_eq!(sol.part_two(), "621");
        Ok(())
    }
}
