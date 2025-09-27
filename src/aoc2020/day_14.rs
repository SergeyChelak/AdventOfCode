use crate::solution::Solution;

use std::{collections::HashMap, io};

type Int = usize;

struct Group {
    mask: String,
    collection: Vec<(usize, Int)>,
}

impl From<&str> for Group {
    fn from(value: &str) -> Self {
        let elems = value.trim().split('\n').collect::<Vec<_>>();

        assert!(elems.len() > 1, "invalid value: {value}");

        let mut collection = Vec::new();

        for row in elems[1..].iter() {
            let row = row.trim();
            assert!(row.starts_with("mem["));

            let (index, v) = row[4..].split_once("] = ").expect("Invalid input format");
            let index = index.parse::<usize>().expect("mem index must be integer");
            let v = v.parse::<Int>().expect("mem value must be integer");

            collection.push((index, v));
        }

        Self {
            mask: elems[0].to_string(),
            collection,
        }
    }
}

fn apply_bit_mask(mask: &str, value: Int) -> Int {
    mask.chars()
        .rev()
        .enumerate()
        .fold(value, |value, (idx, val)| {
            let x = 1 << idx;
            match val {
                '0' => value & !x,
                '1' => value | x,
                _ => value,
            }
        })
}

fn fill(mask: &[char], bit: usize, acc: usize, value: Int, mem: &mut HashMap<usize, Int>) {
    if bit == mask.len() {
        mem.insert(acc, value);
        return;
    }

    let (set, remove) = match mask[bit] {
        '0' => (false, false),
        '1' => (true, false),
        _ => (true, true),
    };

    if !set && !remove {
        fill(mask, bit + 1, acc, value, mem);
        return;
    }

    let x = 1 << bit;
    if set {
        fill(mask, bit + 1, acc | x, value, mem);
    }
    if remove {
        fill(mask, bit + 1, acc & !x, value, mem);
    }
}

pub struct AoC2020_14 {
    input: Vec<Group>,
}

impl AoC2020_14 {
    pub fn new() -> io::Result<Self> {
        let input = std::fs::read_to_string("input/aoc2020_14")?;
        Ok(Self::parse(&input))
    }

    fn parse(data: &str) -> Self {
        let input = data
            .split("mask = ")
            .filter(|x| !x.is_empty())
            .map(Group::from)
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2020_14 {
    fn part_one(&self) -> String {
        let mut memory = HashMap::<usize, Int>::new();
        for group in self.input.iter() {
            for (key, val) in &group.collection {
                memory.insert(*key, apply_bit_mask(&group.mask, *val));
            }
        }

        memory.values().sum::<usize>().to_string()
    }

    fn part_two(&self) -> String {
        let mut memory = HashMap::<usize, Int>::new();
        for group in self.input.iter() {
            let mask = group.mask.chars().rev().collect::<Vec<_>>();
            for (idx, val) in group.collection.iter() {
                fill(&mask, 0, *idx, *val, &mut memory);
            }
        }
        memory.values().sum::<usize>().to_string()
    }

    fn description(&self) -> String {
        "Day 14: Docking Data".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_14_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2020_14_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "13105044880745");
        Ok(())
    }

    #[test]
    fn aoc2020_14_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "3505392154485");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_14> {
        AoC2020_14::new()
    }

    #[test]
    fn aoc2020_14_case2() {
        let data = "mask = 000000000000000000000000000000X1001X
        mem[42] = 100
        mask = 00000000000000000000000000000000X0XX
        mem[26] = 1";
        let sol = AoC2020_14::parse(data);
        assert_eq!(sol.part_two(), "208");
    }
}
