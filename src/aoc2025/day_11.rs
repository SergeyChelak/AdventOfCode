use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::{io, mem};

type Connections = HashMap<String, Vec<String>>;

pub struct AoC2025_11 {
    input: Connections,
}

impl AoC2025_11 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2025_11")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut connections = Connections::new();
        lines
            .iter()
            .map(|x| x.as_ref())
            .map(Self::parse_line)
            .for_each(|(k, v)| {
                connections.insert(k, v);
            });
        Self { input: connections }
    }

    fn parse_line(line: &str) -> (String, Vec<String>) {
        let (key, value) = line.split_once(':').expect("Invalid input format");
        let values = value
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        (key.to_string(), values)
    }
}

impl Solution for AoC2025_11 {
    fn part_one(&self) -> String {
        paths_count(&self.input, "you", "out", &mut HashMap::new()).to_string()
    }

    fn part_two(&self) -> String {
        let mut first_device = "fft";
        let mut second_device = "dac";
        let intermediate = loop {
            let value = paths_count(
                &self.input,
                first_device,
                second_device,
                &mut HashMap::new(),
            );
            if value > 0 {
                break value;
            }
            mem::swap(&mut first_device, &mut second_device);
        };
        let begin = paths_count(&self.input, "svr", first_device, &mut HashMap::new());
        let end = paths_count(&self.input, second_device, "out", &mut HashMap::new());

        (begin * intermediate * end).to_string()
    }

    fn description(&self) -> String {
        "Day 11: Reactor".to_string()
    }
}

fn paths_count(
    connections: &Connections,
    device: &str,
    destination: &str,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if device == destination {
        return 1;
    }
    if let Some(count) = memo.get(device) {
        return *count;
    }
    let Some(devices) = connections.get(device) else {
        return 0;
    };
    let count = devices
        .iter()
        .map(|other| paths_count(connections, other, destination, memo))
        .sum::<usize>();
    memo.insert(device.to_string(), count);
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2025_11_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2025_11_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "753");
        Ok(())
    }

    #[test]
    fn aoc2025_11_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "450854305019580");
        Ok(())
    }

    #[test]
    fn aoc2025_11_case_2() {
        let lines = [
            "svr: aaa bbb",
            "aaa: fft",
            "fft: ccc",
            "bbb: tty",
            "tty: ccc",
            "ccc: ddd eee",
            "ddd: hub",
            "hub: fff",
            "eee: dac",
            "dac: fff",
            "fff: ggg hhh",
            "ggg: out",
            "hhh: out",
        ];
        let sol = AoC2025_11::parse_lines(&lines);
        assert_eq!(sol.part_two(), "2")
    }

    fn make_solution() -> io::Result<AoC2025_11> {
        AoC2025_11::new()
    }
}
