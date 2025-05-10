use crate::solution::Solution;
use crate::utils::*;
use mod_exp::mod_exp;

use std::io;

type Int = i128;

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
            .next_back()
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
        position(&self.input, 2019, 10007).to_string()
    }

    fn part_two(&self) -> String {
        let total: Int = 119315717514047;
        let steps: Int = 101741582076661;
        card(&self.input, 2020, total, steps).to_string()
    }

    fn description(&self) -> String {
        "Day 22: Slam Shuffle".to_string()
    }
}

fn position(rules: &[Shuffle], card: Int, total: Int) -> Int {
    rules.iter().fold(card, |acc, val| match *val {
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

fn card(rules: &[Shuffle], position: Int, total: Int, steps: Int) -> Int {
    // Convert the whole process to a linear equation: ax + b
    let (a, b) = rules.iter().rev().fold((1, 0), |(a, b), shuffle| {
        let (a_new, b_new) = match *shuffle {
            Shuffle::NewStack => (-a, -b - 1),
            Shuffle::Cut(n) => (a, b + n),
            Shuffle::Increment(n) => {
                let n = mod_exp(n, total - 2, total);
                (a * n, b * n)
            }
        };
        (a_new % total, b_new % total)
    });

    // Applying the function n times simplifies to:
    // x * a^n + b * (a^n - 1) / (a-1)
    let term1 = position * mod_exp(a, steps, total) % total;
    let tmp = (mod_exp(a, steps, total) - 1) * mod_exp(a - 1, total - 2, total) % total;
    let term2 = b * tmp % total;
    (term1 + term2) % total
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
        assert_eq!(sol.part_two(), "68849657493596");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_22> {
        AoC2019_22::new()
    }
}
