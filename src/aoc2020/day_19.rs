use crate::{
    solution::Solution,
    utils::{remove_first_and_last, Vec2},
};

use std::{collections::HashSet, io};

type Memo = Vec<Vec<String>>;

struct RawRule {
    index: usize,
    content: Rule,
}

#[derive(Clone)]
enum Rule {
    Value(char),
    Refer(Vec2<usize>),
}

fn calculate_options<'l>(rules: &[Rule], index: usize, store: &'l mut Memo) -> &'l [String] {
    if !store[index].is_empty() {
        return &store[index];
    }

    match &rules[index] {
        Rule::Value(ch) => {
            store[index] = vec![ch.to_string()];
        }
        Rule::Refer(arr) => {
            for group in arr {
                let mut acc = Vec::new();
                for idx in group {
                    let nested = calculate_options(rules, *idx, store);
                    acc = merged_acc(&acc, nested);
                }
                store[index].append(&mut acc);
            }
        }
    }

    &store[index]
}

fn merged_acc(current: &[String], input: &[String]) -> Vec<String> {
    if current.is_empty() {
        return input.to_vec();
    }
    let mut output = Vec::new();
    for cur in current {
        for inp in input {
            let mut s = String::new();
            s.push_str(cur);
            s.push_str(inp);
            output.push(s);
        }
    }
    output
}

fn calculate_valid_messages(rules: &[Rule], messages: &[String]) -> String {
    let mut store = vec![Vec::new(); rules.len()];
    let options = calculate_options(rules, 0, &mut store)
        .iter()
        .collect::<HashSet<_>>();

    messages
        .iter()
        .filter(|m| options.contains(*m))
        .count()
        .to_string()
}

impl From<&str> for RawRule {
    fn from(value: &str) -> Self {
        let (index, data) = value.split_once(": ").expect("Invalid input format");
        let index = index.parse::<usize>().expect("Invalid index value");

        let content = if data.starts_with('"') {
            let val = remove_first_and_last(data);
            assert_eq!(val.len(), 1);
            let ch = val.chars().next().expect("Invalid rule value");
            Rule::Value(ch)
        } else {
            let conditions = data
                .split(" | ")
                .map(|s| {
                    s.split(' ')
                        .map(|x| x.parse::<usize>().expect("Invalid rule ref index"))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            Rule::Refer(conditions)
        };

        Self { index, content }
    }
}

pub struct AoC2020_19 {
    rules: Vec<Rule>,
    messages: Vec<String>,
}

impl AoC2020_19 {
    pub fn new() -> io::Result<Self> {
        let input = std::fs::read_to_string("input/aoc2020_19")?;
        Ok(Self::parse(&input))
    }

    fn parse(input: &str) -> Self {
        let (rules, messages) = input.split_once("\n\n").expect("Invalid file format");
        let mut rules = rules
            .split('\n')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(RawRule::from)
            .collect::<Vec<_>>();
        rules.sort_by_key(|elem| elem.index);
        assert!(rules.iter().enumerate().all(|(i, elem)| i == elem.index));
        let rules = rules
            .into_iter()
            .map(|elem| elem.content)
            .collect::<Vec<_>>();

        let messages = messages
            .split('\n')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        Self { rules, messages }
    }
}

impl Solution for AoC2020_19 {
    fn part_one(&self) -> String {
        calculate_valid_messages(&self.rules, &self.messages)
    }

    fn part_two(&self) -> String {
        let mut rules = self.rules.clone();
        rules[8] = {
            let arr = vec![vec![42], vec![42, 8]];
            Rule::Refer(arr)
        };
        rules[11] = {
            let arr = vec![vec![42, 31], vec![42, 11, 31]];
            Rule::Refer(arr)
        };
        calculate_valid_messages(&rules, &self.messages)
    }

    fn description(&self) -> String {
        "Day 19: Monster Messages".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_19_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.messages.is_empty());
        // assert!(!sol.rules.is_empty());
        assert_eq!(134, sol.rules.len());
        Ok(())
    }

    #[test]
    fn aoc2020_19_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "210");
        Ok(())
    }

    #[test]
    fn aoc2020_19_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_19> {
        AoC2020_19::new()
    }

    #[test]
    fn aoc2020_19_parse_val() {
        let rule = RawRule::from(r#"2: "a""#);
        assert_eq!(rule.index, 2);
        assert!(matches!(rule.content, Rule::Value('a')));
    }

    #[test]
    fn aoc2020_19_parse_cond_1() {
        let rule = RawRule::from("61: 19 2");
        assert_eq!(rule.index, 61);
        let arr = vec![vec![19, 2]];
        match rule.content {
            Rule::Refer(r) => assert_eq!(r, arr),
            _ => panic!("Parse error"),
        }
    }

    #[test]
    fn aoc2020_19_parse_cond_2() {
        let rule = RawRule::from("18: 38 2 | 108 118");
        assert_eq!(rule.index, 18);
        let arr = vec![vec![38, 2], vec![108, 118]];
        match rule.content {
            Rule::Refer(r) => assert_eq!(r, arr),
            _ => panic!("Parse error"),
        }
    }
}
