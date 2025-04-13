use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Element {
    center: String,
    orbit: String,
}

impl From<&str> for Element {
    fn from(value: &str) -> Self {
        let (center, orbit) = value.split_once(')').expect("Invalid input format");
        Self {
            center: center.to_string(),
            orbit: orbit.to_string(),
        }
    }
}

pub struct AoC2019_06 {
    input: Vec<Element>,
}

impl AoC2019_06 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2019_06")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(Element::from)
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2019_06 {
    fn part_one(&self) -> String {
        orbit_count(&self.input).to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 6: Universal Orbit Map".to_string()
    }
}

fn orbit_count(input: &[Element]) -> usize {
    let mut length = HashMap::<String, usize>::new();
    let links = make_map(input);
    let mut queue = vec!["COM".to_string()];
    while let Some(node) = queue.pop() {
        let Some(orbits) = links.get(&node) else {
            // leaf node, do nothing
            continue;
        };
        let dist = *length.get(&node).unwrap_or(&0);
        for orbit in orbits {
            length.insert(orbit.clone(), dist + 1);
            queue.push(orbit.clone());
        }
    }
    length.values().sum::<usize>()
}

fn make_map(input: &[Element]) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::<String, Vec<String>>::new();
    for elem in input {
        let entry = map.entry(elem.center.clone()).or_default();
        entry.push(elem.orbit.clone());
    }
    map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_06_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_06_case_1() {
        let puzzle = AoC2019_06::with_lines(&[
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ]);
        assert_eq!(puzzle.part_one(), "42")
    }

    #[test]
    fn aoc2019_06_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "253104");
        Ok(())
    }

    #[test]
    fn aoc2019_06_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_06> {
        AoC2019_06::new()
    }
}
