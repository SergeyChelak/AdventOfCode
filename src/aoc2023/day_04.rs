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

    fn part_two(&self) -> String {
        let mut instances = vec![1usize; self.input.len()];
        for (i, (win, cur)) in self.input.iter().enumerate() {
            let matches = cur.iter().filter(|x| win.contains(x)).count();
            for j in 1..=matches {
                instances[j + i] += instances[i];
            }
        }
        instances.iter().sum::<usize>().to_string()
    }

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
    fn aoc2023_04_ex2() {
        let lines = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let input = AoC2023_04::parse(&lines);
        let sol = AoC2023_04 { input };
        assert_eq!("30", sol.part_two());
    }

    #[test]
    fn aoc2023_04_correctness() -> io::Result<()> {
        let sol = AoC2023_04::new()?;
        assert_eq!(sol.part_one(), "20667");
        assert_eq!(sol.part_two(), "5833065");
        Ok(())
    }
}
