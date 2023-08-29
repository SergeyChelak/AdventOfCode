use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

type Chars = Vec<char>;
type MutationMap = HashMap<Chars, char>;

pub struct AoC2018_12 {
    initial_state: Chars,
    mutations: MutationMap,
}

impl AoC2018_12 {
    pub fn new() -> io::Result<Self> {
        let input = read_file_as_lines("input/aoc2018_12")?;
        Ok(Self::from_strings(&input))
    }

    fn from_strings(input: &[String]) -> Self {
        let (_, initial_state) = input[0].split_once(": ").unwrap();
        let initial_state = initial_state.chars().collect::<Chars>();
        let mut mutations: MutationMap = HashMap::new();
        for s in &input[2..] {
            let (pattern, output) = s.split_once(" => ").unwrap();
            let pattern = pattern.chars().collect::<Chars>();
            let output = output.parse::<char>().unwrap();
            mutations.insert(pattern, output);
        }
        Self {
            initial_state,
            mutations,
        }
    }

    fn mutate(&self, value: &Chars) -> Chars {
        let len = value.len();
        let mut result = vec!['.'; len];
        for p in 0..len - 5 {
            let s = &value[p..p + 5];
            if let Some(val) = self.mutations.get(s) {
                result[p + 2] = *val;
            }
        }
        result
    }
}

impl Solution for AoC2018_12 {
    fn part_one(&self) -> String {
        let mut state = self.initial_state.clone();
        let ext = ['.', '.', '.', '.'];
        let steps = 20;
        for _ in 0..steps {
            state = ext
                .into_iter()
                .chain(state)
                .chain(ext)
                .collect();
            state = self.mutate(&state);
        }
        let offset = ext.len() as isize * steps;
        state
            .iter()
            .enumerate()
            .filter_map(|(i, ch)| {
                if *ch == '#' {
                    Some(i as isize - offset)
                } else {
                    None
                }
            })
            .sum::<isize>()
            .to_string()
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
        assert_eq!(sol.part_one(), "1816");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
