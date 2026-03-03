use regex::Regex;

use crate::solution::Solution;
use crate::utils::not_found;

use std::collections::{HashMap, VecDeque};
use std::io::{self};

type Int = isize;

struct Valve {
    name: String,
    rate: Int,
    connections: Vec<String>,
}

type Graph = HashMap<String, Valve>;

pub struct AoC2022_16 {
    input: Graph,
}

impl AoC2022_16 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2022_16")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        Self::parse_lines(&data.lines().collect::<Vec<_>>())
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let re =
            Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.+)")
                .expect("Failed to build regex");
        let mut input = Graph::new();
        for line in lines.iter().map(|s| s.as_ref()) {
            let captures = re.captures(line).expect("Failed to get regex captures");
            let name = captures[1].to_string();
            let rate = captures[2].parse::<Int>().expect("Invalid rate value");
            let connections = captures[3]
                .split(", ")
                .map(|x| x.to_string())
                .collect::<Vec<_>>();

            let valve = Valve {
                name: name.clone(),
                rate,
                connections,
            };
            input.insert(name, valve);
        }
        Self { input }
    }

    fn solve(&self, time: Int) -> Memo<'_> {
        assert!(self.input.iter().len() <= 64);
        let distances = calculate_distances(&self.input);

        let active_valves = self
            .input
            .values()
            .filter(|x| x.rate > 0)
            .map(|x| x.name.as_str())
            .collect::<Vec<_>>();

        let state = State {
            valve: "AA",
            time_left: time,
            opened_bitmap: 0,
            current_pressure: 0,
        };

        let mut memo = Memo::new();
        most_pressure(state, &active_valves, &self.input, &distances, &mut memo);
        memo
    }
}

impl Solution for AoC2022_16 {
    fn part_one(&self) -> String {
        let memo = self.solve(30);
        memo.values()
            .max()
            .map(|x| x.to_string())
            .unwrap_or(not_found())
    }

    fn part_two(&self) -> String {
        let arr = self.solve(26).into_iter().collect::<Vec<_>>();

        let mut max_total = 0;
        for (i, first) in arr.iter().enumerate() {
            for second in arr.iter().skip(i + 1) {
                let (mask_f, pressure_f) = first;
                let (mask_s, pressure_s) = second;
                if (mask_f & mask_s) == 0 {
                    max_total = max_total.max(pressure_f + pressure_s);
                }
            }
        }

        max_total.to_string()
    }

    fn description(&self) -> String {
        "Day 16: Proboscidea Volcanium".to_string()
    }
}

type DistanceMap<'l> = HashMap<(&'l str, &'l str), Int>;

fn calculate_distances<'l>(graph: &'l Graph) -> DistanceMap<'l> {
    let mut distances = DistanceMap::new();

    for start in graph.keys().map(|x| x.as_str()) {
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));

        while let Some((name, distance)) = queue.pop_back() {
            let Some(info) = graph.get(name) else {
                continue;
            };
            let key = (start, name);
            if distances.contains_key(&key) {
                continue;
            }
            distances.insert(key, distance);

            for adj in info.connections.iter().map(|x| x.as_str()) {
                if distances.contains_key(&(start, adj)) {
                    continue;
                }
                queue.push_front((adj, distance + 1));
            }
        }
    }
    distances
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct State<'l> {
    valve: &'l str,
    time_left: Int,
    current_pressure: Int,
    opened_bitmap: u64,
}

type Memo<'l> = HashMap<u64, Int>;

fn most_pressure<'l>(
    state: State<'l>,
    valves: &'l [&str],
    graph: &'l Graph,
    distances: &'l DistanceMap,
    memo: &mut Memo<'l>,
) {
    let entry = memo.entry(state.opened_bitmap).or_default();

    if state.current_pressure > *entry {
        *entry = state.current_pressure;
    }

    for (bit, target) in valves
        .iter()
        .enumerate()
        .map(|(i, x)| (1 << i, x))
        .filter(|(bit, _)| state.opened_bitmap & bit == 0)
    {
        let dist = *distances.get(&(state.valve, target)).unwrap();
        let remaining_time = state.time_left - dist - 1;
        if remaining_time <= 0 {
            continue;
        }
        let flow = graph.get(*target).unwrap().rate * remaining_time;

        let next_state = State {
            valve: target,
            time_left: remaining_time,
            opened_bitmap: state.opened_bitmap | bit,
            current_pressure: state.current_pressure + flow,
        };

        most_pressure(next_state, valves, graph, distances, memo);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_16_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2022_16_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "2359");
        Ok(())
    }

    #[test]
    fn aoc2022_16_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "2999");
        Ok(())
    }

    #[test]
    fn aoc2022_16_case_1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "1651")
    }

    #[test]
    fn aoc2022_16_case_2() {
        let sol = make_test_solution();
        assert_eq!(sol.part_two(), "1707")
    }

    fn make_solution() -> io::Result<AoC2022_16> {
        AoC2022_16::new()
    }

    fn make_test_solution() -> AoC2022_16 {
        let lines = [
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
            "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
            "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
            "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
            "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
            "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
            "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
            "Valve HH has flow rate=22; tunnel leads to valve GG",
            "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
            "Valve JJ has flow rate=21; tunnel leads to valve II",
        ];
        AoC2022_16::parse_lines(&lines)
    }
}
