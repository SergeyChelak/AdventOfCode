use crate::solution::Solution;
use crate::utils::*;

use std::io;

struct Replacement {
    inp: String,
    out: String
}

impl Replacement {
    fn from_str(s: &str) -> Self {
        let pair = s.split_once(" => ")
            .expect("Invalid replacement string format");
        Self {
            inp: pair.0.to_string(),
            out: pair.1.to_string(),
        }
    }
}

pub struct AoC2015_19 {
    molecule: String,
    replacement: Vec<Replacement>,
}

impl AoC2015_19 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2015_19")?;
        let index = lines.iter().position(|elem| elem.is_empty()).expect("Empty line is expected");
        let replacement = lines[..index].iter()
            .map(|elem| Replacement::from_str(elem))
            .collect::<Vec<Replacement>>();
        let molecule = lines[lines.len() - 1].to_string();
        Ok(Self {
            molecule,
            replacement,
        })
    }
}

impl Solution for AoC2015_19 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2015/Day 19: Medicine for Rudolph".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_19_input_load_test() -> io::Result<()> {
        let sol = AoC2015_19::new()?;
        assert!(sol.molecule.len() > 0);
        assert_eq!(sol.replacement.len(), 43);
        Ok(())
    }

    #[test]
    fn aoc2015_19_correctness() -> io::Result<()> {
        let sol = AoC2015_19::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}