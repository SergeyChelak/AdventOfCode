use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = isize;
type Interval = PlainInterval<Int>;
type Pair = (Interval, Interval);

pub struct AoC2022_04 {
    input: Vec<Pair>,
}

impl AoC2022_04 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2022_04")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(parse_pair)
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2022_04 {
    fn part_one(&self) -> String {
        self.input
            .iter()
            .filter(|x| is_full_overlap(x))
            .count()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.input
            .iter()
            .filter(|x| x.0.has_intersection(&x.1))
            .count()
            .to_string()
    }

    fn description(&self) -> String {
        "Day 4: Camp Cleanup".to_string()
    }
}

fn parse_pair(value: &str) -> Pair {
    let (first, second) = value.split_once(',').expect("Invalid interval pair format");
    (
        Interval::parse(first, "-").expect("Invalid 1st interval format"),
        Interval::parse(second, "-").expect("Invalid 1st interval format"),
    )
}

fn is_full_overlap(pair: &Pair) -> bool {
    let len = |interval: &Interval| -> Int { interval.end - interval.begin };

    let len_f = len(&pair.0);
    let len_s = len(&pair.1);

    let Some(intersection) = pair.0.intersection(&pair.1) else {
        return false;
    };

    let len_i = len(&intersection);

    len_i == len_f || len_i == len_s
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_04_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2022_04_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "305");
        Ok(())
    }

    #[test]
    fn aoc2022_04_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "811");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2022_04> {
        AoC2022_04::new()
    }
}
