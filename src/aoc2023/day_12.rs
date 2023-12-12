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
        //let mut backtrack = HashSet::new();
        let mut count = 0;
        let mut state = VecDeque::from([self.spring.clone()]);
        while !state.is_empty() {
            let spring = state.pop_front().expect("state isn't expected to be empty");
            match self.matching(&spring) {
                PatternMatch::Yes => {
                    // backtrack.insert(spring);
                    count += 1;
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
        // backtrack.len()
        count
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

    fn unfolded(&self) -> Self {
        let pattern = vec![self.pattern.clone(); 5]
            .iter()
            .flat_map(|x| x)
            .copied()
            .collect::<Vec<_>>();

        let sep = ['?'];
        let spring = vec![self.spring.clone(); 5].join(&sep[..]);

        Self { pattern, spring }
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
            let group = GroupInfo::from(*inp);
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
    fn aoc2023_12_unfold() {
        // let item = GroupInfo::from(".# 1");
        // let unfolded =
    }

    #[test]
    fn aoc2023_12_correctness() -> io::Result<()> {
        let sol = AoC2023_12::new()?;
        assert_eq!(sol.part_one(), "7361");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
