use crate::solution::Solution;
use crate::utils::*;

use std::io;

enum JoinPort {
    A,
    B,
}

#[derive(Default, Copy, Clone)]
struct BridgeParams {
    strength: i32,
    length: usize,
}

struct Component {
    port_a: i32,
    port_b: i32,
}

impl Component {
    fn from_str(s: &str) -> Self {
        let (port_a, port_b) = s
            .split_once('/')
            .expect("Pin's values should be separated with slash");
        let port_a = port_a
            .parse::<i32>()
            .expect("Pins count for first port should be integer");
        let port_b = port_b
            .parse::<i32>()
            .expect("Pins count for second port should be integer");
        Self { port_a, port_b }
    }

    fn join_port(&self, pins: i32) -> Option<JoinPort> {
        if self.port_a == pins {
            Some(JoinPort::A)
        } else if self.port_b == pins {
            Some(JoinPort::B)
        } else {
            None
        }
    }

    fn get_opposite_value(&self, port: JoinPort) -> i32 {
        match port {
            JoinPort::A => self.port_b,
            JoinPort::B => self.port_a,
        }
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

    fn find_max_strength(
        &self,
        indices: &mut Vec<usize>,
        params: &mut BridgeParams,
        pin_count: i32,
        criteria: &dyn Fn(&BridgeParams, &[usize]) -> BridgeParams,
    ) {
        for (i, comp) in self.components.iter().enumerate() {
            if indices.contains(&i) {
                continue;
            }
            if let Some(port) = comp.join_port(pin_count) {
                indices.push(i);
                let pins = comp.get_opposite_value(port);
                self.find_max_strength(indices, params, pins, criteria);
                indices.pop();
            }
        }
        *params = criteria(params, indices);
    }

    fn calc_strength(&self, indices: &[usize]) -> i32 {
        indices
            .iter()
            .map(|x| &self.components[*x])
            .map(|x| x.port_a + x.port_b)
            .sum::<i32>()
    }
}

impl Solution for AoC2017_24 {
    fn part_one(&self) -> String {
        let mut params = BridgeParams::default();
        self.find_max_strength(&mut Vec::new(), &mut params, 0, &|p, indices| {
            let strength = self.calc_strength(indices);
            BridgeParams {
                strength: strength.max(p.strength),
                ..*p
            }
        });
        params.strength.to_string()
    }

    fn part_two(&self) -> String {
        let mut params = BridgeParams::default();
        self.find_max_strength(&mut Vec::new(), &mut params, 0, &|p, indices| {
            let length = indices.len();
            let strength = self.calc_strength(indices);
            if length >= p.length && strength > p.strength {
                BridgeParams { strength, length }
            } else {
                *p
            }
        });
        params.strength.to_string()
    }

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
        assert_eq!(sol.part_one(), "1940");
        assert_eq!(sol.part_two(), "1928");
        Ok(())
    }
}
