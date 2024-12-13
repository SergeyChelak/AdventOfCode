use crate::solution::Solution;
use crate::utils::Vec2;

use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io;

pub struct AoC2024_05 {
    ordering_rules: Vec<(usize, usize)>,
    updates: Vec2<usize>,
}

impl AoC2024_05 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2024_05")?;
        Ok(Self::with_str(&input))
    }

    fn with_str(s: &str) -> Self {
        let (ordering_rules, page_numbers) =
            s.split_once("\n\n").expect("Failed to split input data");

        let ordering_rules = ordering_rules
            .split("\n")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.split_once("|").expect("Failed to split ordering rule"))
            .map(|(a, b)| {
                let a = a
                    .parse::<usize>()
                    .expect("Failed to parse first value in ordering rule");
                let b = b
                    .parse::<usize>()
                    .expect("Failed to parse second value in ordering rule");
                (a, b)
            })
            .collect::<Vec<_>>();

        let updates = page_numbers
            .split("\n")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.split(",")
                    .map(|s| s.parse::<usize>().expect("Failed to parse page number"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self {
            ordering_rules,
            updates,
        }
    }
}

type OrderMap = HashMap<usize, HashSet<usize>>;

fn is_correct(input: &[usize], order_map: &OrderMap) -> bool {
    let index_map = make_index_map(input);
    for (index, num) in input.iter().enumerate() {
        let Some(next_set) = order_map.get(num) else {
            continue;
        };
        let is_ok = next_set
            .iter()
            .filter_map(|val| index_map.get(val))
            .all(|val| *val > index);
        if !is_ok {
            return false;
        }
    }
    true
}

fn fix_order(input: &[usize], order_map: &OrderMap) -> Vec<usize> {
    let index_map = make_index_map(input);
    let mut priorities = input.iter().map(|value| (value, 0)).collect::<Vec<_>>();
    for (index, num) in input.iter().enumerate() {
        if let Some(next_set) = order_map.get(num) {
            priorities[index].1 += next_set.iter().filter_map(|val| index_map.get(val)).count();
        }
    }
    priorities.sort_by(|a, b| a.1.cmp(&b.1));
    priorities.iter().map(|x| *x.0).collect::<Vec<_>>()
}

fn make_order_map(ordering_rules: &[(usize, usize)]) -> OrderMap {
    let mut order_map = OrderMap::new();
    for (a, b) in ordering_rules {
        let set = order_map.entry(*a).or_default();
        set.insert(*b);
    }
    order_map
}

fn make_index_map(input: &[usize]) -> HashMap<usize, usize> {
    let mut map = HashMap::<usize, usize>::new();
    for (i, val) in input.iter().enumerate() {
        map.insert(*val, i);
    }
    map
}

fn get_middle(arr: &[usize]) -> usize {
    let middle = arr.len() / 2;
    arr[middle]
}

impl Solution for AoC2024_05 {
    fn part_one(&self) -> String {
        let order_map = make_order_map(&self.ordering_rules);
        self.updates
            .iter()
            .filter(|arr| is_correct(arr, &order_map))
            .map(|arr| get_middle(arr))
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        let order_map = make_order_map(&self.ordering_rules);
        self.updates
            .iter()
            .filter(|arr| !is_correct(arr, &order_map))
            .map(|arr| fix_order(arr, &order_map))
            .map(|arr| get_middle(&arr))
            .sum::<usize>()
            .to_string()
    }

    fn description(&self) -> String {
        "2024/Day 5: Print Queue".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_05_input_load_test() -> io::Result<()> {
        let sol = AoC2024_05::new()?;
        assert!(!sol.ordering_rules.is_empty());
        assert!(!sol.updates.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2024_05_case_1() {
        let puzzle = make_puzzle();
        assert_eq!(puzzle.part_one(), "143");
    }

    #[test]
    fn aoc2024_05_case_2() {
        let puzzle = make_puzzle();
        assert_eq!(puzzle.part_two(), "123");
    }

    fn make_puzzle() -> AoC2024_05 {
        let puzzle = AoC2024_05::with_str(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );
        assert_eq!(21, puzzle.ordering_rules.len());
        assert_eq!(6, puzzle.updates.len());
        puzzle
    }

    #[test]
    fn aoc2024_05_correctness() -> io::Result<()> {
        let sol = AoC2024_05::new()?;
        assert_eq!(sol.part_one(), "5991");
        assert_eq!(sol.part_two(), "5479");
        Ok(())
    }
}
