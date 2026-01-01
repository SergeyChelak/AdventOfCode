use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = isize;

pub struct AoC2021_01 {
    input: Vec<Int>,
}

impl AoC2021_01 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2021_01")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(|x| x.parse::<Int>())
            .collect::<Result<Vec<_>, _>>();
        let Ok(input) = input else {
            panic!("Invalid input format");
        };
        Self { input }
    }
}

impl Solution for AoC2021_01 {
    fn part_one(&self) -> String {
        increase_count(&self.input).to_string()
    }

    fn part_two(&self) -> String {
        let arr = self
            .input
            .windows(3)
            .map(|w| w.iter().sum())
            .collect::<Vec<_>>();
        increase_count(&arr).to_string()
    }

    fn description(&self) -> String {
        "Day 1: Sonar Sweep".to_string()
    }
}

fn increase_count(arr: &[Int]) -> usize {
    arr.windows(2).filter(|w| w[1] - w[0] > 0).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_01_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2021_01_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "1233");
        Ok(())
    }

    #[test]
    fn aoc2021_01_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "1275");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2021_01> {
        AoC2021_01::new()
    }
}
