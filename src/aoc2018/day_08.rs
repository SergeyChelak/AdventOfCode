use crate::solution::Solution;

use std::fs::read_to_string;
use std::io;

type Int = usize;

pub struct AoC2018_08 {
    input: Vec<Int>,
}

impl AoC2018_08 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2018_08")?
            .split_whitespace()
            .map(|x| x.parse::<Int>().expect("Non int value in the input"))
            .collect();

        Ok(Self { input })
    }
}

impl Solution for AoC2018_08 {
    fn part_one(&self) -> String {
        let mut sum = 0;
        sum_metadata(&self.input, &mut 0, &mut sum);
        sum.to_string()
    }

    fn part_two(&self) -> String {
        calc_root_node(&self.input, &mut 0).to_string()
    }

    fn description(&self) -> String {
        "AoC 2018/Day 8: Memory Maneuver".to_string()
    }
}

fn sum_metadata(input: &[Int], pos: &mut usize, sum: &mut Int) {
    let child_count = input[*pos];
    let metadata_count = input[*pos + 1];
    *pos += 2;
    (0..child_count).for_each(|_| sum_metadata(input, pos, sum));
    (0..metadata_count).for_each(|_| {
        *sum += input[*pos];
        *pos += 1;
    });
}

fn calc_root_node(input: &[Int], pos: &mut usize) -> usize {
    let child_count = input[*pos];
    let metadata_count = input[*pos + 1];
    *pos += 2;
    let mut children: Vec<Int> = Vec::with_capacity(child_count);
    for _ in 0..child_count {
        let val = calc_root_node(input, pos);
        children.push(val);
    }
    let metadata = &input[*pos..*pos + metadata_count];
    *pos += metadata_count;
    if child_count == 0 {
        metadata.iter().sum::<usize>()
    } else {
        metadata
            .iter()
            .filter_map(|&x| {
                if x > 0 && x <= child_count {
                    Some(x)
                } else {
                    None
                }
            })
            .map(|x| children[x - 1])
            .sum::<usize>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_08_input_load_test() -> io::Result<()> {
        let sol = AoC2018_08::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_08_correctness() -> io::Result<()> {
        let sol = AoC2018_08::new()?;
        assert_eq!(sol.part_one(), "48260");
        assert_eq!(sol.part_two(), "25981");
        Ok(())
    }

    #[test]
    fn aoc2018_08_example1() {
        let input = input_data();
        let mut sum = 0;
        sum_metadata(&input, &mut 0, &mut sum);
        assert_eq!(sum, 138);
    }

    #[test]
    fn aoc2018_08_example2() {
        let input = input_data();
        let val = calc_root_node(&input, &mut 0);
        assert_eq!(val, 66);
    }

    fn input_data() -> Vec<Int> {
        "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"
            .split_whitespace()
            .map(|x| x.parse::<Int>().expect("Non int value in the input"))
            .collect::<Vec<Int>>()
    }
}
