use crate::solution::Solution;

use std::io;

fn hash(s: &str) -> u32 {
    s.chars().fold(0u32, |acc, ch| {
        let cur = ch as u8 as u32;
        (acc + cur) * 17 % 256
    })
}
pub struct AoC2023_15 {
    input: Vec<String>,
}

impl AoC2023_15 {
    pub fn new() -> io::Result<Self> {
        let input = std::fs::read_to_string("input/aoc2023_15")?;
        Ok(Self::with_str(&input))
    }

    fn with_str(s: &str) -> Self {
        let input = s
            .trim()
            .split(',')
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2023_15 {
    fn part_one(&self) -> String {
        self.input.iter().map(|s| hash(&s)).sum::<u32>().to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 15: Lens Library".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_15_input_load_test() -> io::Result<()> {
        let sol = AoC2023_15::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_15_hash() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn=1"), 30);
    }

    #[test]
    fn aoc2023_15_ex1() {
        let puzzle = AoC2023_15::with_str("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(puzzle.part_one(), "1320");
    }
    #[test]
    fn aoc2023_15_correctness() -> io::Result<()> {
        let sol = AoC2023_15::new()?;
        assert_eq!(sol.part_one(), "516804");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
