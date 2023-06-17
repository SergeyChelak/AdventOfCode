use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Value = i32;

#[derive(Clone, Copy)]
struct Value3d {
    x: Value,
    y: Value,
    z: Value,
}

impl Value3d {
    fn from_str(s: &str) -> Self {
        let tokens = s.split_once('=')
            .expect("Incorrect Value3d declaration");
        let tokens = remove_first_and_last(tokens.1).split(',').collect::<Vec<&str>>();
        let x = tokens[0].parse::<Value>().expect("Failed parse X");
        let y = tokens[1].parse::<Value>().expect("Failed parse Y");
        let z = tokens[2].parse::<Value>().expect("Failed parse Z");
        Value3d { x, y, z }
    }

    fn plus(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

#[derive(Clone, Copy)]
struct Particle {
    position: Value3d,
    speed: Value3d,
    acceleration: Value3d,
}

impl Particle {
    fn from_str(s: &str) -> Self {
        let parts = s.split(", ").collect::<Vec<&str>>();
        let position = Value3d::from_str(parts[0]);
        let speed = Value3d::from_str(parts[1]);
        let acceleration = Value3d::from_str(parts[2]);
        Self {
            position,
            speed,
            acceleration,
        }
    }

    fn distance(&self) -> Value {
        let p = &self.position;
        p.x.abs() + p.y.abs() + p.z.abs()
    }

    fn teak(&mut self) {
        self.speed.plus(&self.acceleration);
        self.position.plus(&self.speed);
    }
}

pub struct AoC2017_20 {
    particles: Vec<Particle>,
}

impl AoC2017_20 {
    pub fn new() -> io::Result<Self> {
        let particles = read_file_as_lines("input/aoc2017_20")?
            .iter()
            .map(|s| Particle::from_str(s))
            .collect();
        Ok(Self { particles })
    }
}

impl Solution for AoC2017_20 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/cDay 20: Particle Swarm".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_20_input_load_test() -> io::Result<()> {
        let sol = AoC2017_20::new()?;
        assert!(!sol.particles.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_20_correctness() -> io::Result<()> {
        let sol = AoC2017_20::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
