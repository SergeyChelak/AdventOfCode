use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2020_01 {
    input: Vec<usize>,
}

impl AoC2020_01 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2020_01")?;
        Ok(Self::parse(&lines))
    }

    fn parse<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(|x| x.parse::<usize>().expect("Invalid value in input file"))
            .collect();
        Self { input }
    }
}

impl Solution for AoC2020_01 {
    fn part_one(&self) -> String {
        for (i, a) in self.input.iter().enumerate() {
            for b in self.input.iter().skip(i) {
                if *a + *b != 2020 {
                    continue;
                }
                return (*a * *b).to_string();
            }
        }
        not_found()
    }

    fn part_two(&self) -> String {
        for (i, a) in self.input.iter().enumerate() {
            for (j, b) in self.input.iter().enumerate().skip(i) {
                for c in self.input.iter().skip(j) {
                    if *a + *b + *c != 2020 {
                        continue;
                    }
                    return (*a * *b * *c).to_string();
                }
            }
        }
        not_found()
    }

    fn description(&self) -> String {
        "Day 1: Report Repair".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_01_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2020_01_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "468051");
        Ok(())
    }

    #[test]
    fn aoc2020_01_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "272611658");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_01> {
        AoC2020_01::new()
    }
}
