use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = i64;

pub struct AoC2019_01 {
    input: Vec<Int>,
}

impl AoC2019_01 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2019_01")?;
        Ok(Self::from_lines(&lines))
    }

    fn from_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|s| s.as_ref().parse::<Int>().expect("Failed to parse input"))
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2019_01 {
    fn part_one(&self) -> String {
        self.input
            .iter()
            .map(|x| fuel_amount(*x))
            .sum::<Int>()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.input
            .iter()
            .map(|x| extra_fuel_amount(*x))
            .sum::<Int>()
            .to_string()
    }

    fn description(&self) -> String {
        "Day 1: The Tyranny of the Rocket Equation".to_string()
    }
}

fn fuel_amount(mass: Int) -> Int {
    // Fuel required to launch a given module is based on its mass.
    // Specifically, to find the fuel required for a module,
    // take its mass, divide by three, round down, and subtract 2.
    mass / 3 - 2
}

fn extra_fuel_amount(mut mass: Int) -> Int {
    let mut amount = 0;
    loop {
        mass = fuel_amount(mass);
        if mass <= 0 {
            break;
        }
        amount += mass;
    }
    amount
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_01_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_01_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "");
        Ok(())
    }

    #[test]
    fn aoc2019_01_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_01> {
        AoC2019_01::new()
    }
}
