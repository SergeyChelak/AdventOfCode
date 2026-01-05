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
}

impl Solution for AoC2021_14 {
    fn part_one(&self) -> String {
        let mut template = self.template.clone();
        for _ in 0..10 {
            template = simulate(&template, &self.rules);
        }
        let freq = frequencies(&template);

        let max_fr = freq.iter().max().unwrap();
        let min_fr = freq.iter().filter(|val| **val > 0).min().unwrap();

        (max_fr - min_fr).to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 14: Extended Polymerization".to_string()
    }
}

fn simulate(template: &Chars, rules: &Rules) -> Chars {
    let mut buffer = template
        .iter()
        .flat_map(|x| [Some(*x), None])
        .collect::<Vec<_>>();

    for (idx, window) in template.windows(2).enumerate() {
        let Some(ch) = rules.get(window) else {
            continue;
        };
        buffer[2 * idx + 1] = Some(*ch);
    }

    buffer.iter().filter_map(|x| *x).collect::<Chars>()
}

fn frequencies(chars: &Chars) -> [usize; 256] {
    let mut frq = [0usize; 256];
    chars.iter().for_each(|ch| {
        let idx = *ch as u8 as usize;
        frq[idx] += 1;
    });
    frq
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
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2021_14_simulate() {
        let sol = make_test_solution();

        let result = simulate(&sol.template, &sol.rules)
            .iter()
            .collect::<String>();

        assert_eq!(result, "NCNBCHB")
    }

    fn make_solution() -> io::Result<AoC2021_14> {
        AoC2021_14::new()
    }

    fn make_test_solution() -> AoC2021_14 {
        let input = "NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
";
        AoC2021_14::parse_data(input)
    }
}
