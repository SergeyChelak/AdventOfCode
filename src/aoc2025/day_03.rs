use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = u32;
type Digits = Vec<Int>;

fn str_to_digits(s: &str) -> Digits {
    s.chars()
        .map(|ch| ch.to_digit(10).expect("Input must contain digits only"))
        .collect::<Digits>()
}

pub struct AoC2025_03 {
    input: Vec<Digits>,
}

impl AoC2025_03 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2025_03")?;
        Ok(Self::parse(&lines))
    }

    fn parse<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(str_to_digits)
            .collect::<Vec<_>>();
        Self { input }
    }

    fn calculate(&self, criteria: impl Fn(&[Int]) -> usize) -> String {
        self.input
            .iter()
            .map(|x| criteria(x))
            .sum::<usize>()
            .to_string()
    }
}

impl Solution for AoC2025_03 {
    fn part_one(&self) -> String {
        self.calculate(max_joltage_2)
    }

    fn part_two(&self) -> String {
        self.calculate(max_joltage_12)
    }

    fn description(&self) -> String {
        "Day 3: Lobby".to_string()
    }
}

fn max_joltage_2(battery: &[Int]) -> usize {
    let size = battery.len();
    assert!(size > 1);
    let (mut f, mut s) = (0, 0);
    for (i, elem) in battery.iter().enumerate() {
        assert_ne!(*elem, 0);
        let has_next = i < size - 1;
        if has_next && f < *elem {
            f = *elem;
            s = 0;
        } else if s < *elem {
            s = *elem;
        }
    }
    assert!(f < 10);
    assert!(s < 10);
    (f * 10 + s) as usize
}

fn max_joltage_12(battery: &[Int]) -> usize {
    const SLOTS: usize = 12;
    let mut digits: [Int; SLOTS] = [0; SLOTS];
    let len = battery.len();
    let mut start_idx = 0;
    for (slot, digit) in digits.iter_mut().enumerate() {
        let end_idx = len + slot - SLOTS;
        for (i, item) in battery.iter().enumerate().take(end_idx + 1).skip(start_idx) {
            if item > digit {
                *digit = *item;
                start_idx = i + 1;
            }
        }
    }
    digits
        .iter()
        .inspect(|x| assert_ne!(0, **x))
        .fold(0, |acc, x| acc * 10 + *x as usize)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2025_03_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2025_03_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "17281");
        Ok(())
    }

    #[test]
    fn aoc2025_03_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "171388730430281");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2025_03> {
        AoC2025_03::new()
    }

    #[test]
    fn aoc2025_03_max_joltage_2() {
        let data = [
            ("987654321111111", 98),
            ("811111111111119", 89),
            ("234234234234278", 78),
            ("818181911112111", 92),
        ];
        for (s, exp) in data {
            let digits = str_to_digits(s);
            assert_eq!(max_joltage_2(&digits), exp);
        }
    }

    #[test]
    fn aoc2025_03_max_joltage_12() {
        let data = [
            ("987654321111111", 987654321111),
            ("811111111111119", 811111111119),
            ("234234234234278", 434234234278),
            ("818181911112111", 888911112111),
        ];
        for (s, exp) in data {
            let digits = str_to_digits(s);
            assert_eq!(max_joltage_12(&digits), exp);
        }
    }
}
