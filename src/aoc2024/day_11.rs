use crate::solution::Solution;

use std::fs::read_to_string;
use std::io;

type Int = u64;

pub struct AoC2024_11 {
    numbers: Vec<Int>,
}

impl AoC2024_11 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2024_11")?;
        Ok(Self::with_str(&input))
    }

    fn with_str(s: &str) -> Self {
        let numbers = s
            .split_whitespace()
            .map(|s| s.parse::<Int>().expect("Non numerical value found"))
            .collect::<Vec<_>>();
        Self { numbers }
    }
}

impl Solution for AoC2024_11 {
    fn part_one(&self) -> String {
        let mut array = remap(&self.numbers);
        // dump(&array);
        for _ in 0..25 {
            array = sparse_vec(&array);
            blink(&mut array);
            // dump(&array);
            // break;
        }
        array.iter().filter(|x| x.is_some()).count().to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "2024/Day 11: Plutonian Pebbles".to_string()
    }
}

fn dump(src: &[Option<Int>]) {
    src.iter().filter_map(|x| *x).for_each(|x| print!("{x} "));
    println!()
}

fn remap(src: &[Int]) -> Vec<Option<Int>> {
    src.iter().map(|x| Some(*x)).collect()
}

fn sparse_vec(src: &[Option<Int>]) -> Vec<Option<Int>> {
    let mut sparse = Vec::new();
    for val in src {
        if val.is_none() {
            continue;
        }
        sparse.push(*val);
        sparse.push(None);
    }
    sparse
}

fn blink(array: &mut Vec<Option<Int>>) {
    let mut ptr = 0;
    while ptr < array.len() {
        let Some(value) = array[ptr] else {
            ptr += 1;
            continue;
        };

        if value == 0 {
            array[ptr] = Some(1);
            ptr += 1;
            continue;
        }

        let digits = digits_count(value);
        if digits % 2 == 0 {
            let (a, b) = split(value, digits / 2);
            array[ptr] = Some(a);
            assert!(array[ptr + 1].is_none());
            array[ptr + 1] = Some(b);
            ptr += 2;
            continue;
        }

        array[ptr] = Some(value * 2024);
        ptr += 1;
    }
}

fn split(value: Int, at: u8) -> (Int, Int) {
    let f = pow10(at);
    let a = value / f;
    let b = value % f;
    (a, b)
}

fn pow10(i: u8) -> Int {
    if i == 0 {
        return 1;
    }
    (0..i).fold(1, |acc, _| acc * 10)
}

fn digits_count(value: Int) -> u8 {
    (value as f32).log10() as u8 + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_11_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.numbers.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2024_11_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "183435");
        Ok(())
    }

    #[test]
    fn aoc2024_11_case_1() -> io::Result<()> {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "55312");
        Ok(())
    }

    #[test]
    fn aoc2024_11_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2024_11_digits_count() {
        assert_eq!(1, digits_count(1));
        assert_eq!(2, digits_count(12));
        assert_eq!(3, digits_count(234));
        assert_eq!(4, digits_count(3456));
        assert_eq!(5, digits_count(45678));
    }

    #[test]
    fn aoc2024_11_pow10() {
        assert_eq!(1, pow10(0));
        assert_eq!(10, pow10(1));
        assert_eq!(100, pow10(2));
        assert_eq!(1000, pow10(3));
        assert_eq!(10000, pow10(4));
    }

    #[test]
    fn aoc2024_11_split() {
        assert_eq!(split(10, 1), (1, 0));
        assert_eq!(split(12, 1), (1, 2));
        assert_eq!(split(2340, 2), (23, 40));
        assert_eq!(split(2304, 2), (23, 4));
        assert_eq!(split(2345, 2), (23, 45));
        assert_eq!(split(234567, 3), (234, 567));
    }

    fn make_solution() -> io::Result<AoC2024_11> {
        AoC2024_11::new()
    }

    fn make_test_solution() -> AoC2024_11 {
        AoC2024_11::with_str("125 17")
    }
}
