use crate::solution::Solution;
use crate::utils::*;

use std::fmt::Display;
use std::io;

type Number = f64;
const TOLERANCE: Number = 1e-10;

#[derive(Debug, Copy, Clone)]
struct Vector3d {
    x: Number,
    y: Number,
    z: Number,
}

impl From<&str> for Vector3d {
    fn from(value: &str) -> Self {
        let values = value
            .split(", ")
            .map(|s| {
                s.trim()
                    .parse::<Number>()
                    .expect("Numeric value is expected")
            })
            .collect::<Vec<_>>();
        assert!(value.len() > 2);
        Self {
            x: values[0],
            y: values[1],
            z: values[2],
        }
    }
}

impl Display for Vector3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.y, self.z)
    }
}

struct Hailstone {
    position: Vector3d,
    velocity: Vector3d,
}

impl From<&str> for Hailstone {
    fn from(value: &str) -> Self {
        let (position, velocity) = value
            .split_once(" @ ")
            .expect("Failed to parse hailstone data");
        Self {
            position: Vector3d::from(position),
            velocity: Vector3d::from(velocity),
        }
    }
}

impl Display for Hailstone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}", self.position, self.velocity)
    }
}

impl Hailstone {
    fn position(&self, time: Number) -> Vector3d {
        let x = self.position.x + time * self.velocity.x;
        let y = self.position.y + time * self.velocity.y;
        let z = self.position.z + time * self.velocity.z;
        Vector3d { x, y, z }
    }
}

pub struct AoC2023_24 {
    input: Vec<Hailstone>,
}

impl AoC2023_24 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_24")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let input = lines
            .iter()
            .map(|s| Hailstone::from(s.as_str()))
            .collect::<Vec<_>>();
        Self { input }
    }

    fn path_cross_count(&self, min: Number, max: Number) -> usize {
        let mut count = 0;
        let len = self.input.len();
        for (c, item1) in self.input.iter().take(len - 1).enumerate() {
            for item2 in self.input.iter().skip(c + 1) {
                let Some((t1, t2)) = intersection(item1, item2) else {
                    continue;
                };
                if t1 < 0.0 || t2 < 0.0 {
                    continue;
                }
                let Vector3d { x, y, .. } = item2.position(t2);
                if x > min && x < max && y > min && y < max {
                    count += 1;
                }
            }
        }
        count
    }
}

impl Solution for AoC2023_24 {
    fn part_one(&self) -> String {
        self.path_cross_count(200000000000000.0, 400000000000000.0)
            .to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 24: Never Tell Me The Odds".to_string()
    }
}

fn intersection(a: &Hailstone, b: &Hailstone) -> Option<(Number, Number)> {
    let a_x = a.position.x; // A
    let a_dx = a.velocity.x; // alpha
    if a_dx.abs() < TOLERANCE {
        return None;
    }

    let a_y = a.position.y; // C
    let a_dy = a.velocity.y; // gamma

    let b_x = b.position.x; // B
    let b_dx = b.velocity.x; // beta

    let b_y = b.position.y; // D
    let b_dy = b.velocity.y; // delta

    // linear equation
    // a_x + t1 * a_dx = b_x + t2 * b_dx
    // a_y + t1 * a_dy = b_y + t2 * b_dy

    // solution
    // t2 = (alpha * D - alpha * C - gamma * B + gamma * A) / (gamma * beta - alpha * delta)
    // t1 = (B - A + beta * t2) / alpha

    let denom: Number = a_dy * b_dx - a_dx * b_dy;
    if denom.abs() < TOLERANCE {
        return None;
    }

    let t2 = (a_dx * b_y - a_dx * a_y - a_dy * b_x + a_dy * a_x) / denom;

    let t1 = (b_x - a_x + b_dx * t2) / a_dx;

    Some((t1, t2))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_24_input_load_test() -> io::Result<()> {
        let sol = AoC2023_24::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_24_ex1() {
        let input = [
            "19, 13, 30 @ -2,  1, -2",
            "18, 19, 22 @ -1, -1, -2",
            "20, 25, 34 @ -2, -2, -4",
            "12, 31, 28 @ -1, -2, -1",
            "20, 19, 15 @  1, -5, -3",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let puzzle = AoC2023_24::with_lines(&input);
        assert_eq!(puzzle.path_cross_count(7.0, 27.0), 2);
    }

    #[test]
    fn aoc2023_24_correctness() -> io::Result<()> {
        let sol = AoC2023_24::new()?;
        assert_eq!(sol.part_one(), "25433");
        assert_eq!(sol.part_two(), "885093461440405");
        Ok(())
    }
}
