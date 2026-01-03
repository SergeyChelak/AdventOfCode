use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

type Chars = Vec<char>;

struct Record {
    pattern: Vec<Chars>,
    output: Vec<Chars>,
}

pub struct AoC2021_08 {
    input: Vec<Record>,
}

impl AoC2021_08 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2021_08")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(Record::from)
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2021_08 {
    fn part_one(&self) -> String {
        let mut count = 0;
        for output in self.input.iter().map(|rec| &rec.output) {
            for val in output {
                match val.len() {
                    2 | 3 | 4 | 7 => count += 1,
                    _ => {}
                }
            }
        }
        count.to_string()
    }

    fn part_two(&self) -> String {
        let display_map = display_mapping();
        self.input
            .iter()
            .fold(0, |acc, x| acc + x.deduce(&display_map).unwrap_or_default())
            .to_string()
    }

    fn description(&self) -> String {
        "Day 8: Seven Segment Search".to_string()
    }
}

impl From<&str> for Record {
    fn from(value: &str) -> Self {
        let (patterns, outputs) = value.split_once(" | ").expect("Invalid record format");
        let parse = |input: &str| -> Vec<Chars> {
            input
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.chars().collect::<Chars>())
                .collect::<Vec<_>>()
        };
        let pattern = parse(patterns);
        let output = parse(outputs);
        Self { pattern, output }
    }
}

impl Record {
    fn deduce(&self, display_map: &HashMap<usize, usize>) -> Option<usize> {
        let arr = "abcdefg".chars().collect::<Chars>();
        for mapping in PermutationIterator::from_array(&arr) {
            let is_valid = self
                .pattern
                .iter()
                .map(|x| transform(x, &mapping))
                .map(|x| bitmap(&x))
                .all(|x| display_map.contains_key(&x));
            if !is_valid {
                continue;
            }
            let result = self
                .output
                .iter()
                .map(|x| transform(x, &mapping))
                .map(|x| bitmap(&x))
                .filter_map(|x| display_map.get(&x))
                .fold(0usize, |acc, x| acc * 10 + *x);
            return Some(result);
        }
        None
    }
}

fn transform(input: &Chars, map: &[char]) -> Chars {
    input
        .iter()
        .map(|ch| {
            let idx = (*ch as u8 - b'a') as usize;
            map[idx]
        })
        .collect()
}

fn display_mapping() -> HashMap<usize, usize> {
    #[rustfmt::skip]
    let segments = [
    "abcefg",
    "cf",
    "acdeg",
    "acdfg",
    "bcdf",
    "abdfg",
    "abdefg",
    "acf",
    "abcdefg",
    "abcdfg",
    ];
    segments
        .iter()
        .map(|s| bitmap(&s.chars().collect::<Chars>()))
        .enumerate()
        .map(|(num, set)| (set, num))
        .collect::<HashMap<_, _>>()
}

fn bitmap(value: &[char]) -> usize {
    // abcdefg
    // 6543210
    value.iter().fold(0, |acc, ch| {
        let shift = match ch {
            'a' => 6,
            'b' => 5,
            'c' => 4,
            'd' => 3,
            'e' => 2,
            'f' => 1,
            'g' => 0,
            _ => panic!("unexpected value"),
        };
        acc | (1 << shift)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_08_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2021_08_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "392");
        Ok(())
    }

    #[test]
    fn aoc2021_08_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "1004688");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2021_08> {
        AoC2021_08::new()
    }
}
