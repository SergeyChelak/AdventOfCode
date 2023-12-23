use crate::{
    solution::Solution,
    utils::{remove_first_and_last, PlainInterval},
};

use std::{collections::HashMap, io};

type Int = u64;
type Part = [Int; 4];
type Interval = PlainInterval<Int>;
type Intervals = [Interval; 4];

const RATING_MIN: Int = 1;
const RATING_MAX: Int = 4000;
const WORKFLOW_START: &str = "in";
const WORKFLOW_ACCEPT: &str = "A";
const WORKFLOW_REJECT: &str = "R";

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

impl Default for Interval {
    fn default() -> Self {
        Self::new(RATING_MIN, RATING_MAX)
    }
}

impl Interval {
    fn len(&self) -> Int {
        self.end - self.begin + 1
    }
}

enum Criteria {
    Greater,
    Less,
}

enum Rule {
    Operation(Criteria, usize, Int, String),
    Jump(String),
}

impl Rule {
    fn exec(&self, part: &Part) -> Option<String> {
        match self {
            Rule::Jump(next) => Some(next.clone()),
            Rule::Operation(criteria, idx, value, next) => {
                let res = match criteria {
                    Criteria::Greater => part[*idx] > *value,
                    Criteria::Less => part[*idx] < *value,
                };
                if res {
                    Some(next.clone())
                } else {
                    None
                }
            }
        }
    }

    fn interval(&self) -> Option<Interval> {
        match self {
            Rule::Jump(_) => None,
            Rule::Operation(Criteria::Greater, _, value, _) => {
                Some(Interval::new(value + 1, RATING_MAX))
            }
            Rule::Operation(Criteria::Less, _, value, _) => {
                Some(Interval::new(RATING_MIN, value - 1))
            }
        }
    }

    fn rev_interval(&self) -> Option<Interval> {
        match self {
            Rule::Jump(_) => None,
            Rule::Operation(Criteria::Greater, _, value, _) => {
                Some(Interval::new(RATING_MIN, *value))
            }
            Rule::Operation(Criteria::Less, _, value, _) => Some(Interval::new(*value, RATING_MAX)),
        }
    }
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        if let Some(idx) = value.find(':') {
            let chars = value.chars().collect::<Vec<_>>();
            let operand = ch_to_idx(chars[0]);
            let number = value[2..idx]
                .parse::<Int>()
                .expect("Failed to parse value in the condition");
            let jump = value[idx + 1..].to_string();
            let criteria = match chars[1] {
                '>' => Criteria::Greater,
                '<' => Criteria::Less,
                ch => panic!("Unexpected condition {ch}"),
            };
            Rule::Operation(criteria, operand, number, jump)
        } else {
            Self::Jump(value.to_string())
        }
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

        let parts = parts
            .split_whitespace()
            .map(Self::parse_ratings)
            .collect::<Vec<_>>();
        let workflows = workflows
            .split_whitespace()
            .map(Workflow::from)
            .map(|elem| (elem.id.clone(), elem))
            .collect::<HashMap<_, _>>();
        Self { workflows, parts }
    }

    fn parse_ratings(value: &str) -> Part {
        let tokens = remove_first_and_last(value).split(',').collect::<Vec<_>>();
        let mut result = [0; 4];
        for token in tokens {
            let (tag_name, tag_str_value) = token.split_once('=').expect("Incorrect part format");
            let tag_value = tag_str_value
                .parse::<Int>()
                .expect("Part parameter value should be integer");
            let ch = tag_name.parse::<char>().expect("Incorrect tag name");
            result[ch_to_idx(ch)] = tag_value;
        }
        result
    }
}

impl Solution for AoC2023_19 {
    fn part_one(&self) -> String {
        let mut accepted: Vec<&Part> = Vec::new();
        for part in &self.parts {
            let mut id = WORKFLOW_START.to_string();
            loop {
                let workflow = self.workflows.get(&id).expect("workflow should be present");
                for rule in &workflow.rules {
                    if let Some(next) = rule.exec(part) {
                        id = next;
                        break;
                    }
                }
                if id == WORKFLOW_ACCEPT {
                    accepted.push(part);
                    break;
                }
                if id == WORKFLOW_REJECT {
                    break;
                }
            }
        }
        accepted
            .iter()
            .map(|p| p.iter().sum::<Int>())
            .sum::<Int>()
            .to_string()
    }

    fn part_two(&self) -> String {
        let mut accepted: Vec<Intervals> = Vec::new();
        let mut stack = vec![(WORKFLOW_START.to_string(), Intervals::default())];
        while let Some(state) = stack.pop() {
            let id = state.0;
            let mut current = state.1;
            if id == WORKFLOW_ACCEPT {
                accepted.push(current);
                continue;
            }
            if id == WORKFLOW_REJECT {
                continue;
            }
            let Some(workflow) = self.workflows.get(&id) else {
                panic!("Workflow {id} should exist");
            };
            for rule in &workflow.rules {
                match rule {
                    Rule::Jump(next) => stack.push((next.clone(), current)),
                    Rule::Operation(_, idx, _, next) => {
                        let interval = rule.interval().unwrap();
                        if let Some(intersection) = interval.intersection(&current[*idx]) {
                            let mut tmp = current;
                            tmp[*idx] = intersection;
                            stack.push((next.clone(), tmp));
                        }
                        let interval = rule.rev_interval().unwrap();
                        if let Some(intersection) = interval.intersection(&current[*idx]) {
                            current[*idx] = intersection;
                        } else {
                            panic!("is it possible??");
                        }
                    }
                }
            }
        }
        accepted
            .iter()
            .map(|interval| interval.iter().map(|x| x.len()).product::<Int>())
            .sum::<Int>()
            .to_string()
    }

    fn description(&self) -> String {
        "AoC 2023/Day 19: Aplenty".to_string()
    }
}

fn ch_to_idx(ch: char) -> usize {
    match ch {
        'x' => 0,
        'm' => 1,
        'a' => 2,
        's' => 3,
        _ => panic!("Unexpected tag id {ch}"),
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
    fn aoc2023_19_ex1() {
        assert_eq!(puzzle().part_one(), "19114");
    }

    #[test]
    fn aoc2023_19_ex2() {
        assert_eq!(puzzle().part_two(), "167409079868000");
    }

    fn puzzle() -> AoC2023_19 {
        let input = r#"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
        "#;
        AoC2023_19::with_str(input)
    }

    #[test]
    fn aoc2023_19_correctness() -> io::Result<()> {
        let sol = AoC2023_19::new()?;
        assert_eq!(sol.part_one(), "446517");
        assert_eq!(sol.part_two(), "130090458884662");
        Ok(())
    }
}
