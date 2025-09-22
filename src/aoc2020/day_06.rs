use crate::solution::Solution;

use std::io;

pub struct AoC2020_06 {
    input: String,
}

impl AoC2020_06 {
    pub fn new() -> io::Result<Self> {
        let input = std::fs::read_to_string("input/aoc2020_06")?;
        Ok(Self { input })
    }

    fn question_count(&self, criteria: impl Fn(&str) -> usize) -> String {
        self.input
            .split("\n\n")
            .map(criteria)
            .sum::<usize>()
            .to_string()
    }
}

impl Solution for AoC2020_06 {
    fn part_one(&self) -> String {
        self.question_count(anyone_question_count)
    }

    fn part_two(&self) -> String {
        self.question_count(everyone_question_count)
    }

    fn description(&self) -> String {
        "Day 6: Custom Customs".to_string()
    }
}

fn anyone_question_count(s: &str) -> usize {
    let mut in_use = [false; 26];
    for ch in s.chars() {
        match ch {
            'a'..='z' => {
                let idx = ch as u8 - b'a';
                in_use[idx as usize] = true;
            }
            _ => continue,
        }
    }
    in_use.iter().filter(|x| **x).count()
}

fn everyone_question_count(s: &str) -> usize {
    let mut in_use = [0usize; 26];
    let mut target = 1;
    for ch in s.trim().chars() {
        match ch {
            'a'..='z' => {
                let idx = ch as u8 - b'a';
                in_use[idx as usize] += 1;
            }
            '\n' => target += 1,
            _ => continue,
        }
    }
    in_use.into_iter().filter(|x| *x == target).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_06_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2020_06_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "6259");
        Ok(())
    }

    #[test]
    fn aoc2020_06_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "3178");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_06> {
        AoC2020_06::new()
    }

    #[test]
    fn aoc2020_06_case_2() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b"
        .to_string();
        let sol = AoC2020_06 { input };
        assert_eq!(sol.part_two(), "6")
    }
}
