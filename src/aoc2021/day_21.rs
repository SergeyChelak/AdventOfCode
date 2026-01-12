use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

pub struct AoC2021_21 {
    first: usize,
    second: usize,
}

impl AoC2021_21 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2021_21")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let position = |s: &str| -> usize {
            let (_, val) = s.split_once(": ").expect("Invalid input");
            val.parse::<usize>().expect("positing must be integer")
        };
        Self {
            first: position(lines[0].as_ref()),
            second: position(lines[1].as_ref()),
        }
    }
}

impl Solution for AoC2021_21 {
    fn part_one(&self) -> String {
        simulate(self.first, self.second).to_string()
    }

    fn part_two(&self) -> String {
        let first = PlayerState::new(self.first);
        let second = PlayerState::new(self.second);
        let frequency_map = make_frequency_map();
        let stats = simulate_21(&frequency_map, &first, &second, 0, &mut Memo::new());
        stats[0].max(stats[1]).to_string()
    }

    fn description(&self) -> String {
        "Day 21: Dirac Dice".to_string()
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct PlayerState {
    position: usize,
    score: usize,
}

impl PlayerState {
    fn new(position: usize) -> Self {
        Self { position, score: 0 }
    }

    fn moved_by(&self, value: usize) -> Self {
        let pos = self.position + value;
        let position = 1 + (pos - 1) % 10;
        assert!(position > 0 && position < 11);
        let score = self.score + position;
        Self { position, score }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Key {
    first: PlayerState,
    second: PlayerState,
    turn_idx: usize,
}

impl Key {
    fn new(first: &PlayerState, second: &PlayerState, turn_idx: usize) -> Self {
        Self {
            first: first.clone(),
            second: second.clone(),
            turn_idx,
        }
    }
}

type Memo = HashMap<Key, [usize; 2]>;
type FrequencyMap = HashMap<usize, usize>;

fn make_frequency_map() -> FrequencyMap {
    let mut f_map = FrequencyMap::new();
    for val1 in [1, 2, 3] {
        for val2 in [1, 2, 3] {
            for val3 in [1, 2, 3] {
                let key = val3 + val2 + val1;
                *f_map.entry(key).or_default() += 1;
            }
        }
    }
    f_map
}

fn simulate_21(
    frequency_map: &FrequencyMap,
    first: &PlayerState,
    second: &PlayerState,
    idx: usize,
    memo: &mut Memo,
) -> [usize; 2] {
    let mut wins = [0, 0];
    let opposite_idx = (idx + 1) % 2;
    {
        let arr = [first, second];
        if arr[opposite_idx].score >= 21 {
            wins[opposite_idx] += 1;
            return wins;
        }
    }

    let key = Key::new(first, second, idx);

    if let Some(result) = memo.get(&key) {
        return *result;
    }

    for (val, times) in frequency_map {
        let (f, s) = match idx {
            0 => (&first.moved_by(*val), second),
            1 => (first, &second.moved_by(*val)),
            _ => unreachable!(),
        };
        let tmp = simulate_21(frequency_map, f, s, opposite_idx, memo);
        wins[0] += tmp[0] * times;
        wins[1] += tmp[1] * times;
    }
    memo.insert(key, wins);
    wins
}

fn simulate(start_first: usize, start_second: usize) -> usize {
    let mut counter = 0;
    let mut next = move || -> usize {
        let v = counter % 100 + 1;
        counter += 1;
        v
    };

    let mut stats = [
        PlayerState::new(start_first),
        PlayerState::new(start_second),
    ];

    let mut ptr = 0;

    let mut rolls = 0;
    while stats.iter().all(|x| x.score < 1000) {
        let offset = next() + next() + next();
        stats[ptr] = stats[ptr].moved_by(offset);
        rolls += 3;
        ptr = (ptr + 1) % 2;
    }
    rolls * stats[0].score.min(stats[1].score)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_21_input_load_test() -> io::Result<()> {
        _ = make_solution()?;
        Ok(())
    }

    #[test]
    fn aoc2021_21_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "675024");
        Ok(())
    }

    #[test]
    fn aoc2021_21_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "570239341223618");
        Ok(())
    }

    #[test]
    fn aoc2021_21_case1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "739785");
    }

    #[test]
    fn aoc2021_21_case2() {
        let sol = make_test_solution();
        assert_eq!(sol.part_two(), "444356092776315");
    }

    fn make_test_solution() -> AoC2021_21 {
        AoC2021_21 {
            first: 4,
            second: 8,
        }
    }

    fn make_solution() -> io::Result<AoC2021_21> {
        AoC2021_21::new()
    }
}
