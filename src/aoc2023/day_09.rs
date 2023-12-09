use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = i64;

pub struct AoC2023_09 {
    input: Vec<Vec<Int>>,
}

impl AoC2023_09 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_09")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let input = lines
            .iter()
            .map(|s| {
                s.split_whitespace()
                    .map(|val| val.parse::<Int>().expect("Int value is expected"))
                    .collect::<Vec<_>>()
            })
            .collect();
        Self { input }
    }
}

impl Solution for AoC2023_09 {
    fn part_one(&self) -> String {
        self.input
            .iter()
            .map(|arr| predict(arr))
            .sum::<Int>()
            .to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 9: Mirage Maintenance".to_string()
    }
}

fn predict(arr: &[Int]) -> Int {
    let mut acc = 0;
    let mut cur = arr;
    let mut out: Vec<Int>;
    loop {
        let len = cur.len();
        acc += cur[len - 1];
        out = cur
            .iter()
            .take(len - 1)
            .zip(cur.iter().skip(1))
            .map(|(a, b)| *b - *a)
            .collect::<Vec<Int>>();
        if out.iter().all(|x| *x == 0) {
            break;
        }
        cur = &out;
    }
    acc
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_09_input_load_test() -> io::Result<()> {
        let sol = AoC2023_09::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_09_ex1() {
        assert_eq!(predict(&[0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(predict(&[1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(predict(&[10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn aoc2023_09_correctness() -> io::Result<()> {
        let sol = AoC2023_09::new()?;
        assert_eq!(sol.part_one(), "2101499000");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
