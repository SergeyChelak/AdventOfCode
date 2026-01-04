use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
use std::io;

type Graph = HashMap<String, Vec<String>>;

pub struct AoC2021_12 {
    input: Graph,
}

impl AoC2021_12 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2021_12")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut input = Graph::new();
        let insert = |graph: &mut Graph, from: &str, to: &str| {
            let entry = graph.entry(from.to_string()).or_default();
            entry.push(to.to_string());
        };

        for line in lines.iter().map(|x| x.as_ref()) {
            let (from, to) = line.trim().split_once('-').expect("Invalid link format");
            insert(&mut input, from, to);
            insert(&mut input, to, from);
        }
        Self { input }
    }
}

impl Solution for AoC2021_12 {
    fn part_one(&self) -> String {
        let mut count = 0;
        paths_count(&self.input, "start", "end", &mut count, &mut HashSet::new());
        count.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 12: Passage Pathing".to_string()
    }
}

fn is_minor_cave(cave: &str) -> bool {
    let Some(ch) = cave.chars().next() else {
        unreachable!()
    };
    ch.is_lowercase()
}

fn paths_count<'a>(
    graph: &'a Graph,
    current: &'a str,
    target: &str,
    count: &mut usize,
    seen: &mut HashSet<&'a str>,
) {
    if current == target {
        *count += 1;
        return;
    }
    if seen.contains(current) {
        return;
    }

    let Some(nodes) = graph.get(current) else {
        return;
    };

    let is_minor = is_minor_cave(current);
    if is_minor {
        seen.insert(current);
    }
    for node in nodes.iter() {
        if seen.contains(node.as_str()) {
            continue;
        }
        paths_count(graph, node.as_str(), target, count, seen);
    }
    if is_minor {
        seen.remove(current);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_12_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2021_12_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "3856");
        Ok(())
    }

    #[test]
    fn aoc2021_12_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2021_12> {
        AoC2021_12::new()
    }
}
