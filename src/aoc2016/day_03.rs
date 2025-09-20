use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Sides = [i32; 3];

pub struct AoC2016_03 {
    input: Vec<Sides>,
}

impl AoC2016_03 {
    pub fn new() -> io::Result<Self> {
        let sides = read_file_as_lines("input/aoc2016_03")?
            .iter()
            .map(|s| {
                let values = s
                    .split_whitespace()
                    .map(|val| val.parse::<i32>().expect("integer value expected"))
                    .collect::<Vec<i32>>();
                [values[0], values[1], values[2]]
            })
            .collect::<Vec<Sides>>();
        Ok(Self { input: sides })
    }
}

impl Solution for AoC2016_03 {
    fn part_one(&self) -> String {
        self.input
            .iter()
            .filter(|&sides| is_possible_triangle(sides))
            .count()
            .to_string()
    }

    fn part_two(&self) -> String {
        let len = self.input.len();
        if len.is_multiple_of(3) {
            let mut count = 0usize;
            for i in (0..len).step_by(3) {
                let a = self.input[i];
                let b = self.input[i + 1];
                let c = self.input[i + 2];
                for j in 0..3 {
                    if is_possible_triangle(&[a[j], b[j], c[j]]) {
                        count += 1;
                    }
                }
            }
            count.to_string()
        } else {
            "Incorrect number of lines".to_string()
        }
    }

    fn description(&self) -> String {
        "AoC 2016/Day 3: Squares With Three Sides".to_string()
    }
}

fn is_possible_triangle(sides: &[i32; 3]) -> bool {
    sides[0] + sides[1] > sides[2]
        && sides[0] + sides[2] > sides[1]
        && sides[1] + sides[2] > sides[0]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_03_input_load_test() -> io::Result<()> {
        let sol = AoC2016_03::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2016_03_correctness() -> io::Result<()> {
        let sol = AoC2016_03::new()?;
        assert_eq!(sol.part_one(), "983");
        assert_eq!(sol.part_two(), "1836");
        Ok(())
    }
}
