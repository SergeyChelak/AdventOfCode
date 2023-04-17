use crate::solution::Solution;
use crate::utils::*;

use std::io;
use regex::Regex;

pub struct AoC2016_09 {
    lines: Vec<String>
}

impl AoC2016_09 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            lines: read_file_as_lines("input/aoc2016_09")?
        })
    }
}

impl Solution for AoC2016_09 {
    fn part_one(&self) -> String {
        self.lines
            .iter()
            .map(|s| decompress(s))
            .map(|s| s.len())
            .sum::<usize>()
            .to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2016/Day 9: Explosives in Cyberspace".to_string()
    }
}

fn decompress(s: &str) -> String {
    let regex = Regex::new(r"[(]\d*[x]\d*[)]")
        .expect("regex format should be valid");
    let mut result = String::new();
    let mut rest = s;
    while let Some(reg_match) = regex.find(rest) {
        let prefix = &rest[0..reg_match.start()];
        result.push_str(prefix);
        let marker = &rest[reg_match.start()..reg_match.end()];
        let (count, reps) = parse_marker(marker);        
        let rep_str = &rest[reg_match.end()..reg_match.end() + count];
        for _ in 0..reps {
            result.push_str(rep_str);
        }
        rest = &rest[reg_match.end() + count..];
    }
    result.push_str(rest);
    result
}

fn parse_marker(marker: &str) -> (usize, usize) {
    let (count, reps) = &marker[1..marker.len()-1]
        .split_once('x')
        .expect("delimited 'x' should be present in marker");
    let count = count.parse::<usize>().expect("count should be integer");
    let reps = reps.parse::<usize>().expect("reps should be integer");
    (count, reps)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_09_input_load_test() -> io::Result<()> {
        let sol = AoC2016_09::new()?;
        assert!(sol.lines.len() > 0);
        Ok(())
    }

    #[test]
    fn aoc2016_09_correctness() -> io::Result<()> {
        let sol = AoC2016_09::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2016_09_decompress() {
        assert_eq!(decompress("ADVENT"), "ADVENT");
        assert_eq!(decompress("A(1x5)BC"), "ABBBBBC");
        assert_eq!(decompress("(3x3)XYZ"), "XYZXYZXYZ");
        assert_eq!(decompress("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
        assert_eq!(decompress("(6x1)(1x3)A"), "(1x3)A");
        assert_eq!(decompress("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
    }
}