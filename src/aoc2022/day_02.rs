use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Pair = (char, char);

pub struct AoC2022_02 {
    input: Vec<Pair>,
}

impl AoC2022_02 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2022_02")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|arr| arr.as_ref())
            .map(|row| {
                let mut iter = row.chars();
                let first = iter.next().expect("empty input");
                _ = iter.next(); // skip space
                let second = iter.next().expect("string is too short");
                (first, second)
            })
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2022_02 {
    fn part_one(&self) -> String {
        self.input.iter().map(scores_1).sum::<usize>().to_string()
    }

    fn part_two(&self) -> String {
        self.input.iter().map(scores_2).sum::<usize>().to_string()
    }

    fn description(&self) -> String {
        "Day 2: Rock Paper Scissors".to_string()
    }
}

const WIN: usize = 6;
const DRAW: usize = 3;
const LOSE: usize = 0;

fn scores_1(pair: &Pair) -> usize {
    let player = (pair.1 as u8 - b'X') as usize;
    let opponent = (pair.0 as u8 - b'A') as usize;

    1 + player
        + if player == opponent {
            DRAW
        } else if (player + 3 - 1) % 3 == opponent {
            WIN
        } else {
            LOSE
        }
}

fn scores_2(pair: &Pair) -> usize {
    // X means you need to lose,
    // Y means you need to end the round in a draw, and
    // Z means you need to win
    let index = (pair.0 as u8 - b'A') as usize;
    1 + match pair.1 {
        'X' => LOSE + (index + 3 - 1) % 3,
        'Y' => DRAW + index,
        'Z' => WIN + (index + 3 + 1) % 3,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_02_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2022_02_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "15523");
        Ok(())
    }

    #[test]
    fn aoc2022_02_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "15702");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2022_02> {
        AoC2022_02::new()
    }
}
