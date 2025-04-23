use crate::solution::Solution;

use std::fs::read_to_string;
use std::io;

type Int = i64;

pub struct AoC2019_16 {
    input: Vec<Int>,
}

impl AoC2019_16 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2019_16")?;
        Ok(Self::with_str(&input))
    }

    fn with_str(s: &str) -> Self {
        let input = s
            .trim()
            .chars()
            .map(|x| x.to_digit(10).map(|x| x as Int).expect("Bad input"))
            .collect::<Vec<_>>();
        Self { input }
    }

    fn process(&self, phases: usize) -> String {
        process(&self.input, phases)
            .iter()
            .take(8)
            .map(|x| x.to_string())
            .collect::<String>()
    }
}

impl Solution for AoC2019_16 {
    fn part_one(&self) -> String {
        self.process(100)
    }

    fn part_two(&self) -> String {
        let offset = self
            .input
            .iter()
            .take(7)
            .fold(0usize, |acc, x| acc * 10 + *x as usize);

        let mut input = (0..10_000)
            .flat_map(|_| self.input.clone())
            .collect::<Vec<Int>>();
        assert!(offset > input.len() / 2, "Solution is not applicable");
        let input = &mut input[offset..];
        for _ in 0..100 {
            let mut total = 0;
            input.iter_mut().rev().for_each(|x| {
                total += *x;
                *x = total % 10;
            });
        }

        input
            .iter()
            .take(8)
            .map(|x| x.to_string())
            .collect::<String>()
    }

    fn description(&self) -> String {
        "Day 16: Flawed Frequency Transmission".to_string()
    }
}

fn process(input: &[Int], phases: usize) -> Vec<Int> {
    let mut data = input.to_owned();
    for _ in 0..phases {
        data = process_phase(&data);
    }
    data
}

fn process_phase(input: &[Int]) -> Vec<Int> {
    (0..input.len())
        .map(|step| process_step(input, step))
        .collect::<Vec<_>>()
}

fn process_step(input: &[Int], step: usize) -> Int {
    input
        .iter()
        .enumerate()
        .map(|(i, val)| *val * pattern(step + 1, i + 1))
        .sum::<Int>()
        .abs()
        % 10
}

fn pattern(step: usize, at: usize) -> Int {
    assert!(step > 0);
    let base_pattern = [0, 1, 0, -1];
    let period = base_pattern.len() * step;
    let mut position = at % period;
    position /= step;
    assert!(
        position < base_pattern.len(),
        "f({step},{at}) :: position {position}, len {}, period {period}",
        base_pattern.len()
    );
    base_pattern[position]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_16_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_16_pattern_test() {
        {
            let output = (0..8).map(|x| pattern(1, x)).collect::<Vec<_>>();
            assert_eq!(output, [0, 1, 0, -1, 0, 1, 0, -1]);
        }
        {
            let output = (0..8).map(|x| pattern(2, x)).collect::<Vec<_>>();
            assert_eq!(output, [0, 0, 1, 1, 0, 0, -1, -1]);
        }
        {
            let output = (0..12).map(|x| pattern(3, x)).collect::<Vec<_>>();
            assert_eq!(output, [0, 0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1]);
        }
        {
            let output = (0..24).map(|x| pattern(3, x)).collect::<Vec<_>>();
            assert_eq!(
                output,
                [0, 0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, 0, 0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1]
            );
        }
    }

    #[test]
    fn aoc2019_16_case_1() {
        let puzzle = AoC2019_16::with_str("12345678");
        assert_eq!("01029498", puzzle.process(4));
    }

    #[test]
    fn aoc2019_16_case_2() {
        let puzzle = AoC2019_16::with_str("80871224585914546619083218645595");
        assert_eq!(puzzle.part_one(), "24176176");
    }

    #[test]
    fn aoc2019_16_case_3() {
        let puzzle = AoC2019_16::with_str("19617804207202209144916044189917");
        assert_eq!(puzzle.part_one(), "73745418");
    }

    #[test]
    fn aoc2019_16_case_pt2_1() {
        let puzzle = AoC2019_16::with_str("03036732577212944063491565474664");
        assert_eq!(puzzle.part_two(), "84462026");
    }

    #[test]
    fn aoc2019_16_case_pt2_2() {
        let puzzle = AoC2019_16::with_str("02935109699940807407585447034323");
        assert_eq!(puzzle.part_two(), "78725270");
    }

    #[test]
    fn aoc2019_16_case_pt2_3() {
        let puzzle = AoC2019_16::with_str("03081770884921959731165446850517");
        assert_eq!(puzzle.part_two(), "53553731");
    }

    #[test]
    fn aoc2019_16_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "27229269");
        Ok(())
    }

    #[test]
    fn aoc2019_16_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "26857164");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_16> {
        AoC2019_16::new()
    }
}
