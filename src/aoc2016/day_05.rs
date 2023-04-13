use crate::solution::Solution;
use md5;
use std::io;

pub struct AoC2016_05 {
    door_id: String,
}

impl AoC2016_05 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            door_id: String::from("ojvtpuvg"),
        })
    }
}

impl Solution for AoC2016_05 {
    fn part_one(&self) -> String {
        let mut password = String::new();
        for i in 0..usize::MAX {
            let input = format!("{}{i}", self.door_id);
            let hash = format!("{:x}", md5::compute(input));
            if hash.starts_with("00000") {
                password.push_str(&hash[5..6]);
                if password.len() == 8 {
                    return password;
                }
            }
        }
        "Not found".to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2016/Day 5: How About a Nice Game of Chess?".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_05_correctness() -> io::Result<()> {
        let sol = AoC2016_05::new()?;
        assert_eq!(sol.part_one(), "4543c154");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
