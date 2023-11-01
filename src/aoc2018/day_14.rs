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

    // fn part_two(&self) -> String {
    // }

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_14_correctness() -> io::Result<()> {
        let sol = AoC2018_14::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2018_14_ex1() {
        assert_eq!(produce_recipes(9), "5158916779".to_string());
        assert_eq!(produce_recipes(5), "0124515891".to_string());
        assert_eq!(produce_recipes(18), "9251071085".to_string());
        assert_eq!(produce_recipes(2018), "5941429882".to_string());
    }
}
