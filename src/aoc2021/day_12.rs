use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

type Graph = HashMap<String, Vec<String>>;

const START: &str = "start";
const END: &str = "end";

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

    fn calculate<'a>(&'a self, can_visit: impl Fn(&'a str, &Visits<'a>) -> bool) -> usize {
        let mut count = 0;
        let mut visits = setup_minor_visits(&self.input);
        paths_count(&self.input, START, END, &mut count, &can_visit, &mut visits);
        count
    }
}

impl Solution for AoC2021_12 {
    fn part_one(&self) -> String {
        self.calculate(|name, visits| {
            let Some(entry) = visits.get(name) else {
                return true;
            };
            *entry == 0
        })
        .to_string()
    }

    fn part_two(&self) -> String {
        self.calculate(|name, visits| {
            let Some(times) = visits.get(name) else {
                return true;
            };
            if *times == 0 {
                return true;
            }
            if name == START {
                return false;
            }
            if name == END {
                return true;
            }
            visits.values().all(|x| *x < 2)
        })
        .to_string()
    }

    fn description(&self) -> String {
        "Day 12: Passage Pathing".to_string()
    }
}

type Visits<'l> = HashMap<&'l str, usize>;

fn setup_minor_visits<'l>(graph: &'l Graph) -> Visits<'l> {
    let mut visits = Visits::new();
    for node in graph.keys() {
        let key = node.as_str();
        if key == key.to_lowercase() {
            visits.insert(key, 0);
        }
    }
    visits
}

fn paths_count<'a>(
    graph: &'a Graph,
    current: &'a str,
    target: &str,
    count: &mut usize,
    can_visit: &impl Fn(&'a str, &Visits<'a>) -> bool,
    visits: &mut Visits<'a>,
) {
    if current == target {
        *count += 1;
        return;
    }
    let Some(nodes) = graph.get(current) else {
        return;
    };

    visits.entry(current).and_modify(|val| *val += 1);

    for node in nodes.iter() {
        let name = node.as_str();
        if !can_visit(name, visits) {
            continue;
        }
        paths_count(graph, name, target, count, can_visit, visits);
    }

    visits.entry(current).and_modify(|val| *val -= 1);
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
        assert_eq!(sol.part_two(), "116692");
        Ok(())
    }

    #[test]
    fn aoc2021_12_case2() {
        let sol = make_test_solution();
        assert_eq!(sol.part_two(), "36");
    }

    #[test]
    fn aoc2021_12_case1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "10");
    }

    fn make_solution() -> io::Result<AoC2021_12> {
        AoC2021_12::new()
    }

    fn make_test_solution() -> AoC2021_12 {
        #[rustfmt::skip]
        let lines = [
            "start-A",
            "start-b",
            "A-c",
            "A-b",
            "b-d",
            "A-end",
            "b-end",
        ];
        AoC2021_12::parse_lines(&lines)
    }
}
