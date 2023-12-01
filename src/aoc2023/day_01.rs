use crate::solution::Solution;
use crate::utils::*;

use std::char::from_digit;
use std::io;

pub struct AoC2023_01 {
    input: Vec<String>,
}

impl AoC2023_01 {
    pub fn new() -> io::Result<Self> {
        let input = read_file_as_lines("input/aoc2023_01")?;
        Ok(Self { input })
    }
}

impl Solution for AoC2023_01 {
    fn part_one(&self) -> String {
        self.input
            .iter()
            .map(|x| get_digit(x))
            .sum::<u32>()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.input
            .iter()
            .map(|x| get_spelled_digit(x))
            .sum::<u32>()
            .to_string()
    }

    fn description(&self) -> String {
        "AoC 2023/Day 1: Trebuchet?!".to_string()
    }
}

fn get_digit(s: &str) -> u32 {
    let digits = s
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>();
    digits.first().expect("digit should be present (1)") * 10
        + digits.last().expect("digit should be present (2)")
}

fn get_spelled_digit(s: &str) -> u32 {
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut output = s.chars().collect::<Vec<char>>();
    for (digit, w) in words.iter().enumerate() {
        s.match_indices(w).map(|(idx, _)| idx).for_each(|pos| {
            let ch = from_digit((digit + 1) as u32, 10)
                .expect("Impossible value during converting to char");
            output[pos] = ch;
        });
    }
    let out = String::from_iter(output);
    get_digit(&out)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_01_input_load_test() -> io::Result<()> {
        let sol = AoC2023_01::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_01_correctness() -> io::Result<()> {
        let sol = AoC2023_01::new()?;
        assert_eq!(sol.part_one(), "54644");
        assert_eq!(sol.part_two(), "53348");
        Ok(())
    }

    #[test]
    fn aoc2023_01_ex_2() {
        let arr = [
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
        ];
        for (inp, out) in arr {
            assert_eq!(get_spelled_digit(inp), out)
        }
    }
}
