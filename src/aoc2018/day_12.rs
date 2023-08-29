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

    fn calculate(&self, steps: usize) -> isize {
        // initialize
        const SIZE: usize = 200;
        const SPACER: isize = 4;
        let mut offset = 0isize;
        let mut count = self.initial_state.len();

        let mut state = vec!['.'; SIZE];
        let mut buffer = state.clone();

        self.initial_state
            .iter()
            .enumerate()
            .for_each(|(i, val)| state[i] = *val);

        let mut loops: HashMap<String, isize> = HashMap::new();
        //
        for step in 0..steps {
            // align pots
            let prefix_dots = prefix_dots_count(&state);
            let suffix_dots = suffix_dots_count(&state, count);

            let mut state_i = 0usize;
            let mut buf_i = 0usize;

            let delta = prefix_dots.abs_diff(SPACER) as isize;
            let mut new_count = count;
            if prefix_dots > SPACER {
                offset += delta;
                state_i = delta as usize;
                new_count -= delta as usize;
            } else {
                offset -= delta;
                buf_i = delta as usize;
                new_count += delta as usize;
            }

            for _ in 0..count {
                buffer[buf_i] = state[state_i];
                buf_i += 1;
                state_i += 1;
            }

            count = new_count;
            if suffix_dots < SPACER {
                count += suffix_dots.abs_diff(SPACER);
            }

            // This is a hack that allows to pass the challenge
            // Theoretically, the loop may include the range of elements
            // and this calculation logic will fail at all
            {
                let pattern = &buffer[0..count].iter().collect::<String>();
                if let Some(offset_prev) = loops.get(pattern) {
                    let delta = offset - offset_prev;
                    offset += (steps - step - 1) as isize * delta;
                    break;
                } else {
                    loops.insert(pattern.to_owned(), offset);
                }

            }

            // clear array for the next state
            state.iter_mut().for_each(|ch| *ch = '.');

            // mutate
            for i in 0..count {
                let s = &buffer[i..i + 5];
                if let Some(val) = self.mutations.get(s) {
                    state[i + 2] = *val;
                }
            }
        }
        // calc results
        state
            .iter()
            .enumerate()
            .filter_map(|(i, ch)| {
                if *ch == '#' {
                    Some(i as isize + offset)
                } else {
                    None
                }
            })
            .sum::<isize>()
    }
}

impl Solution for AoC2018_12 {
    fn part_one(&self) -> String {
        self.calculate(20).to_string()
    }

    fn part_two(&self) -> String {
        self.calculate(50000000000).to_string()
    }

    fn description(&self) -> String {
        "AoC 2018/Day 12: Subterranean Sustainability".to_string()
    }
}

fn prefix_dots_count(chars: &Chars) -> isize {
    let mut sum = 0;
    for ch in chars {
        if *ch == '.' {
            sum += 1;
        } else {
            break;
        }
    }
    sum
}

fn suffix_dots_count(chars: &Chars, end: usize) -> isize {
    let mut sum = 0;
    for ch in chars[..end].iter().rev() {
        if *ch == '.' {
            sum += 1;
        } else {
            break;
        }
    }
    sum
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
        assert_eq!(sol.part_two(), "399999999957");
        Ok(())
    }

    #[test]
    fn aoc2018_12_dots_prefix_count() {
        let cases = [
            ("#..#.#..##......###...###", 0),
            ("..#.#..##......###...###", 2),
            ("......###...###", 6),
            ("......", 6),
            ("", 0),
            ("##", 0),
            (".#", 1),
        ];
        for (val, exp) in cases {
            let case = val.chars().collect::<Chars>();
            assert_eq!(prefix_dots_count(&case), exp);
        }
    }

    #[test]
    fn aoc2018_12_dots_suffix_count() {
        let cases = [
            ("#..#.#..##......###...", usize::MAX, 3),
            ("#..#.#..##......###...", 16, 6),
            ("...", usize::MAX, 3),
            ("......###..####....", usize::MAX, 4),
            ("......", usize::MAX, 6),
            ("", usize::MAX, 0),
            ("##", usize::MAX, 0),
            ("#.", usize::MAX, 1),
        ];
        for (val, end, exp) in cases {
            let case = val.chars().collect::<Chars>();
            let end = if end == usize::MAX { case.len() } else { end };
            assert_eq!(suffix_dots_count(&case, end), exp);
        }
    }
}
