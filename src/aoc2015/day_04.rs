use crate::solution::Solution;

use std::io;
use md5;

pub struct AoC2015_04 {
    input: String,
}

impl AoC2015_04 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: "bgvyzdsv".to_string()
        })
    }
}

impl Solution for AoC2015_04 {
    fn part_one(&self) -> String {        
        for i in 1..usize::MAX {
            let input = format!("{}{i}", self.input);
            let hash = format!("{:x}", md5::compute(input));
            if hash.starts_with("00000") {
                return i.to_string()
            }
        }
        "Not found".to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2015/Day 4".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn AoC2015_04_correctness() -> io::Result<()> {
        assert_eq!(AoC2015_04::new()?.part_one(), "254575");
        Ok(())
    }
}