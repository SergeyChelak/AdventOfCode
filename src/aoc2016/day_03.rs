use crate::solution::Solution;
use crate::utils::*;

use std::io;

struct Sides(i32, i32, i32);

impl Sides {
    fn is_possible_triangle(&self) -> bool {
        self.0 + self.1 > self.2 &&
        self.0 + self.2 > self.1 &&
        self.1 + self.2 > self.0
    }
}

pub struct AoC2016_03 {
    input: Vec<Sides>
}

impl AoC2016_03 {
    pub fn new() -> io::Result<Self> {
        let sides = read_file_as_lines("input/aoc2016_03")?
            .iter()
            .map(|s| {
                let values = s.split_whitespace()
                    .map(|val| val.parse::<i32>().expect("integer value expected"))
                    .collect::<Vec<i32>>();
                Sides(values[0], values[1], values[2])
            })
            .collect::<Vec<Sides>>();
        Ok(Self {
            input: sides
        })
    }
}

impl Solution for AoC2016_03 {
    fn part_one(&self) -> String {
        self.input.iter()
            .filter(|&sides| sides.is_possible_triangle())
            .count()
            .to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2016/Day 3: Squares With Three Sides".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_03_input_load_test() -> io::Result<()> {
        let sol = AoC2016_03::new()?;
        assert!(sol.input.len() > 0);
        Ok(())
    }

    #[test]
    fn aoc2016_03_correctness() -> io::Result<()> {
        let sol = AoC2016_03::new()?;
        assert_eq!(sol.part_one(), "983");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}