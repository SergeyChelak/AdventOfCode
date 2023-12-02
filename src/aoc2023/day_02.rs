use crate::solution::Solution;
use crate::utils::*;

use std::io;

const MAX_RED_CUBES: usize = 12;
const MAX_GREEN_CUBES: usize = 13;
const MAX_BLUE_CUBES: usize = 14;

struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl CubeSet {
    fn is_possible(&self) -> bool {
        self.red <= MAX_RED_CUBES && self.green <= MAX_GREEN_CUBES && self.blue <= MAX_BLUE_CUBES
    }
}

impl From<&str> for CubeSet {
    fn from(value: &str) -> Self {
        let mut output = Self {
            red: 0,
            green: 0,
            blue: 0,
        };
        value.split(',').map(|s| s.trim()).for_each(|line| {
            let (value, color) = line
                .split_once(' ')
                .expect("Can't split value and color name");
            let val = value.parse::<usize>().expect("Failed to parse cube number");
            match color {
                "green" => output.green = val,
                "blue" => output.blue = val,
                "red" => output.red = val,
                _ => panic!("Unexpected color name {color}"),
            }
        });
        output
    }
}
pub struct AoC2023_02 {
    input: Vec<Vec<CubeSet>>,
}

impl AoC2023_02 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_02")?;
        let input = Self::parse(&lines);
        Ok(Self { input })
    }

    fn parse(lines: &[String]) -> Vec<Vec<CubeSet>> {
        lines
            .iter()
            .map(|line| Self::parse_line(line))
            .collect::<Vec<_>>()
    }

    fn parse_line(line: &str) -> Vec<CubeSet> {
        let (_, sets) = line
            .split_once(": ")
            .expect("Wrong separator after game id");
        sets.split(';').map(|s| CubeSet::from(s)).collect()
    }
}

impl Solution for AoC2023_02 {
    fn part_one(&self) -> String {
        let mut sum = 0usize;
        for (i, game_set) in self.input.iter().enumerate() {
            let is_possible = game_set.iter().fold(true, |acc, x| acc && x.is_possible());
            if is_possible {
                sum += i + 1;
            }
        }
        sum.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 2: Cube Conundrum".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_02_input_load_test() -> io::Result<()> {
        let sol = AoC2023_02::new()?;
        assert_eq!(100, sol.input.len());
        Ok(())
    }

    #[test]
    fn aoc2023_02_correctness() -> io::Result<()> {
        let sol = AoC2023_02::new()?;
        assert_eq!(sol.part_one(), "2913");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
