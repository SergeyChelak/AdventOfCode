use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet, VecDeque};
use std::io;

type Color = String;

#[derive(Debug, PartialEq, Eq)]
struct Content {
    color: Color,
    count: usize,
}

impl From<&str> for Content {
    fn from(value: &str) -> Self {
        let tokens = value.split(" ").collect::<Vec<_>>();

        let count = tokens[0]
            .parse::<usize>()
            .expect("Expected integer number of bags");

        let color = [tokens[1], tokens[2]].join(" ");

        Self { color, count }
    }
}

struct Entry {
    color: Color,
    contains: Vec<Content>,
}

impl From<&str> for Entry {
    fn from(value: &str) -> Self {
        let (color, contain) = value
            .split_once(" bags contain ")
            .expect("Invalid input format {value}");

        let color = color.to_string();

        if contain.starts_with("no other") {
            return Self {
                color,
                contains: Vec::new(),
            };
        }
        let contains = contain.split(", ").map(Content::from).collect();
        Self { color, contains }
    }
}

const SHINY_GOLD: &str = "shiny gold";

pub struct AoC2020_07 {
    input: HashMap<Color, Vec<Content>>,
}

impl AoC2020_07 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2020_07")?;
        Ok(Self::parse(&lines))
    }

    fn parse<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(Entry::from)
            .map(|e| (e.color, e.contains))
            .collect::<HashMap<_, _>>();
        Self { input }
    }
}

impl Solution for AoC2020_07 {
    fn part_one(&self) -> String {
        let mut colors = VecDeque::new();
        colors.push_back(SHINY_GOLD);
        let mut seen = HashSet::<&str>::new();
        let mut count = 0;
        while let Some(color) = colors.pop_front() {
            for (k, v) in self.input.iter() {
                if seen.contains(k.as_str()) {
                    continue;
                }
                let hit = v.iter().map(|x| x.color.as_str()).any(|x| x == color);
                if hit {
                    seen.insert(k);
                    count += 1;
                    colors.push_back(k);
                }
            }
        }
        count.to_string()
    }

    fn part_two(&self) -> String {
        // shiny gold also included, that's why subtract 1
        (nested_count(&self.input, SHINY_GOLD) - 1).to_string()
    }

    fn description(&self) -> String {
        "Day 7: Handy Haversacks".to_string()
    }
}

fn nested_count(bags: &HashMap<Color, Vec<Content>>, current: &str) -> usize {
    let Some(content) = bags.get(current) else {
        // unreachable
        return 1;
    };

    let mut count = 1;
    for elem in content {
        let k = elem.count * nested_count(bags, elem.color.as_str());
        count += k;
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_07_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2020_07_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "370");
        Ok(())
    }

    #[test]
    fn aoc2020_07_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "29547");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_07> {
        AoC2020_07::new()
    }

    #[test]
    fn aoc2020_07_entry_parse() {
        {
            let e = Entry::from("light red bags contain 1 bright white bag, 2 muted yellow bags.");
            assert_eq!(e.color, "light red");
            assert_eq!(
                e.contains,
                vec![
                    Content {
                        color: "bright white".to_string(),
                        count: 1
                    },
                    Content {
                        color: "muted yellow".to_string(),
                        count: 2
                    },
                ]
            )
        }

        {
            let e = Entry::from("faded blue bags contain no other bags.");
            assert_eq!(e.color, "faded blue");
            assert!(e.contains.is_empty())
        }

        {
            let e = Entry::from("bright white bags contain 1 shiny gold bag.");
            assert_eq!(e.color, "bright white");
            assert_eq!(
                e.contains,
                vec![Content {
                    color: "shiny gold".to_string(),
                    count: 1
                },]
            )
        }
    }

    #[test]
    fn aoc2020_07_case_1() {
        let sol = test_make_solution();
        assert_eq!(sol.part_one(), "4");
    }

    #[test]
    fn aoc2020_07_case_2() {
        let sol = test_make_solution();
        assert_eq!(sol.part_two(), "32");
    }

    fn test_make_solution() -> AoC2020_07 {
        AoC2020_07::parse(&[
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ])
    }

    #[test]
    fn aoc2020_07_case_2_1() {
        let sol = AoC2020_07::parse(&[
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ]);
        assert_eq!(sol.part_two(), "126");
    }
}
