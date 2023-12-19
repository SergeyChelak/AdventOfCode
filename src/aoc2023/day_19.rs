use crate::{solution::Solution, utils::remove_first_and_last};

use std::{collections::HashMap, io};

type Int = i64;

struct Workflow {
    id: String,
    rules: Vec<Rule>,
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let Some(index) = value.find('{') else {
            panic!("Bracket {{ is expected");
        };
        let id = value[..index].to_string();
        let rules = remove_first_and_last(&value[index..])
            .split(',')
            .map(Rule::from)
            .collect::<Vec<_>>();
        Self { id, rules }
    }
}

enum Rule {
    Compare(char, Condition, Int, String),
    Goto(String),
}

enum Condition {
    Greater,
    Less,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        if let Some(idx) = value.find(':') {
            let chars = value.chars().collect::<Vec<_>>();
            let condition = match chars[1] {
                '>' => Condition::Greater,
                '<' => Condition::Less,
                ch => panic!("Unexpected condition {ch}"),
            };
            let number = value[2..idx]
                .parse::<Int>()
                .expect("Failed to parse value in the condition");
            let jump = value[idx + 1..].to_string();
            Self::Compare(chars[0], condition, number, jump)
        } else {
            Self::Goto(value.to_string())
        }
    }
}

struct Part {
    x: Int,
    m: Int,
    a: Int,
    s: Int,
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let tokens = remove_first_and_last(value).split(',').collect::<Vec<_>>();
        let (mut x, mut m, mut a, mut s) = (0, 0, 0, 0);
        for token in tokens {
            let (tag_name, tag_str_value) = token.split_once('=').expect("Incorrect part format");
            let tag_value = tag_str_value
                .parse::<Int>()
                .expect("Part parameter value should be integer");
            match tag_name {
                "x" => x = tag_value,
                "m" => m = tag_value,
                "a" => a = tag_value,
                "s" => s = tag_value,
                _ => panic!("Unexpected tag name {tag_name}"),
            }
        }
        Part { x, m, a, s }
    }
}

pub struct AoC2023_19 {
    parts: Vec<Part>,
    workflows: HashMap<String, Workflow>,
}

impl AoC2023_19 {
    pub fn new() -> io::Result<Self> {
        let input = std::fs::read_to_string("input/aoc2023_19")?;
        Ok(Self::with_str(&input))
    }

    fn with_str(s: &str) -> Self {
        let (workflows, parts) = s
            .split_once("\n\n")
            .expect("Workflows should be separated with empty line");

        let parts = parts.split_whitespace().map(Part::from).collect::<Vec<_>>();
        let workflows = workflows
            .split_whitespace()
            .map(Workflow::from)
            .map(|elem| (elem.id.clone(), elem))
            .collect::<HashMap<_, _>>();
        Self { workflows, parts }
    }
}

impl Solution for AoC2023_19 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 19: Aplenty".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_19_input_load_test() -> io::Result<()> {
        let sol = AoC2023_19::new()?;
        assert!(!sol.parts.is_empty());
        assert!(!sol.workflows.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_19_correctness() -> io::Result<()> {
        let sol = AoC2023_19::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
