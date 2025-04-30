use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = isize;

enum Shuffle {
    NewStack,
    Cut(Int),
    Increment(Int),
}

impl From<&str> for Shuffle {
    fn from(value: &str) -> Self {
        if value == "deal into new stack" {
            return Self::NewStack;
        }
        let number = value
            .split(' ')
            .last()
            .and_then(|x| x.parse::<Int>().ok())
            .expect("Failed to parse number");
        if value.starts_with("deal with increment") {
            return Shuffle::Increment(number);
        }
        if value.starts_with("cut") {
            return Shuffle::Cut(number);
        }
        panic!("unexpected input {}", value);
    }
}

pub struct AoC2019_22 {
    input: Vec<Shuffle>,
}

impl AoC2019_22 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2019_22")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|val| val.as_ref())
            .map(Shuffle::from)
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2019_22 {
    fn part_one(&self) -> String {
        shuffle(&self.input, 2019, 10007).to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 22: Slam Shuffle".to_string()
    }
}

fn shuffle(rules: &[Shuffle], input: Int, total: Int) -> Int {
    rules.iter().fold(input, |acc, val| match *val {
        Shuffle::NewStack => total - acc - 1,
        Shuffle::Cut(n) => {
            let tmp = acc - n;
            if tmp < 0 {
                tmp + total
            } else {
                tmp % total
            }
        }
        Shuffle::Increment(n) => (acc * n) % total,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_22_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_22_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "4649");
        Ok(())
    }

    #[test]
    fn aoc2019_22_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_22> {
        AoC2019_22::new()
    }
}
