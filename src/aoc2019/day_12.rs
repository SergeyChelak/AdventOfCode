use regex::Regex;

use crate::solution::Solution;
use crate::utils::*;

use std::io;

#[derive(Debug, Clone, Copy)]
struct Point3D<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Point3D<T> {
    fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

type Int = i64;
type Point = Point3D<Int>;

impl Point {
    fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    fn add(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    fn negative(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    fn gravity_vector(&self, other: &Self) -> Self {
        let gravity = |a: Int, b: Int| -> Int {
            if a > b {
                return -1;
            }
            if a < b {
                return 1;
            }
            0
        };
        Self {
            x: gravity(self.x, other.x),
            y: gravity(self.y, other.y),
            z: gravity(self.z, other.z),
        }
    }

    fn energy(&self) -> Int {
        [self.x, self.y, self.z].iter().map(|x| x.abs()).sum()
    }

    fn dump(&self) -> String {
        format!("<x={:3} | y={:3} | z={:3}>", self.x, self.y, self.z)
    }
}

impl TryFrom<&str> for Point {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let Some(captures) = Regex::new(r"<x=(-?\d*), y=(-?\d*), z=(-?\d*)>")
            .map_err(|e| format!("Regex parse error: {e:?}"))?
            .captures(value)
        else {
            return Err("Empty input".to_string());
        };
        let coordinates = captures
            .iter()
            .skip(1)
            .filter_map(|x| x.map(|x| x.as_str()))
            .map(|x| x.parse::<Int>())
            .take_while(|x| x.is_ok())
            .map(Result::unwrap)
            .collect::<Vec<_>>();
        if coordinates.len() != 3 {
            return Err(format!(
                "Failed to parse input: only {} coordinated passed",
                coordinates.len()
            ));
        }
        Ok(Self {
            x: coordinates[0],
            y: coordinates[1],
            z: coordinates[2],
        })
    }
}

pub struct AoC2019_12 {
    moons: Vec<Point>,
}

impl AoC2019_12 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2019_12")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let moons = lines
            .iter()
            .map(|x| x.as_ref())
            .map(Point::try_from)
            .map(Result::unwrap)
            .collect::<Vec<_>>();
        Self { moons }
    }

    fn simulate(&self, steps: usize) -> Int {
        let count = self.moons.len();
        let mut velocity = vec![Point::zero(); count];
        let mut position = self.moons.clone();
        let mut gravity = vec![Point::zero(); count];
        for _ in 0..steps {
            gravity.iter_mut().for_each(|x| {
                *x = Point::zero();
            });

            for (i, i_pos) in position.iter().enumerate().take(count - 1) {
                for (j, j_pos) in position.iter().enumerate().skip(1 + i) {
                    let g = i_pos.gravity_vector(j_pos);
                    gravity[i].add(&g);
                    gravity[j].add(&g.negative());
                }
            }
            velocity
                .iter_mut()
                .zip(gravity.iter())
                .for_each(|(v, a)| v.add(a));
            position
                .iter_mut()
                .zip(velocity.iter())
                .for_each(|(p, v)| p.add(v));
        }
        position
            .iter()
            .zip(velocity.iter())
            .map(|(a, b)| a.energy() * b.energy())
            .sum::<Int>()
    }
}

impl Solution for AoC2019_12 {
    fn part_one(&self) -> String {
        self.simulate(1000).to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 12: The N-Body Problem".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_12_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.moons.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_12_parse() {
        let inp = "<x=-15, y=-14, z=12>";
        let point = Point::try_from(inp).expect("Failed to parse");
        assert_eq!(point.x, -15);
        assert_eq!(point.y, -14);
        assert_eq!(point.z, 12);
    }

    #[test]
    fn aoc2019_12_simulate() {
        let inp = [
            "<x=-1, y=0, z=2>",
            "<x=2, y=-10, z=-7>",
            "<x=4, y=-8, z=8>",
            "<x=3, y=5, z=-1>",
        ];
        let p = AoC2019_12::with_lines(&inp);
        assert_eq!(p.simulate(10), 179);
    }

    #[test]
    fn aoc2019_12_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "10635");
        Ok(())
    }

    #[test]
    fn aoc2019_12_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_12> {
        AoC2019_12::new()
    }
}
