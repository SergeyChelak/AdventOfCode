use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

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
        paths_count(&self.input, "you", &mut HashMap::new()).to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 11: Reactor".to_string()
    }
}

fn paths_count(
    connections: &Connections,
    device: &str,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if device == "out" {
        return 1;
    }
    if let Some(count) = memo.get(device) {
        return *count;
    }
    let count = connections
        .get(device)
        .expect("Corrupted input data")
        .iter()
        .map(|other| paths_count(connections, other, memo))
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
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2025_11> {
        AoC2025_11::new()
    }
}
