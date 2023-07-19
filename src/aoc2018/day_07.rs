use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
use std::io;

#[derive(Clone, Copy)]
struct Dependency {
    dependency: char,
    step: char,
}

pub struct AoC2018_07 {
    input: Vec<Dependency>,
}

impl AoC2018_07 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2018_07")?;
        Ok(Self::from_lines(&lines))
    }

    fn from_lines(lines: &[String]) -> Self {
        let input = lines
            .iter()
            .map(|s| {
                let tokens = s.split(' ').collect::<Vec<&str>>();
                let dependency = tokens[1].parse::<char>().expect("2nd token should be char");
                let step = tokens[7].parse::<char>().expect("7th token should be char");
                Dependency { dependency, step }
            })
            .collect();
        Self { input }
    }
}

impl Solution for AoC2018_07 {
    fn part_one(&self) -> String {
        let mut output: Vec<char> = Vec::new();
        let mut step_data = build_steps_data(&self.input);
        loop {
            let finished = find_finished(&step_data, &output);
            if let Some(ch) = finished.first() {
                output.push(*ch);
                step_data.iter_mut().for_each(|(_, set)| {
                    set.remove(ch);
                });
            } else {
                break;
            }
        }
        output.iter().collect()
    }

    fn part_two(&self) -> String {
        todo!()
    }

    fn description(&self) -> String {
        "AoC 2018/Day 7: The Sum of Its Parts".to_string()
    }
}

type StepsData = HashMap<char, HashSet<char>>;

fn build_steps_data(input: &[Dependency]) -> StepsData {
    let mut steps: StepsData = HashMap::new();
    input.iter().for_each(|x| {
        let entry = steps.entry(x.step).or_insert(HashSet::new());
        entry.insert(x.dependency);
        _ = steps.entry(x.dependency).or_insert(HashSet::new());
    });
    steps
}

fn find_finished(step_data: &StepsData, in_use: &[char]) -> Vec<char> {
    let mut finished = step_data
        .iter()
        .filter_map(|(x, set)| {
            if !in_use.contains(x) && set.is_empty() {
                Some(*x)
            } else {
                None
            }
        })
        .collect::<Vec<char>>();
    finished.sort();
    finished
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_07_input_load_test() -> io::Result<()> {
        let sol = AoC2018_07::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_07_correctness() -> io::Result<()> {
        let sol = AoC2018_07::new()?;
        assert_eq!(sol.part_one(), "LAPFCRGHVZOTKWENBXIMSUDJQY");
        // assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2018_07_example1() {
        let lines = example_input();
        let aoc = AoC2018_07::from_lines(&lines);
        assert_eq!(aoc.part_one(), "CABDFE");
    }

    #[test]
    fn aoc2018_07_example2() {
        let lines = example_input();
        let aoc = AoC2018_07::from_lines(&lines);
        // TODO: assert_eq!(???, 15);
    }

    fn example_input() -> Vec<String> {
        [
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin.",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
    }
}
