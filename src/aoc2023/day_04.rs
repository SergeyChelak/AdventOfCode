use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2023_04 {
    input: Vec<(Vec<u32>, Vec<u32>)>,
}

impl AoC2023_04 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_04")?;
        let input = Self::parse(&lines);
        Ok(Self { input })
    }

    fn parse(lines: &[String]) -> Vec<(Vec<u32>, Vec<u32>)> {
        lines
            .iter()
            .map(|s| {
                let (_, card_info) = s.split_once(": ").expect("Card number delimiter not found");
                let (win_numbers, card_numbers) = card_info
                    .split_once(" | ")
                    .expect("Win number delimiter not found");
                let win = Self::parse_numbers(win_numbers);
                let current = Self::parse_numbers(card_numbers);
                (win, current)
            })
            .collect::<Vec<(Vec<u32>, Vec<u32>)>>()
    }

    fn parse_numbers(s: &str) -> Vec<u32> {
        s.split(' ')
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u32>().expect("Number is expected"))
            .collect::<Vec<_>>()
    }
}

impl Solution for AoC2023_04 {
    fn part_one(&self) -> String {
        let mut total = 0;
        for (win, cur) in &self.input {
            let matches = cur.iter().filter(|x| win.contains(x)).count();
            if matches > 0 {
                total += 1 << (matches - 1);
            }
        }
        total.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 4: Scratchcards".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_04_input_load_test() -> io::Result<()> {
        let sol = AoC2023_04::new()?;
        assert_eq!(211, sol.input.len());
        Ok(())
    }

    #[test]
    fn aoc2023_04_correctness() -> io::Result<()> {
        let sol = AoC2023_04::new()?;
        assert_eq!(sol.part_one(), "20667");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
