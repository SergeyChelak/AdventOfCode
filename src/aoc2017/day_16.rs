use crate::solution::Solution;

use std::{fs::read_to_string, io};

enum Movement {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl Movement {
    fn from_str(value: &str) -> Self {
        match &value[0..1] {
            "s" => Self::parse_spin(&value[1..]),
            "x" => Self::parse_exchange(&value[1..]),
            "p" => Self::parse_partner(&value[1..]),
            _ => panic!("Unexpected movement {value}"),
        }
    }

    fn parse_spin(s: &str) -> Self {
        let num = s
            .parse::<usize>()
            .expect("Int value expected for spin movement");
        Self::Spin(num)
    }

    fn parse_exchange(s: &str) -> Self {
        let (a, b) = s
            .split_once('/')
            .expect("Exchange movement should contain 2 params");
        let a = a
            .parse::<usize>()
            .expect("First exchange parameter should be integer");
        let b = b
            .parse::<usize>()
            .expect("Second exchange parameter should be integer");
        Self::Exchange(a, b)
    }

    fn parse_partner(s: &str) -> Self {
        let (a, b) = s
            .split_once('/')
            .expect("Partner movement should contain 2 params");
        let a = a
            .parse::<char>()
            .expect("First exchange parameter should be character");
        let b = b
            .parse::<char>()
            .expect("Second exchange parameter should be character");
        Self::Partner(a, b)
    }
}

pub struct AoC2017_16 {
    movements: Vec<Movement>,
}

impl AoC2017_16 {
    pub fn new() -> io::Result<Self> {
        let movements = read_to_string("input/aoc2017_16")?
            .trim()
            .split(',')
            .map(Movement::from_str)
            .collect();
        Ok(Self { movements })
    }
}

impl Solution for AoC2017_16 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 16: Permutation Promenade".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_16_input_load_test() -> io::Result<()> {
        let sol = AoC2017_16::new()?;
        assert!(!sol.movements.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_16_correctness() -> io::Result<()> {
        let sol = AoC2017_16::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
