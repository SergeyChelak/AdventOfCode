use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

struct Maneuver {
    left: String,
    right: String,
}

impl From<&str> for Maneuver {
    fn from(value: &str) -> Self {
        let (left, right) = remove_first_and_last(value)
            .split_once(", ")
            .expect("Invalid format (2)");
        Self {
            left: left.to_string(),
            right: right.to_string(),
        }
    }
}

pub struct AoC2023_08 {
    map: HashMap<String, Maneuver>,
    route: String,
}

impl AoC2023_08 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_08")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        assert!(lines[1].is_empty());
        let map: HashMap<String, Maneuver> = lines
            .iter()
            .skip(2)
            .map(|s| {
                let (pos, maneuver) = s.split_once(" = ").expect("Invalid format (1)");
                (pos.to_string(), Maneuver::from(maneuver))
            })
            .collect();
        Self {
            route: lines[0].to_string(),
            map,
        }
    }
}

impl Solution for AoC2023_08 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_08_input_load_test() -> io::Result<()> {
        let sol = AoC2023_08::new()?;
        assert!(!sol.route.is_empty());
        assert!(!sol.map.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_08_correctness() -> io::Result<()> {
        let sol = AoC2023_08::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
