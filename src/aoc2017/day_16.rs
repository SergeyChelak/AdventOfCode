use crate::{solution::Solution, utils::ArraySpin};

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

type CharArray = Vec<char>;

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

    fn perform(&self, times: usize) -> String {
        let mut arr = CharArray::with_capacity(16);
        ('a'..='p').for_each(|ch| arr.push(ch));
        let mut permutations: Vec<String> = Vec::new();
        permutations.push(arr.iter().collect());
        loop {
            for movement in &self.movements {
                match movement {
                    Movement::Spin(count) => arr.spin_right(*count),
                    Movement::Exchange(a, b) => arr.swap(*a, *b),
                    Movement::Partner(a, b) => arr.iter_mut().for_each(|ch| {
                        if *ch == *a {
                            *ch = *b;
                        } else if *ch == *b {
                            *ch = *a;
                        }
                    }),
                }
            }
            let tmp = arr.iter().collect();
            if permutations.contains(&tmp) {
                break;
            }
            permutations.push(tmp);
        }
        let period = permutations.len();
        permutations[times % period].clone()
    }
}

impl Solution for AoC2017_16 {
    fn part_one(&self) -> String {
        self.perform(1)
    }

    fn part_two(&self) -> String {
        self.perform(1000000000)
    }

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
        assert_eq!(sol.part_one(), "ehdpincaogkblmfj");
        assert_eq!(sol.part_two(), "bpcekomfgjdlinha");
        Ok(())
    }
}
