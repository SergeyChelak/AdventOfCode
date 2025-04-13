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

    fn part_two(&self) -> String {
        let map = make_distance_map(&self.input);
        let restore_path = |target: &str| -> Vec<String> {
            let mut current = target;
            let mut output = Vec::<String>::new();
            while let Some(info) = map.get(current) {
                output.push(info.prev.clone());
                current = &info.prev;
            }
            output
        };

        let mut you_path = restore_path("YOU");
        let mut san_path = restore_path("SAN");
        loop {
            let (Some(you_last), Some(san_last)) = (you_path.last(), san_path.last()) else {
                break;
            };
            if you_last != san_last {
                break;
            };
            _ = you_path.pop();
            _ = san_path.pop();
        }

        (you_path.len() + san_path.len()).to_string()
    }

    fn description(&self) -> String {
        "Day 6: Universal Orbit Map".to_string()
    }
}

fn orbit_count(input: &[Element]) -> usize {
    let length = make_distance_map(input);
    length.values().map(|x| x.length).sum::<usize>()
}

struct DistanceInfo {
    length: usize,
    prev: String,
}

impl Default for DistanceInfo {
    fn default() -> Self {
        Self {
            length: 0,
            prev: "".to_string(),
        }
    }
}

fn make_distance_map(input: &[Element]) -> HashMap<String, DistanceInfo> {
    let mut distances = HashMap::<String, DistanceInfo>::new();
    let links = make_map(input);
    let mut queue = vec!["COM".to_string()];
    while let Some(node) = queue.pop() {
        let Some(orbits) = links.get(&node) else {
            // leaf node, do nothing
            continue;
        };
        let dist = distances.get(&node).map(|x| x.length).unwrap_or(0);
        for orbit in orbits {
            let info = DistanceInfo {
                length: dist + 1,
                prev: node.clone(),
            };
            distances.insert(orbit.clone(), info);
            queue.push(orbit.clone());
        }
    }
    distances
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
    fn aoc2019_06_case_2() {
        let puzzle = AoC2019_06::with_lines(&[
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ]);
        assert_eq!(puzzle.part_two(), "4")
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
        assert_eq!(sol.part_two(), "499");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_06> {
        AoC2019_06::new()
    }
}
