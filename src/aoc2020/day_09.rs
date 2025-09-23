use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = usize;

pub struct AoC2020_09 {
    input: Vec<Int>,
}

impl AoC2020_09 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2020_09")?;
        Ok(Self::parse(&lines))
    }

    fn parse<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = parse(lines).expect("Invalid input value");
        Self { input }
    }
}

impl Solution for AoC2020_09 {
    fn part_one(&self) -> String {
        first_invalid_number(&self.input, 25)
            .map(|x| x.to_string())
            .unwrap_or(not_found())
    }

    fn part_two(&self) -> String {
        first_invalid_number(&self.input, 25)
            .and_then(|target| encryption_weakness(&self.input, target))
            .map(|x| x.to_string())
            .unwrap_or(not_found())
    }

    fn description(&self) -> String {
        "Day 9: Encoding Error".to_string()
    }
}

fn first_invalid_number(numbers: &[Int], preamble: usize) -> Option<Int> {
    if numbers.len() <= preamble {
        return None;
    }
    for window in numbers.windows(preamble + 1) {
        let target = window[preamble];
        let mut is_valid = false;
        'c: for (i, a) in window.iter().enumerate() {
            for b in window.iter().skip(i + 1) {
                is_valid = *a + *b == target;
                if is_valid {
                    break 'c;
                }
            }
        }
        if !is_valid {
            return Some(target);
        }
    }
    None
}

fn encryption_weakness(numbers: &[Int], target: usize) -> Option<Int> {
    let len = numbers.len();
    if len < 2 {
        return None;
    }

    let mut sum = numbers[0] + numbers[1];
    let mut l = 0;
    let mut r = 1;

    while r < len {
        if sum == target {
            let slice = &numbers[l..=r];
            let (Some(smallest), Some(largest)) = (slice.iter().min(), slice.iter().max()) else {
                return None;
            };
            return Some(smallest + largest);
        }
        if sum > target {
            sum -= numbers[l];
            l += 1;
        } else {
            r += 1;
            sum += numbers[r];
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_09_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2020_09_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "373803594");
        Ok(())
    }

    #[test]
    fn aoc2020_09_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "51152360");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_09> {
        AoC2020_09::new()
    }

    #[test]
    fn aoc2020_09_case_1() {
        let numbers = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(first_invalid_number(&numbers, 5), Some(127));
    }

    #[test]
    fn aoc2020_09_case_2() {
        let numbers = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(encryption_weakness(&numbers, 127), Some(62));
    }
}
