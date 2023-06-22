use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Port = i32;

struct Component {
    port_a: Port,
    port_b: Port,
}

impl Component {
    fn from_str(s: &str) -> Self {
        let (port_a, port_b) = s
            .split_once('/')
            .expect("Pin's values should be separated with slash");
        let port_a = port_a
            .parse::<Port>()
            .expect("Pins count for first port should be integer");
        let port_b = port_b
            .parse::<Port>()
            .expect("Pins count for second port should be integer");
        Self { port_a, port_b }
    }

    fn can_join(&self, pins: Port) -> bool {
        self.port_a == pins || self.port_b == pins
    }
}

pub struct AoC2017_24 {
    components: Vec<Component>,
}

impl AoC2017_24 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2017_24")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let components = lines.iter().map(|s| Component::from_str(s)).collect();
        Self { components }
    }
}

impl Solution for AoC2017_24 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 24: Electromagnetic Moat".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_24_input_load_test() -> io::Result<()> {
        let sol = AoC2017_24::new()?;
        assert!(!sol.components.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_24_correctness() -> io::Result<()> {
        let sol = AoC2017_24::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
