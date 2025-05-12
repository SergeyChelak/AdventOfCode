use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Number = f64;
const TOLERANCE: Number = 1e-10;

#[derive(Debug, Copy, Clone)]
struct Vector3d {
    x: Number,
    y: Number,
    z: Number,
}

impl Vector3d {
    fn sum(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn diff(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn scalar_mul(&self, scalar: Number) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    fn scalar_div(&self, scalar: Number) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }

    fn dot_product(&self, other: &Self) -> Number {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross_product(&self, other: &Self) -> Vector3d {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
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

impl Hailstone {
    fn position(&self, time: Number) -> Vector3d {
        self.position.sum(&self.velocity.scalar_mul(time))
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

    fn part_two(&self) -> String {
        // https://www.reddit.com/r/adventofcode/comments/18pnycy/2023_day_24_solutions/
        assert!(self.input.len() > 2);
        let relative = |a: usize, b: usize| -> (Vector3d, Vector3d) {
            let p = self.input[a].position.diff(&self.input[b].position);
            let v = self.input[a].velocity.diff(&self.input[b].velocity);
            (p, v)
        };
        let (p1, v1) = relative(1, 0);
        let (p2, v2) = relative(2, 0);
        // t1 = -((p1 x p2) * v2) / ((v1 x p2) * v2)
        let t1 = -p1.cross_product(&p2).dot_product(&v2) / (v1.cross_product(&p2).dot_product(&v2));
        // t2 = -((p1 x p2) * v1) / ((p1 x v2) * v1)
        let t2 = -p1.cross_product(&p2).dot_product(&v1) / (p1.cross_product(&v2).dot_product(&v1));
        // c1 = position_1 + t1 * velocity_1
        let c1 = self.input[1].position(t1);
        // c2 = position_2 + t2 * velocity_2
        let c2 = self.input[2].position(t2);
        // v = (c2 - c1) / (t2 - t1)
        let v = c2.diff(&c1).scalar_div(t2 - t1);
        // p = c1 - t1 * v
        let p = c1.diff(&v.scalar_mul(t1));

        (p.x + p.y + p.z).to_string()
    }

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
