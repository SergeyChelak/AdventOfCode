use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

type MutationMap = HashMap<String, char>;

pub struct AoC2018_12 {
    initial_state: String,
    mutations: MutationMap,
}

impl AoC2018_12 {
    pub fn new() -> io::Result<Self> {
        let input = read_file_as_lines("input/aoc2018_12")?;
        Ok(Self::from_strings(&input))
    }

    fn from_strings(input: &[String]) -> Self {
        let (_, initial_state) = input[0].split_once(": ").unwrap();
        let initial_state = initial_state.to_string();
        let mut mutations: MutationMap = HashMap::new();
        for s in &input[2..] {
            let (pattern, output) = s.split_once(" => ").unwrap();
            let pattern = pattern.to_string();
            let output = output.parse::<char>().unwrap();
            mutations.insert(pattern, output);
        }
        Self {
            initial_state,
            mutations,
        }
    }

    fn mutate(&self, value: &str) -> String {
        let len = value.len();
        let mut result = vec!['.'; 40];
        for p in 0..len - 5 {
            let s = &value[p..p + 5];
            if let Some(val) = self.mutations.get(s) {
                result[p + 2] = *val;
            }
        }
        result.iter().collect::<String>()
    }
}

impl Solution for AoC2018_12 {
    fn part_one(&self) -> String {
        let mut sum = 0;
        let mut state = self.initial_state.clone();
        for _ in 0..=20 {
            println!("{state}");
            state = self.mutate(&state);
            sum += state.chars().filter(|ch| *ch == '#').count();
        }
        sum.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 12: Subterranean Sustainability".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_12_input_load_test() -> io::Result<()> {
        let sol = AoC2018_12::new()?;
        assert!(!sol.initial_state.is_empty());
        assert!(!sol.mutations.is_empty());
        assert_eq!(sol.mutations.len(), 32);
        Ok(())
    }

    #[test]
    fn aoc2018_12_example1() {
        let input = [
            "initial state: #..#.#..##......###...###",
            "",
            "...## => #",
            "..#.. => #",
            ".#... => #",
            ".#.#. => #",
            ".#.## => #",
            ".##.. => #",
            ".#### => #",
            "#.#.# => #",
            "#.### => #",
            "##.#. => #",
            "##.## => #",
            "###.. => #",
            "###.# => #",
            "####. => #",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        let sol = AoC2018_12::from_strings(&input);
        assert_eq!(sol.part_one(), "325");
    }

    #[test]
    fn aoc2018_12_correctness() -> io::Result<()> {
        let sol = AoC2018_12::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
