use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
use std::io;

type StringPair = (String, String);

pub struct AoC2024_23 {
    input: Vec<StringPair>,
}

impl AoC2024_23 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2024_23")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|s| s.as_ref())
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.split_once("-").expect("Invalid input format"))
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2024_23 {
    fn part_one(&self) -> String {
        let map = make_connection_map(&self.input);
        let mut output = HashSet::new();
        for (key, val) in map.iter() {
            if !key.starts_with('t') {
                continue;
            }
            if val.len() < 2 {
                continue;
            }
            for b in val {
                for c in val {
                    if b == c {
                        continue;
                    }
                    if check_triple(&map, &key, b, c) {
                        let mut arr = vec![key.to_string(), b.to_string(), c.to_string()];
                        arr.sort();
                        output.insert(arr);
                    }
                }
            }
        }
        output.len().to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "2024/Day 23: LAN Party".to_string()
    }
}

type ConnectionMap = HashMap<String, HashSet<String>>;

fn check_triple(map: &ConnectionMap, a: &str, b: &str, c: &str) -> bool {
    let check = |a: &str, b: &str, c: &str| -> bool {
        let Some(a_set) = map.get(a) else {
            return false;
        };
        a_set.contains(b) && a_set.contains(c)
    };

    check(a, b, c) && check(b, a, c) && check(c, a, b)
}

fn make_connection_map(input: &[StringPair]) -> ConnectionMap {
    let mut map = HashMap::<String, HashSet<String>>::new();
    for (first, second) in input {
        let entry = map.entry(first.clone()).or_default();
        entry.insert(second.clone());
        let entry = map.entry(second.clone()).or_default();
        entry.insert(first.clone());
    }
    map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_23_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2024_23_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "1248");
        Ok(())
    }

    #[test]
    fn aoc2024_23_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2024_23> {
        AoC2024_23::new()
    }

    #[test]
    fn aoc2024_23_case_1() {
        let s = make_test_solution();
        assert_eq!(s.part_one(), "7");
    }

    fn make_test_solution() -> AoC2024_23 {
        let lines = [
            "kh-tc", "qp-kh", "de-cg", "ka-co", "yn-aq", "qp-ub", "cg-tb", "vc-aq", "tb-ka",
            "wh-tc", "yn-cg", "kh-ub", "ta-co", "de-co", "tc-td", "tb-wq", "wh-td", "ta-ka",
            "td-qp", "aq-cg", "wq-ub", "ub-vc", "de-ta", "wq-aq", "wq-vc", "wh-yn", "ka-de",
            "kh-ta", "co-tc", "wh-qp", "tb-vc", "td-yn",
        ];
        AoC2024_23::with_lines(&lines)
    }
}
