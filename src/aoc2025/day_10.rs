use crate::solution::Solution;
use crate::utils::*;

use std::collections::VecDeque;
use std::io;

struct MachineConfiguration {
    indicators: Vec<bool>,
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
            .map(|x| indicator_setup_presses(&x.indicators, &x.buttons))
            .sum::<usize>()
            .to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 10: Factory".to_string()
    }
}

struct State {
    indicators: Vec<bool>,
    taps: usize,
}

impl State {
    fn with(len: usize) -> Self {
        Self {
            indicators: vec![false; len],
            taps: 0,
        }
    }

    fn tap(&self, wiring: &[usize]) -> State {
        let mut indicators = self.indicators.clone();
        for &idx in wiring {
            indicators[idx] = !indicators[idx];
        }
        Self {
            indicators,
            taps: self.taps + 1,
        }
    }
}

fn indicator_setup_presses(target: &[bool], buttons: &[Vec<usize>]) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(State::with(target.len()));
    while let Some(state) = queue.pop_back() {
        for button in buttons {
            let next_state = state.tap(button);
            if next_state.indicators == target {
                return next_state.taps;
            }
            queue.push_front(next_state);
        }
    }
    unreachable!()
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

fn parse_indicators(s: &str) -> Vec<bool> {
    let mut iter = s.chars();
    iter.next();
    iter.next_back();
    iter.map(|ch| ch == '#').collect::<Vec<_>>()
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
    fn aoc2025_10_parse_indicators_test() {
        let arr = parse_indicators("[.##.]");
        assert_eq!(arr, [false, true, true, false]);
    }

    #[test]
    fn aoc2025_10_parse_machine_config() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let config = MachineConfiguration::from(input);
        assert_eq!(config.indicators, [false, true, true, false]);
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
