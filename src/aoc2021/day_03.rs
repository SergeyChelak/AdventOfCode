use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2021_03 {
    input: Vec2<char>,
}

impl AoC2021_03 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2021_03")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2021_03 {
    fn part_one(&self) -> String {
        let mut gamma = 0usize;
        let mut epsilon = 0usize;
        let Some(width) = self.input.first().map(|x| x.len()) else {
            return not_found();
        };

        let height = self.input.len();
        for col in 0..width {
            let ones = ones_count(&self.input, col);
            let zeros = height - ones;
            let bit = ones > zeros;
            push_bit(&mut gamma, bit);
            push_bit(&mut epsilon, !bit);
        }
        (gamma * epsilon).to_string()
    }

    fn part_two(&self) -> String {
        let generator = calculate(
            &self.input,
            |ones, zeros| if ones >= zeros { '1' } else { '0' },
        );
        let scrubber = calculate(
            &self.input,
            |ones, zeros| if ones >= zeros { '0' } else { '1' },
        );
        (generator * scrubber).to_string()
    }

    fn description(&self) -> String {
        "Day 3: Binary Diagnostic".to_string()
    }
}

fn calculate(input: &Vec2<char>, crit: impl Fn(usize, usize) -> char) -> usize {
    let Some(width) = input.first().map(|x| x.len()) else {
        return 0;
    };
    let mut input = input.clone();
    for col in 0..width {
        let len = input.len();
        if len == 1 {
            break;
        }
        let ones = ones_count(&input, col);
        let zeros = len - ones;
        let val = crit(ones, zeros);
        input.retain(|x| x[col] == val);
    }
    chars_to_int(&input[0])
}

fn ones_count(input: &Vec2<char>, col: usize) -> usize {
    input.iter().filter(|x| x[col] == '1').count()
}

fn chars_to_int(input: &[char]) -> usize {
    input.iter().fold(0usize, |mut acc, val| {
        push_bit(&mut acc, *val == '1');
        acc
    })
}

fn push_bit(input: &mut usize, bit: bool) {
    let bit = if bit { 1 } else { 0 };
    *input = (*input << 1) | bit;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_03_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2021_03_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "3148794");
        Ok(())
    }

    #[test]
    fn aoc2021_03_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "2795310");
        Ok(())
    }

    #[test]
    fn aoc2021_03_case2() {
        let sol = make_test_solution();
        assert_eq!(sol.part_two(), "230")
    }

    fn make_solution() -> io::Result<AoC2021_03> {
        AoC2021_03::new()
    }

    fn make_test_solution() -> AoC2021_03 {
        #[rustfmt::skip]
        let lines = [
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ];
        AoC2021_03::parse_lines(&lines)
    }
}
