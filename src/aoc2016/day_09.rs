use crate::solution::Solution;
use crate::utils::*;

use regex::Regex;
use std::io;

pub struct AoC2016_09 {
    lines: Vec<String>,
}

impl AoC2016_09 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            lines: read_file_as_lines("input/aoc2016_09")?,
        })
    }

    fn decoded_len(&self, decoder: &dyn Fn(&str) -> String) -> usize {
        self.lines
            .iter()
            .map(|s| decoder(s))
            .map(|s| s.len())
            .sum::<usize>()
    }
}

impl Solution for AoC2016_09 {
    fn part_one(&self) -> String {
        self.decoded_len(&decompress_v1).to_string()
    }

    fn part_two(&self) -> String {
        self.decoded_len(&decompress_v2).to_string()
    }

    fn description(&self) -> String {
        "AoC 2016/Day 9: Explosives in Cyberspace".to_string()
    }
}

fn decompress_v2(s: &str) -> String {
    let mut inp = String::from(s);
    loop {
        let (output, is_final) = decompress(&inp);
        if is_final {
            break output;
        }
        inp = output;
    }
}

fn decompress_v1(s: &str) -> String {
    decompress(s).0
}

fn decompress(s: &str) -> (String, bool) {
    let regex = Regex::new(r"[(]\d*[x]\d*[)]").expect("regex format should be valid");
    let mut result = String::new();
    let mut rest = s;
    let mut is_final = true;
    while let Some(reg_match) = regex.find(rest) {
        is_final = false;
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
    (result, is_final)
}

fn parse_marker(marker: &str) -> (usize, usize) {
    let (count, reps) = &marker[1..marker.len() - 1]
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
        assert!(!sol.lines.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2016_09_correctness() -> io::Result<()> {
        let sol = AoC2016_09::new()?;
        assert_eq!(sol.part_one(), "110346");
        assert_eq!(sol.part_two(), "10774309173");
        Ok(())
    }

    #[test]
    fn aoc2016_09_decompress_v1() {
        assert_eq!(decompress_v1("ADVENT"), "ADVENT");
        assert_eq!(decompress_v1("A(1x5)BC"), "ABBBBBC");
        assert_eq!(decompress_v1("(3x3)XYZ"), "XYZXYZXYZ");
        assert_eq!(decompress_v1("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
        assert_eq!(decompress_v1("(6x1)(1x3)A"), "(1x3)A");
        assert_eq!(decompress_v1("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
    }

    #[test]
    fn aoc2016_09_decompress_v2() {
        assert_eq!(decompress_v2("(3x3)XYZ"), "XYZXYZXYZ");
        assert_eq!(decompress_v2("X(8x2)(3x3)ABCY"), "XABCABCABCABCABCABCY");
        assert_eq!(
            decompress_v2("(27x12)(20x12)(13x14)(7x10)(1x12)A").len(),
            241920
        );
        assert_eq!(
            decompress_v2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN").len(),
            445
        );
    }
}
