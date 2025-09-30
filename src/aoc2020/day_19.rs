use crate::{
    solution::Solution,
    utils::{remove_first_and_last, Vec2},
};

use std::{collections::VecDeque, io};

struct RawRule {
    index: usize,
    content: Rule,
}

#[derive(Clone)]
enum Rule {
    Value(char),
    Refer(Vec2<usize>),
}

fn calculate_valid_messages(rules: &[Rule], messages: &[String]) -> String {
    messages
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .filter(|m| is_match(m, rules))
        .count()
        .to_string()
}

fn is_match(message: &[char], rules: &[Rule]) -> bool {
    let mut queue = VecDeque::new();

    queue.push_back((message, vec![0usize]));

    while let Some((msg, indices)) = queue.pop_front() {
        if msg.is_empty() && indices.is_empty() {
            return true;
        }

        if msg.is_empty() || indices.is_empty() || indices.len() > msg.len() {
            continue;
        }

        let indices_rest = &indices[1..];
        match &rules[indices[0]] {
            Rule::Value(ch) if *ch == msg[0] => {
                queue.push_back((&msg[1..], indices_rest.to_vec()));
            }
            Rule::Refer(subrules) => {
                for subrule in subrules {
                    let mut arr = subrule.clone();
                    arr.append(&mut indices_rest.to_vec());
                    queue.push_back((msg, arr));
                }
            }
            _ => continue,
        }
    }

    false
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
        rules[8] = RawRule::from("8: 42 | 42 8").content;
        rules[11] = RawRule::from("11: 42 31 | 42 11 31").content;
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
        assert_eq!(sol.part_two(), "422");
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

    #[test]
    fn aoc2020_19_case_1() {
        let input = r#"0: 4 1 5
        1: 2 3 | 3 2
        2: 4 4 | 5 5
        3: 4 5 | 5 4
        4: "a"
        5: "b"

        ababbb
        bababa
        abbbab
        aaabbb
        aaaabbb"#;
        let sol = AoC2020_19::parse(input);
        assert_eq!(sol.part_one(), "2");
    }
}
