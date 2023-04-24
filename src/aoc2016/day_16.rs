use crate::solution::Solution;

use std::io;

pub struct AoC2016_16 {
    input: String,
}

impl AoC2016_16 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: "11110010111001001".to_string(),
        })
    }

    fn fill(&self, size: usize) -> String {
        checksum(&fill(&self.input, size))
    }
}

impl Solution for AoC2016_16 {
    fn part_one(&self) -> String {
        self.fill(272)
    }

    fn part_two(&self) -> String {
        self.fill(35651584)
    }

    fn description(&self) -> String {
        "AoC 2016/Day 16: Dragon Checksum".to_string()
    }
}

fn fill(s: &str, size: usize) -> String {
    let mut data = s.to_string();
    while data.len() < size {
        data = transform(&data);
    }
    data.truncate(size);
    data
}

fn transform(s: &str) -> String {
    let b = s
        .chars()
        .rev()
        .map(|ch| if ch == '0' { '1' } else { '0' })
        .collect::<String>();
    format!("{s}0{b}")
}

fn checksum(s: &str) -> String {
    let mut data = s.to_string();
    while data.len() % 2 == 0 {
        data = data
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|chunk| if chunk[0] == chunk[1] { '1' } else { '0' })
            .collect::<String>();
    }
    data
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_16_correctness() -> io::Result<()> {
        let sol = AoC2016_16::new()?;
        assert_eq!(sol.part_one(), "01110011101111011");
        assert_eq!(sol.part_two(), "11001111011000111");
        Ok(())
    }

    #[test]
    fn aoc2016_16_transform() {
        assert_eq!(transform("1"), "100");
        assert_eq!(transform("0"), "001");
        assert_eq!(transform("11111"), "11111000000");
        assert_eq!(transform("111100001010"), "1111000010100101011110000");
    }

    #[test]
    fn aoc2016_16_fill() {
        assert_eq!(fill("10000", 20), "10000011110010000111");
    }

    #[test]
    fn aoc2016_16_checksum() {
        assert_eq!(checksum("10000011110010000111"), "01100");
        assert_eq!(checksum("110010110100"), "100");
    }
}
