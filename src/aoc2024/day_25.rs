use crate::solution::Solution;

use std::fs::read_to_string;
use std::io;

type Pins = [u8; 5];
type Heights = Vec<Pins>;

pub struct AoC2024_25 {
    locks: Heights,
    keys: Heights,
}

impl AoC2024_25 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2024_25")?;
        Ok(Self::with_str(&input))
    }

    fn with_str(input: &str) -> Self {
        let tokens = input
            .split("\n\n")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        let mut locks = Heights::new();
        let mut keys = Heights::new();
        for token in tokens {
            let (id, pins) = Self::parse_pins(token);
            if id == '#' {
                locks.push(pins);
            } else {
                keys.push(pins);
            }
        }
        Self { locks, keys }
    }

    fn parse_pins(s: &str) -> (char, Pins) {
        let id = s.chars().next().expect("Invalid pin block");
        let lines = s.split('\n').collect::<Vec<_>>();
        let mut pins = [0; 5];
        for line in lines.iter().skip(1) {
            for (i, ch) in line.chars().enumerate() {
                if ch != id {
                    continue;
                }
                pins[i] += 1;
                assert!(pins[i] < 6);
            }
        }
        if id == '.' {
            pins.iter_mut().for_each(|x| *x = 5 - *x);
        }
        (id, pins)
    }
}

impl Solution for AoC2024_25 {
    fn part_one(&self) -> String {
        let mut total = 0;
        for lock in &self.locks {
            for key in &self.keys {
                if is_fit(lock, key) {
                    total += 1;
                }
            }
        }
        total.to_string()
    }

    fn description(&self) -> String {
        "2024/Day 25: Code Chronicle".to_string()
    }
}

fn is_fit(lock: &Pins, keys: &Pins) -> bool {
    lock.iter().zip(keys.iter()).all(|(x, y)| *x + *y < 6)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_25_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.locks.is_empty());
        assert!(!sol.keys.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2024_25_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "2854");
        Ok(())
    }

    #[test]
    fn aoc2024_25_parse_lock() {
        let (id, pins) = AoC2024_25::parse_pins(
            "#####
.####
.####
.####
.#.#.
.#...
.....",
        );
        assert_eq!(id, '#');
        assert_eq!(pins, [0, 5, 3, 4, 3]);
    }

    #[test]
    fn aoc2024_25_parse_key() {
        let (id, pins) = AoC2024_25::parse_pins(
            ".....
#....
#....
#...#
#.#.#
#.###
#####",
        );
        assert_eq!(id, '.');
        assert_eq!(pins, [5, 0, 2, 1, 3]);
    }

    #[test]
    fn aoc2024_25_fit() {
        assert!(!is_fit(&[0, 5, 3, 4, 3], &[5, 0, 2, 1, 3]));
        assert!(!is_fit(&[0, 5, 3, 4, 3], &[4, 3, 4, 0, 2]));
        assert!(is_fit(&[0, 5, 3, 4, 3], &[3, 0, 2, 0, 1]));
        assert!(!is_fit(&[1, 2, 0, 5, 3], &[5, 0, 2, 1, 3]));
        assert!(is_fit(&[1, 2, 0, 5, 3], &[4, 3, 4, 0, 2]));
        assert!(is_fit(&[1, 2, 0, 5, 3], &[3, 0, 2, 0, 1]));
    }

    fn make_solution() -> io::Result<AoC2024_25> {
        AoC2024_25::new()
    }
}
