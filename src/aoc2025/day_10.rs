use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, VecDeque};
use std::io;

struct MachineConfiguration {
    indicators: usize,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

pub struct AoC2025_10 {
    input: Vec<MachineConfiguration>,
}

impl AoC2025_10 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2025_10")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(MachineConfiguration::from)
            .collect();
        Self { input }
    }
}

impl Solution for AoC2025_10 {
    fn part_one(&self) -> String {
        self.input
            .iter()
            .filter_map(|x| indicator_setup_presses(x.indicators, &x.buttons))
            .sum::<usize>()
            .to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 10: Factory".to_string()
    }
}

fn bitmap(arr: &[usize]) -> usize {
    arr.iter().fold(0, |acc, x| acc | (1 << *x))
}

fn indicator_setup_presses(target: usize, buttons: &[Vec<usize>]) -> Option<usize> {
    let bitmaps = buttons.iter().map(|arr| bitmap(arr)).collect::<Vec<_>>();
    let mut queue = VecDeque::new();
    queue.push_back(0usize);
    let mut count_map = HashMap::<usize, usize>::new();
    count_map.insert(0usize, 0);
    while let Some(state) = queue.pop_back() {
        let count = 1 + count_map.get(&state).expect("Value must be preserved");
        for bitmap in &bitmaps {
            let next = state ^ *bitmap;
            if count_map.contains_key(&next) {
                continue;
            }
            if next == target {
                return Some(count);
            }
            count_map.insert(next, count);
            queue.push_front(next);
        }
    }
    None
}

impl From<&str> for MachineConfiguration {
    fn from(value: &str) -> Self {
        let mut iter = value.split(' ');
        let indicators =
            parse_indicators(iter.next().expect("Empty str for machine configuration"));
        let joltage = parse_csv(
            iter.next_back()
                .expect("Not enough data for machine config"),
        );
        let buttons = iter.map(parse_csv).collect::<Vec<_>>();
        Self {
            indicators,
            buttons,
            joltage,
        }
    }
}

fn parse_csv(s: &str) -> Vec<usize> {
    remove_first_and_last(s)
        .split(',')
        .map(|val| val.parse::<usize>().expect("Invalid input format"))
        .collect()
}

fn parse_indicators(s: &str) -> usize {
    let mut iter = s.chars();
    iter.next();
    iter.next_back();
    iter.rev()
        .fold(0usize, |acc, ch| acc << 1 | if ch == '#' { 1 } else { 0 })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2025_10_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2025_10_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "498");
        Ok(())
    }

    #[test]
    fn aoc2025_10_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2025_10_bitmap_button_test() {
        assert_eq!(0b1000, bitmap(&[3]));
        assert_eq!(0b1010, bitmap(&[1, 3]));
        assert_eq!(0b101, bitmap(&[0, 2]));
    }

    #[test]
    fn aoc2025_10_parse_machine_config() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let config = MachineConfiguration::from(input);
        assert_eq!(config.indicators, 0b110);
        assert_eq!(config.joltage, [3, 5, 4, 7]);
        assert_eq!(config.buttons.len(), 6);
        assert_eq!(config.buttons[0], [3]);
        assert_eq!(config.buttons[1], [1, 3]);
        assert_eq!(config.buttons[2], [2]);
        assert_eq!(config.buttons[3], [2, 3]);
        assert_eq!(config.buttons[4], [0, 2]);
        assert_eq!(config.buttons[5], [0, 1]);
    }

    fn make_solution() -> io::Result<AoC2025_10> {
        AoC2025_10::new()
    }
}
