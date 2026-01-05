use crate::solution::Solution;

use std::{collections::HashMap, io};

type Chars = Vec<char>;
type Rules = HashMap<[char; 2], char>;

pub struct AoC2021_14 {
    template: Chars,
    rules: Rules,
}

impl AoC2021_14 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2021_14")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        let (template, rules) = data.split_once("\n\n").expect("Invalid input format");
        let template = template.chars().collect::<Chars>();
        let rules = rules
            .split('\n')
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(Self::parse_rule)
            .collect::<Rules>();
        Self { template, rules }
    }

    fn parse_rule(data: &str) -> ([char; 2], char) {
        let (inp, out) = data
            .split_once(" -> ")
            .unwrap_or_else(|| panic!("Invalid rule format: '{data}'"));
        assert_eq!(inp.len(), 2);
        assert_eq!(out.len(), 1);

        let mut iter = inp.chars();
        let in_1 = iter.next().expect("1st char not present into input");
        let in_2 = iter.next().expect("2nd char not present into input");

        let out = out.chars().next().expect("Output char not present");
        ([in_1, in_2], out)
    }

    fn calculate(&self, times: usize) -> String {
        let mut state = State::with(&self.template);
        for _ in 0..times {
            state.next(&self.rules);
        }
        state.delta().to_string()
    }
}

impl Solution for AoC2021_14 {
    fn part_one(&self) -> String {
        self.calculate(10)
    }

    fn part_two(&self) -> String {
        self.calculate(40)
    }

    fn description(&self) -> String {
        "Day 14: Extended Polymerization".to_string()
    }
}

struct State {
    frequencies: [usize; 256],
    pairs: HashMap<[char; 2], usize>,
}

impl State {
    fn with(template: &Chars) -> Self {
        let mut frequencies = [0usize; 256];
        template.iter().for_each(|ch| frequencies[ch_idx(*ch)] += 1);
        let pairs = template.windows(2).map(|w| ([w[0], w[1]], 1)).collect();
        Self { pairs, frequencies }
    }

    fn delta(&self) -> usize {
        let max_fr = self.frequencies.iter().max().unwrap();
        let min_fr = self
            .frequencies
            .iter()
            .filter(|val| **val > 0)
            .min()
            .unwrap();
        max_fr - min_fr
    }

    fn next(&mut self, rules: &Rules) {
        let mut next_pairs = HashMap::new();

        for (pair, count) in self.pairs.iter() {
            assert!(self.frequencies[ch_idx(pair[0])] > 0);
            assert!(self.frequencies[ch_idx(pair[1])] > 0);

            let Some(ch) = rules.get(pair) else {
                *next_pairs.entry(*pair).or_default() += count;
                continue;
            };
            *next_pairs.entry([pair[0], *ch]).or_default() += count;
            *next_pairs.entry([*ch, pair[1]]).or_default() += count;
            self.frequencies[ch_idx(*ch)] += count;
        }
        self.pairs = next_pairs;
    }
}

fn ch_idx(ch: char) -> usize {
    ch as u8 as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_14_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.template.is_empty());
        assert!(!sol.rules.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2021_14_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "2549");
        Ok(())
    }

    #[test]
    fn aoc2021_14_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "2516901104210");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2021_14> {
        AoC2021_14::new()
    }
}
