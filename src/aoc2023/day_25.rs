use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
use std::io;

type Graph = HashMap<String, HashSet<String>>;

pub struct AoC2023_25 {
    graph: Graph,
}

impl AoC2023_25 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_25")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let mut graph = Graph::new();
        lines
            .iter()
            .map(|s| s.split_once(": ").expect("Incorrect format (1)"))
            .for_each(|(v, adj)| {
                let adj = adj.split(' ').collect::<Vec<_>>();
                for item in adj {
                    {
                        let entry = graph.entry(v.to_string()).or_default();
                        entry.insert(item.to_string());
                    }
                    {
                        let entry = graph.entry(item.to_string()).or_default();
                        entry.insert(v.to_string());
                    }
                }
            });
        Self { graph }
    }
}

impl Solution for AoC2023_25 {
    // fn part_one(&self) -> String {
    // }

    fn part_two(&self) -> String {
        "".to_string()
    }

    fn description(&self) -> String {
        "AoC 2023/Day 25: Snowverload".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_25_input_load_test() -> io::Result<()> {
        let sol = AoC2023_25::new()?;
        assert!(!sol.graph.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_25_ex() {
        let lines = [
            "jqt: rhn xhk nvd",
            "rsh: frs pzl lsr",
            "xhk: hfx",
            "cmg: qnr nvd lhk bvb",
            "rhn: xhk bvb hfx",
            "bvb: xhk hfx",
            "pzl: lsr hfx nvd",
            "qnr: nvd",
            "ntq: jqt hfx bvb xhk",
            "nvd: lhk",
            "lsr: lhk",
            "rzs: qnr cmg lsr rsh",
            "frs: qnr lhk lsr",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let puzzle = AoC2023_25::with_lines(&lines);
        assert_eq!(puzzle.part_one(), "54");
    }

    #[test]
    fn aoc2023_25_correctness() -> io::Result<()> {
        let sol = AoC2023_25::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
