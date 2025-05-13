use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet, VecDeque};
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
    fn part_one(&self) -> String {
        let mut frequency = EdgeFrequency::new();

        for vertex in self.graph.keys().take(self.graph.len() / 10) {
            bfs(&self.graph, vertex, &mut frequency);
        }

        let mut arr = frequency.into_iter().collect::<Vec<_>>();
        arr.sort_by_key(|x| x.1);
        arr = arr.into_iter().rev().take(3).collect::<Vec<_>>();

        let mut graph = self.graph.clone();
        arr.iter().for_each(|(edge, _)| {
            graph.get_mut(&edge.0).unwrap().remove(&edge.1);
            graph.get_mut(&edge.1).unwrap().remove(&edge.0);
        });

        let any = &arr.first().unwrap().0;
        let len = bfs(&graph, &any.0, &mut HashMap::new());

        (len * (self.graph.len() - len)).to_string()
    }

    fn description(&self) -> String {
        "AoC 2023/Day 25: Snowverload".to_string()
    }
}

type EdgeFrequency = HashMap<(String, String), usize>;

fn bfs(graph: &Graph, initial: &str, edge_frequency: &mut EdgeFrequency) -> usize {
    let mut dequeue = VecDeque::new();
    dequeue.push_back(initial);
    let mut seen = HashSet::new();
    seen.insert(initial);
    let mut length = 0;
    while let Some(vertex) = dequeue.pop_front() {
        length += 1;
        for other in graph.get(vertex).expect("(1)").iter() {
            if seen.contains(other.as_str()) {
                continue;
            }
            seen.insert(other);
            dequeue.push_back(other);

            let edge = if matches!(vertex.cmp(other), std::cmp::Ordering::Less) {
                (vertex.to_string(), other.to_string())
            } else {
                (other.to_string(), vertex.to_string())
            };
            let count = edge_frequency.entry(edge).or_default();
            *count += 1;
        }
    }
    length
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_25_input_load_test() -> io::Result<()> {
        let sol = AoC2023_25::new()?;
        let graph = sol.graph;
        assert!(!graph.is_empty());
        for (_, adj) in graph {
            assert!(!adj.is_empty());
        }
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
        assert_eq!(sol.part_one(), "562772");
        Ok(())
    }
}
