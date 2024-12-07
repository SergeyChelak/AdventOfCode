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
        calibration_result(&self.equations, &part_one_operators).to_string()
    }

    fn part_two(&self) -> String {
        calibration_result(&self.equations, &part_two_operators).to_string()
    }

    fn description(&self) -> String {
        "2024/Day 7: Bridge Repair".to_string()
    }
}

type OpDataProvider = dyn Fn(usize, EqNumber, EqNumber) -> Vec<OpData>;

fn calibration_result(equations: &[Equation], ops: &OpDataProvider) -> EqNumber {
    equations
        .iter()
        .filter(|x| can_evaluate(x, ops))
        .map(|x| x.result)
        .sum::<EqNumber>()
}

fn part_one_operators(next_index: usize, acc: EqNumber, next_value: EqNumber) -> Vec<OpData> {
    vec![
        sum_operator_data(next_index, acc, next_value),
        mul_operator_data(next_index, acc, next_value),
    ]
}

fn part_two_operators(next_index: usize, acc: EqNumber, next_value: EqNumber) -> Vec<OpData> {
    vec![
        sum_operator_data(next_index, acc, next_value),
        mul_operator_data(next_index, acc, next_value),
        concat_operator_data(next_index, acc, next_value),
    ]
}

fn sum_operator_data(next_index: usize, acc: EqNumber, next_value: EqNumber) -> OpData {
    OpData {
        index: next_index,
        accumulator: acc + next_value,
    }
}

fn mul_operator_data(next_index: usize, acc: EqNumber, next_value: EqNumber) -> OpData {
    OpData {
        index: next_index,
        accumulator: acc * next_value,
    }
}

fn concat_operator_data(next_index: usize, acc: EqNumber, next_value: EqNumber) -> OpData {
    OpData {
        index: next_index,
        accumulator: concat(acc, next_value),
    }
}

fn concat(mut a: EqNumber, b: EqNumber) -> EqNumber {
    let mut tmp = b;
    while tmp > 0 {
        a *= 10;
        tmp /= 10;
    }
    a + b
}

struct OpData {
    index: usize,
    accumulator: EqNumber,
}

fn can_evaluate(equation: &Equation, op_provider: &OpDataProvider) -> bool {
    let mut stack = vec![OpData {
        index: 0,
        accumulator: equation.numbers[0],
    }];
    let last_index = equation.numbers.len() - 1;
    while let Some(data) = stack.pop() {
        let acc = data.accumulator;
        if acc == equation.result && data.index == last_index {
            return true;
        }
        let next = data.index + 1;
        if acc > equation.result || next > last_index {
            continue;
        }
        let mut ops = op_provider(next, acc, equation.numbers[next]);
        stack.append(&mut ops);
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

    #[test]
    fn aoc2024_07_case_2() {
        let puzzle = make_puzzle();
        assert_eq!(puzzle.part_two(), "11387")
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
        assert_eq!(sol.part_two(), "124060392153684");
        Ok(())
    }
}
