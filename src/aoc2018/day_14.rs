use crate::solution::Solution;

use std::io;

pub struct AoC2018_14 {
    input: usize,
}

impl AoC2018_14 {
    pub fn new() -> io::Result<Self> {
        Ok(Self { input: 909441 })
    }
}

impl Solution for AoC2018_14 {
    fn part_one(&self) -> String {
        produce_recipes(self.input)
    }

    fn part_two(&self) -> String {
        count_recipes(&self.input.to_string()).to_string()
    }

    fn description(&self) -> String {
        "AoC 2018/Day 14: Chocolate Charts".to_string()
    }
}

fn produce_recipes(count: usize) -> String {
    let mut scoreboard: Vec<usize> = Vec::with_capacity(10_000);
    scoreboard.push(3);
    scoreboard.push(7);
    let mut pos_a = 0;
    let mut pos_b = 1;
    loop {
        let sum = scoreboard[pos_a] + scoreboard[pos_b];
        if sum > 9 {
            scoreboard.push(sum / 10);
            scoreboard.push(sum % 10);
        } else {
            scoreboard.push(sum);
        }
        let len = scoreboard.len();
        if len >= 10 + count {
            break;
        }
        pos_a = (pos_a + scoreboard[pos_a] + 1) % len;
        pos_b = (pos_b + scoreboard[pos_b] + 1) % len;
    }
    scoreboard
        .iter()
        .skip(count)
        .take(10)
        .map(|val| val.to_string())
        .collect::<String>()
}

fn count_recipes(target: &str) -> usize {
    let mut scoreboard: Vec<usize> = Vec::with_capacity(100_000);
    scoreboard.push(3);
    scoreboard.push(7);
    let mut pos_a = 0;
    let mut pos_b = 1;
    let target_len = target.len();
    let target_digits = target
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    'outer: loop {
        let sum = scoreboard[pos_a] + scoreboard[pos_b];
        let mut new_digits = 1usize;
        if sum > 9 {
            new_digits += 1;
            scoreboard.push(sum / 10);
            scoreboard.push(sum % 10);
        } else {
            scoreboard.push(sum);
        }
        let len = scoreboard.len();
        pos_a = (pos_a + scoreboard[pos_a] + 1) % len;
        pos_b = (pos_b + scoreboard[pos_b] + 1) % len;
        if len >= target_len {
            for skips in 0..new_digits {
                let equals = target_digits
                    .iter()
                    .rev()
                    .zip(scoreboard.iter().rev().skip(skips).take(target_len))
                    .fold(true, |acc, (a, b)| acc && (*a == *b));
                if equals {
                    break 'outer len - target_len - skips;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_14_correctness() -> io::Result<()> {
        let sol = AoC2018_14::new()?;
        assert_eq!(sol.part_one(), "2615161213");
        assert_eq!(sol.part_two(), "20403320");
        Ok(())
    }

    #[test]
    fn aoc2018_14_ex1() {
        assert_eq!(produce_recipes(9), "5158916779".to_string());
        assert_eq!(produce_recipes(5), "0124515891".to_string());
        assert_eq!(produce_recipes(18), "9251071085".to_string());
        assert_eq!(produce_recipes(2018), "5941429882".to_string());
    }

    #[test]
    fn aoc2018_14_ex2() {
        assert_eq!(count_recipes("51589"), 9);
        assert_eq!(count_recipes("01245"), 5);
        assert_eq!(count_recipes("92510"), 18);
        assert_eq!(count_recipes("59414"), 2018);
    }
}
