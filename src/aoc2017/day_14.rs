use crate::solution::Solution;

use std::io;

use super::knot_hash::KnotHashable;

pub struct AoC2017_14 {
    input: String,
}

impl AoC2017_14 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: "oundnydw".to_string(),
        })
    }
}

impl Solution for AoC2017_14 {
    fn part_one(&self) -> String {
        (0..128)
            .fold(0, |acc, i| {
                acc + format!("{}-{i}", self.input)
                    .knot_hash()
                    .chars()
                    .filter_map(|ch| ch.to_digit(16))
                    .map(|x| {
                        (0..4)
                            .map(|offset| x & (0x1 << offset))
                            .map(|val| if val == 0 { 0 } else { 1 })
                            .sum::<u32>()
                    })
                    .sum::<u32>()
            })
            .to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 14: Disk Defragmentation".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_14_example1() {
        let sol = AoC2017_14 {
            input: "flqrgnkx".to_string(),
        };
        assert_eq!(sol.part_one(), "8108");
    }

    #[test]
    fn aoc2017_14_correctness() -> io::Result<()> {
        let sol = AoC2017_14::new()?;
        assert_eq!(sol.part_one(), "8106");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
