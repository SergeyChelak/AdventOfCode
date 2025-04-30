use crate::solution::Solution;
use crate::utils::*;

use std::collections::VecDeque;
use std::io;

type Int = isize;

enum Shuffle {
    NewStack,
    Cut(Int),
    Increment(Int),
}

impl From<&str> for Shuffle {
    fn from(value: &str) -> Self {
        if value == "deal into new stack" {
            return Self::NewStack;
        }
        let number = value
            .split(' ')
            .last()
            .and_then(|x| x.parse::<Int>().ok())
            .expect("Failed to parse number");
        if value.starts_with("deal with increment") {
            return Shuffle::Increment(number);
        }
        if value.starts_with("cut") {
            return Shuffle::Cut(number);
        }
        panic!("unexpected input {}", value);
    }
}

pub struct AoC2019_22 {
    input: Vec<Shuffle>,
}

impl AoC2019_22 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2019_22")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|val| val.as_ref())
            .map(Shuffle::from)
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2019_22 {
    fn part_one(&self) -> String {
        let mut card_deck = VecDeque::new();
        for i in 0..10007 {
            card_deck.push_back(i);
        }
        shuffle(&self.input, &card_deck)
            .iter()
            .enumerate()
            .find(|(_, val)| **val == 2019)
            .expect("Not found")
            .0
            .to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 22: Slam Shuffle".to_string()
    }
}

type CardDeck = VecDeque<Int>;

fn shuffle(rules: &[Shuffle], input: &CardDeck) -> CardDeck {
    rules.iter().fold(input.clone(), |acc, val| match *val {
        Shuffle::NewStack => new_stack(&acc),
        Shuffle::Cut(n) => cut(&acc, n),
        Shuffle::Increment(n) => increment(&acc, n),
    })
}

fn new_stack(input: &CardDeck) -> CardDeck {
    input.iter().rev().copied().collect()
}

fn cut(input: &CardDeck, n: Int) -> CardDeck {
    let mut result = input.clone();
    for _ in 0..n.abs() {
        if n > 0 {
            let x = result.pop_front().expect("Failed to pop front");
            result.push_back(x);
        } else {
            let x = result.pop_back().expect("Failed to pop back");
            result.push_front(x);
        }
    }
    result
}

fn increment(input: &CardDeck, n: Int) -> CardDeck {
    let n = n as usize;
    let len = input.len();
    let mut result = vec![0; len].into_iter().collect::<VecDeque<_>>();
    for (i, val) in input.iter().enumerate() {
        let pos = (i * n) % len;
        result[pos] = *val;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_22_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_22_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "4649");
        Ok(())
    }

    #[test]
    fn aoc2019_22_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_22> {
        AoC2019_22::new()
    }
}
