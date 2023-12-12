use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashSet, VecDeque};
use std::io;

enum PatternMatch {
    Yes,
    No,
    Possible,
}

#[derive(Debug)]
struct GroupInfo {
    spring: Vec<char>,
    pattern: Vec<usize>,
}

impl From<&str> for GroupInfo {
    fn from(value: &str) -> Self {
        let (springs, pattern_str) = value.split_once(' ').expect("Separator not found");
        let pattern = pattern_str
            .split(',')
            .map(|s| s.parse::<usize>().expect("Length should be integer"))
            .collect::<Vec<_>>();
        Self {
            spring: springs.chars().collect(),
            pattern,
        }
    }
}

impl GroupInfo {
    fn arrangements_count(&self) -> usize {
        let mut backtrack = HashSet::new();
        let mut state = VecDeque::from([self.spring.clone()]);
        while !state.is_empty() {
            let spring = state.pop_front().expect("state isn't expected to be empty");
            match self.matching(&spring) {
                PatternMatch::Yes => {
                    backtrack.insert(spring);
                }
                PatternMatch::Possible => {
                    let Some(idx) = spring.iter().position(|ch| *ch == '?') else {
                        let s = String::from_iter(spring);
                        println!(">>> {s}");
                        panic!("'?' is expected");
                    };
                    let mut next = spring.clone();
                    next[idx] = '.';
                    state.push_back(next.clone());
                    next[idx] = '#';
                    state.push_back(next);
                }
                _ => {}
            }
        }
        backtrack.len()
    }

    fn matching(&self, spring: &Vec<char>) -> PatternMatch {
        let mut inp_pattern = Vec::new();
        let mut acc = 0usize;
        let mut is_completed = true;
        for ch in spring {
            match ch {
                '?' => {
                    is_completed = false;
                    break;
                }
                '.' => {
                    if acc > 0 {
                        inp_pattern.push(acc);
                    }
                    acc = 0;
                }
                '#' => {
                    acc += 1;
                }
                ch => panic!("Unexpected char {ch}"),
            }
        }
        if is_completed && acc > 0 {
            inp_pattern.push(acc);
        }
        let size = inp_pattern.len();
        if is_completed && size != self.pattern.len() {
            return PatternMatch::No;
        }
        if size > self.pattern.len() {
            return PatternMatch::No;
        }
        for (a, b) in self.pattern[0..size].iter().zip(inp_pattern.iter()) {
            if *a != *b {
                return PatternMatch::No;
            }
        }
        if is_completed {
            PatternMatch::Yes
        } else {
            PatternMatch::Possible
        }
    }
}

pub struct AoC2023_12 {
    input: Vec<GroupInfo>,
}

impl AoC2023_12 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_12")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let input = lines
            .iter()
            .map(|s| GroupInfo::from(s.as_str()))
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

    // fn part_two(&self) -> String {
    // }

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
            let group = GroupInfo::from(*inp);
            assert_eq!(group.arrangements_count(), *expected);
        });
    }

    #[test]
    fn aoc2023_12_ex1() {
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
        let puzzle = AoC2023_12::with_lines(&lines);
        assert_eq!(puzzle.part_one(), "21");
    }

    #[test]
    fn aoc2023_12_correctness() -> io::Result<()> {
        let sol = AoC2023_12::new()?;
        assert_eq!(sol.part_one(), "7361");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
