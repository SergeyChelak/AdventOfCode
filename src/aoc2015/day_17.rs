use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2015_17 {
    values: Vec<i32>,
    target: i32,
}

impl AoC2015_17 {
    pub fn new() -> io::Result<Self> {
        let values = read_file_as_lines("input/aoc2015_17")?
            .iter()
            .map(|s| s.parse::<i32>().ok().expect("non integer value found"))
            .collect::<Vec<i32>>();
        Ok(Self {
            values,
            target: 150
        })
    }

    fn calc(&self, idx: usize, sum: i32, count: &mut i32) {
        if sum == self.target {
            *count += 1;
        } else if sum < self.target && idx < self.values.len() {
            for i in idx..self.values.len() {
                self.calc(i + 1, sum + self.values[i], count);
            }
        }
    }

    fn calc_min(&self, idx: usize, sum: i32, depth: i32, min_depth: &mut i32, count: &mut i32) {
        if sum == self.target {
            if depth == *min_depth {
                *count += 1;
            } else if depth < *min_depth {
                *min_depth = depth;
                *count = 1;
            }
        } else if sum < self.target && idx < self.values.len() {
            for i in idx..self.values.len() {
                self.calc_min(i + 1, sum + self.values[i], depth + 1, min_depth, count);
            }
        }

    }
}

impl Solution for AoC2015_17 {
    fn part_one(&self) -> String {
        let mut count = 0;
        self.calc(0, 0, &mut count);
        count.to_string()
    }

    fn part_two(&self) -> String {
        let mut count = 0;
        let mut min_depth = i32::MAX;
        self.calc_min(0, 0, 0, &mut min_depth, &mut count);
        count.to_string()
    }

    fn description(&self) -> String {
        "AoC 2015/Day 17: No Such Thing as Too Much".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_17_input_load_test() -> io::Result<()> {
        let sol = AoC2015_17::new()?;
        assert_eq!(sol.values.len(), 20);
        Ok(())
    }

    #[test]
    fn aoc2015_17_correctness() -> io::Result<()> {
        let sol = AoC2015_17::new()?;
        assert_eq!(sol.part_one(), "4372");
        assert_eq!(sol.part_two(), "4");
        Ok(())
    }
}