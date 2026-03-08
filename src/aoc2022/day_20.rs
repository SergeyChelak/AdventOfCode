use crate::{solution::Solution, utils::not_found};

use std::io;

type Int = isize;

pub struct AoC2022_20 {
    input: Vec<Int>,
}

impl AoC2022_20 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2022_20")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        Self::parse_lines(&data.lines().collect::<Vec<_>>())
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|l| l.as_ref().parse::<Int>().unwrap())
            .collect();
        Self { input }
    }
}

impl Solution for AoC2022_20 {
    fn part_one(&self) -> String {
        let mixed = mixing(&self.input, 1);

        sum_coordinates(&mixed)
            .map(|x| x.to_string())
            .unwrap_or(not_found())
    }

    fn part_two(&self) -> String {
        let decryption_key: Int = 811_589_153;
        let keyed_input: Vec<Int> = self.input.iter().map(|x| x * decryption_key).collect();
        let mixed = mixing(&keyed_input, 10);
        sum_coordinates(&mixed)
            .map(|x| x.to_string())
            .unwrap_or(not_found())
    }

    fn description(&self) -> String {
        "Day 20: Grove Positioning System".to_string()
    }
}

fn sum_coordinates(input: &[Int]) -> Option<Int> {
    let (zero_idx, _) = input.iter().enumerate().find(|(_, x)| **x == 0)?;
    let len = input.len();
    let value = [1000, 2000, 3000]
        .iter()
        .map(|val| (zero_idx + *val) % len)
        .map(|idx| input[idx])
        .sum::<Int>();
    Some(value)
}

fn mixing(input: &[Int], rounds: usize) -> Vec<Int> {
    let len = input.len();
    let mut order: Vec<usize> = (0..len).collect();

    for _ in 0..rounds {
        for (orig_idx, &value) in input.iter().enumerate() {
            if value == 0 {
                continue;
            }

            let current_pos = order.iter().position(|&p| p == orig_idx).unwrap();
            order.remove(current_pos);

            // Using i128 to safely handle the large decryption key values
            let mut new_pos = (current_pos as i128 + value as i128) % (len as i128 - 1);

            if new_pos < 0 {
                new_pos += len as i128 - 1;
            }

            order.insert(new_pos as usize, orig_idx);
        }
    }

    order.iter().map(|&orig_idx| input[orig_idx]).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_20_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2022_20_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "8028");
        Ok(())
    }

    #[test]
    fn aoc2022_20_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "8798438007673");
        Ok(())
    }

    #[test]
    fn aoc2022_20_case_1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "3")
    }

    fn example_input() -> Vec<Int> {
        vec![1, 2, -3, 3, -2, 0, 4]
    }

    fn make_test_solution() -> AoC2022_20 {
        AoC2022_20 {
            input: example_input(),
        }
    }

    fn make_solution() -> io::Result<AoC2022_20> {
        AoC2022_20::new()
    }
}
