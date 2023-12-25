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
    fn part_one(&self) -> String {
        let mut timer = 0;
        let mut time_in = HashMap::new();
        let mut time_back = HashMap::new();

        for vertex in self.graph.keys() {
            if time_in.contains_key(vertex) {
                continue;
            }
            dfs(
                &self.graph,
                vertex,
                None,
                &mut timer,
                &mut time_in,
                &mut time_back,
            );
        }
        todo!()
    }

    fn part_two(&self) -> String {
        // read here
        // https://blog.thomasjungblut.com/graph/mincut/mincut/
        "".to_string()
    }

    fn description(&self) -> String {
        "AoC 2023/Day 25: Snowverload".to_string()
    }
}

type TimeMap = HashMap<String, usize>;

fn dfs(
    graph: &Graph,
    vertex: &str,
    parent: Option<&str>,
    timer: &mut usize,
    time_in: &mut TimeMap,
    time_back: &mut TimeMap,
) {
    *timer += 1;
    time_in.insert(vertex.to_string(), *timer);
    time_back.insert(vertex.to_string(), *timer);
    let Some(adjacent) = graph.get(vertex) else {
        panic!("Not expected case");
    };
    for next in adjacent {
        if parent
            .and_then(|p| if p == next { Some(p) } else { None })
            .is_some()
        {
            continue;
        }
        if let Some(next_in) = time_in.get(next) {
            let v_back = time_back.get(vertex).expect("Vertex should be visited (3)");
            time_back.insert(vertex.to_string(), *v_back.min(next_in));
        } else {
            dfs(graph, next, Some(vertex), timer, time_in, time_back);

            let next_back = time_back.get(next).expect("Next should be visited");
            let v_in = time_in.get(vertex).expect("Vertex should be visited (2)");
            if next_back > v_in {
                println!("Bridge {vertex} - {next}");
            } else {
                println!("{vertex} {next_back} - {next} {v_in}");
            }
            let v_back = time_back.get(vertex).expect("Vertex should be visited (1)");
            time_back.insert(vertex.to_string(), *v_back.min(next_back));
        }
    }
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
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
