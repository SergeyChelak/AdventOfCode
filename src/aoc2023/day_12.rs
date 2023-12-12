use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct SpringGroup {
    pattern: Vec<char>,
    sizes: Vec<usize>,
}

impl From<&str> for SpringGroup {
    fn from(value: &str) -> Self {
        let (springs, sizes) = value.split_once(' ').expect("Separator not found");
        let sizes = sizes
            .split(',')
            .map(|s| s.parse::<usize>().expect("Length should be integer"))
            .collect::<Vec<_>>();
        Self {
            pattern: springs.chars().collect(),
            sizes,
        }
    }
}

impl SpringGroup {
    // Original solution is here:
    // https://github.com/hyper-neutrino/advent-of-code/blob/main/2023/day12p2.py
    fn arrangements_count(&self) -> usize {
        fn count<'a>(
            pattern: &'a [char],
            sizes: &'a [usize],
            memo: &mut HashMap<(&'a [char], &'a [usize]), usize>,
        ) -> usize {
            if pattern.is_empty() {
                return if sizes.is_empty() { 1 } else { 0 };
            }
            if sizes.is_empty() {
                return if pattern.contains(&'#') { 0 } else { 1 };
            }
            let key = (pattern, sizes);
            if let Some(val) = memo.get(&key) {
                return *val;
            }
            let mut result = 0usize;
            if matches!(pattern[0], '.' | '?') {
                result += count(&pattern[1..], sizes, memo);
            }
            if matches!(pattern[0], '#' | '?')
                && sizes[0] <= pattern.len()
                && !pattern[..sizes[0]].contains(&'.')
                && (sizes[0] == pattern.len() || pattern[sizes[0]] != '#')
            {
                let next = if sizes[0] + 1 < pattern.len() {
                    &pattern[sizes[0] + 1..]
                } else {
                    &[]
                };
                result += count(next, &sizes[1..], memo)
            }
            memo.insert(key, result);
            result
        }

        count(&self.pattern, &self.sizes, &mut HashMap::new())
    }

    #[allow(clippy::useless_vec)]
    fn unfolded(&self) -> Self {
        let sizes = vec![self.sizes.clone(); 5]
            .iter()
            .flatten()
            .copied()
            .collect::<Vec<_>>();

        let sep = ['?'];
        let pattern = vec![self.pattern.clone(); 5].join(&sep[..]);

        Self { sizes, pattern }
    }
}

pub struct AoC2023_12 {
    input: Vec<SpringGroup>,
}

impl AoC2023_12 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_12")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let input = lines
            .iter()
            .map(|s| SpringGroup::from(s.as_str()))
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2023_12 {
    fn part_one(&self) -> String {
        self.input
            .iter()
            .map(|elem| elem.arrangements_count())
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.input
            .iter()
            .map(|elem| elem.unfolded())
            .map(|elem| elem.arrangements_count())
            .sum::<usize>()
            .to_string()
    }

    fn description(&self) -> String {
        "AoC 2023/Day 12: Hot Springs".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_12_input_load_test() -> io::Result<()> {
        let sol = AoC2023_12::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_12_arrangements() {
        [
            ("???.### 1,1,3", 1),
            (".??..??...?##. 1,1,3", 4),
            ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            ("????.#...#... 4,1,1", 1),
            ("????.######..#####. 1,6,5", 4),
            ("?###???????? 3,2,1", 10),
        ]
        .iter()
        .for_each(|(inp, expected)| {
            let group = SpringGroup::from(*inp);
            assert_eq!(group.arrangements_count(), *expected);
        });
    }

    #[test]
    fn aoc2023_12_ex1() {
        assert_eq!(puzzle().part_one(), "21");
    }

    #[test]
    fn aoc2023_12_ex2() {
        assert_eq!(puzzle().part_two(), "525152");
    }

    fn puzzle() -> AoC2023_12 {
        let lines = [
            "???.### 1,1,3",
            ".??..??...?##. 1,1,3",
            "?#?#?#?#?#?#?#? 1,3,1,6",
            "????.#...#... 4,1,1",
            "????.######..#####. 1,6,5",
            "?###???????? 3,2,1",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        AoC2023_12::with_lines(&lines)
    }

    #[test]
    fn aoc2023_12_correctness() -> io::Result<()> {
        let sol = AoC2023_12::new()?;
        assert_eq!(sol.part_one(), "7361");
        assert_eq!(sol.part_two(), "83317216247365");
        Ok(())
    }
}
