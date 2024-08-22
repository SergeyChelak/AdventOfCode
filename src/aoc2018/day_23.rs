use regex::Regex;

use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = isize;
struct Nanobot {
    x: Int,
    y: Int,
    z: Int,
    radius: Int,
}

struct Parser {
    regex: Regex,
}

impl Parser {
    fn new() -> Option<Self> {
        let Ok(regex) = Regex::new(r"\d+") else {
            return None;
        };
        Some(Self { regex })
    }

    fn parse(&self, value: &str) -> Nanobot {
        let values = self
            .regex
            .find_iter(value)
            .filter_map(|x| x.as_str().parse::<Int>().ok())
            .collect::<Vec<Int>>();
        if values.len() != 4 {
            panic!("Invalid input: {value}");
        }
        Nanobot {
            x: values[0],
            y: values[1],
            z: values[2],
            radius: values[3],
        }
    }
}

pub struct AoC2018_23 {
    bots: Vec<Nanobot>,
}

impl AoC2018_23 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2018_23")?;
        let parser = Parser::new().expect("Failed to create parser");
        let bots = lines.iter().map(|s| parser.parse(s.as_str())).collect();
        Ok(Self { bots })
    }
}

impl Solution for AoC2018_23 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 23: Experimental Emergency Teleportation".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_23_input_load_test() -> io::Result<()> {
        let sol = AoC2018_23::new()?;
        assert_eq!(sol.bots.len(), 1000);
        Ok(())
    }

    #[test]
    fn aoc2018_23_parser_nanobot() -> Result<(), String> {
        let inp = "pos=<123,234,345>, r=456";
        let parser = Parser::new().unwrap();
        let nanobot = parser.parse(inp);
        assert_eq!(nanobot.x, 123);
        assert_eq!(nanobot.y, 234);
        assert_eq!(nanobot.z, 345);
        assert_eq!(nanobot.radius, 456);
        Ok(())
    }

    #[test]
    fn aoc2018_23_correctness() -> io::Result<()> {
        let sol = AoC2018_23::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
