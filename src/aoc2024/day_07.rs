use crate::solution::Solution;
use crate::utils::*;

use std::io;

type EqNumber = i64;

struct Equation {
    result: EqNumber,
    numbers: Vec<EqNumber>,
}

impl Equation {
    fn from(s: &str) -> Self {
        let (value, numbers) = s.split_once(":").expect("Invalid equation format");
        let result = value
            .parse::<EqNumber>()
            .expect("Equation result isn't int value");
        let numbers = numbers
            .trim()
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.parse::<EqNumber>()
                    .expect("One of equation numbers isn't int value")
            })
            .collect::<Vec<_>>();
        Self { result, numbers }
    }
}

pub struct AoC2024_07 {
    equations: Vec<Equation>,
}

impl AoC2024_07 {
    pub fn new() -> io::Result<Self> {
        let input = read_file_as_lines("input/aoc2024_07")?;
        Ok(Self::with_strings(&input))
    }

    fn with_strings(strings: &[String]) -> Self {
        let equations = strings
            .iter()
            .map(|s| Equation::from(s))
            .collect::<Vec<_>>();
        Self { equations }
    }
}

impl Solution for AoC2024_07 {
    fn part_one(&self) -> String {
        self.equations
            .iter()
            .filter(|x| can_evaluate(x))
            .map(|x| x.result)
            .sum::<EqNumber>()
            .to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "2024/Day 7: Bridge Repair".to_string()
    }
}

struct StackData {
    index: usize,
    accumulator: EqNumber,
}

fn can_evaluate(equation: &Equation) -> bool {
    let mut stack = vec![StackData {
        index: 0,
        accumulator: equation.numbers[0],
    }];
    let last_index = equation.numbers.len() - 1;
    while let Some(data) = stack.pop() {
        let acc = data.accumulator;
        if acc > equation.result {
            continue;
        }
        if acc == equation.result && data.index == last_index {
            return true;
        }
        let next = data.index + 1;
        if next > last_index {
            continue;
        }
        stack.push(StackData {
            index: next,
            accumulator: acc + equation.numbers[next],
        });
        stack.push(StackData {
            index: next,
            accumulator: acc * equation.numbers[next],
        });
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_07_input_load_test() -> io::Result<()> {
        let sol = AoC2024_07::new()?;
        assert!(!sol.equations.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2024_07_case_1() {
        let puzzle = make_puzzle();
        assert_eq!(puzzle.part_one(), "3749")
    }

    fn make_puzzle() -> AoC2024_07 {
        let input = [
            "190: 10 19",
            "3267: 81 40 27",
            "83: 17 5",
            "156: 15 6",
            "7290: 6 8 6 15",
            "161011: 16 10 13",
            "192: 17 8 14",
            "21037: 9 7 18 13",
            "292: 11 6 16 20",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        AoC2024_07::with_strings(&input)
    }

    #[test]
    fn aoc2024_07_correctness() -> io::Result<()> {
        let sol = AoC2024_07::new()?;
        assert_eq!(sol.part_one(), "2654749936343");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
