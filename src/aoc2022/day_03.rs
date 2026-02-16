use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

pub struct AoC2022_03 {
    input: Vec<String>,
}

impl AoC2022_03 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2022_03")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref().to_string())
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2022_03 {
    fn part_one(&self) -> String {
        self.input
            .iter()
            .filter_map(|x| wrong_char(x))
            .map(priority)
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        let mut seen = HashSet::<&String>::new();
        let mut sum = 0usize;
        'outer: for (i, a) in self.input.iter().enumerate() {
            if seen.contains(&a) {
                continue;
            }
            let set_a = a.chars().collect::<HashSet<_>>();
            for (j, b) in self.input.iter().enumerate().skip(i + 1) {
                if seen.contains(&b) {
                    continue;
                }
                let set_b = b.chars().collect::<HashSet<_>>();
                for c in self.input.iter().skip(j + 1) {
                    if seen.contains(&c) {
                        continue;
                    }
                    let set_c = c.chars().collect::<HashSet<_>>();

                    let result = set_a
                        .iter()
                        .filter(|c| set_b.contains(c) && set_c.contains(c))
                        .cloned()
                        .collect::<Vec<_>>();
                    if result.len() == 1 {
                        sum += priority(result[0]);
                        seen.insert(a);
                        seen.insert(b);
                        seen.insert(c);
                        continue 'outer;
                    }
                }
            }
        }
        sum.to_string()
    }

    fn description(&self) -> String {
        "Day 3: Rucksack Reorganization".to_string()
    }
}

fn wrong_char(inp: &str) -> Option<char> {
    let med = inp.len() / 2;

    let left = inp[..med].chars().collect::<HashSet<_>>();
    let right = inp[med..].chars().collect::<HashSet<_>>();

    left.intersection(&right).next().copied()
}

fn priority(ch: char) -> usize {
    let offset = if ch.is_uppercase() { 26 } else { 0 };
    let ch = ch.to_ascii_lowercase();
    (ch as u8 - b'a') as usize + offset + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_03_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2022_03_priority() {
        let data = [
            (16, 'p'),
            (38, 'L'),
            (42, 'P'),
            (22, 'v'),
            (20, 't'),
            (19, 's'),
        ];
        for (exp, ch) in data {
            assert_eq!(priority(ch), exp);
        }
    }

    #[test]
    fn aoc2022_03_case_1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "157")
    }

    #[test]
    fn aoc2022_03_case_2() {
        let sol = make_test_solution();
        assert_eq!(sol.part_two(), "70")
    }

    #[test]
    fn aoc2022_03_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "7763");
        Ok(())
    }

    #[test]
    fn aoc2022_03_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "2569");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2022_03> {
        AoC2022_03::new()
    }

    fn make_test_solution() -> AoC2022_03 {
        let input = [
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];
        AoC2022_03::parse_lines(&input)
    }
}
