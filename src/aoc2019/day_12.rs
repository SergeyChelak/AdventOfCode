use regex::Regex;

use crate::solution::Solution;
use crate::utils::*;

use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

struct Simulation {
    position: Vec<Point>,
    velocity: Vec<Point>,
    gravity: Vec<Point>,
}

impl Simulation {
    fn new(initial: &[Point]) -> Self {
        let count = initial.len();
        let position = initial.to_owned();
        let velocity = vec![Point::zero(); count];
        let gravity = vec![Point::zero(); count];
        Self {
            position,
            velocity,
            gravity,
        }
    }

    fn step_simulate(&mut self) {
        let count = self.position.len();
        self.gravity.iter_mut().for_each(|x| {
            *x = Point::zero();
        });
        for (i, i_pos) in self.position.iter().enumerate().take(count - 1) {
            for (j, j_pos) in self.position.iter().enumerate().skip(1 + i) {
                let g = i_pos.gravity_vector(j_pos);
                self.gravity[i].add(&g);
                self.gravity[j].add(&g.negative());
            }
        }
        self.velocity
            .iter_mut()
            .zip(self.gravity.iter())
            .for_each(|(v, a)| v.add(a));
        self.position
            .iter_mut()
            .zip(self.velocity.iter())
            .for_each(|(p, v)| p.add(v));
    }

    fn simulate(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step_simulate();
        }
    }

    fn total_energy(&self) -> Int {
        self.position
            .iter()
            .zip(self.velocity.iter())
            .map(|(a, b)| a.energy() * b.energy())
            .sum::<Int>()
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
}

impl Solution for AoC2019_12 {
    fn part_one(&self) -> String {
        let mut simulation = Simulation::new(&self.moons);
        simulation.simulate(1000);
        simulation.total_energy().to_string()
    }

    fn part_two(&self) -> String {
        let count = self.moons.len();

        let mut simulation = Simulation::new(&self.moons);
        let mut step = 0usize;

        let mut axes = [0usize; 3];

        while axes.contains(&0) {
            simulation.step_simulate();
            step += 1;
            let mut x_count = 0;
            let mut y_count = 0;
            let mut z_count = 0;
            for (i, (p, v)) in simulation
                .position
                .iter()
                .zip(simulation.velocity.iter())
                .enumerate()
            {
                if p.x == self.moons[i].x && v.x == 0 {
                    x_count += 1;
                }
                if p.y == self.moons[i].y && v.y == 0 {
                    y_count += 1;
                }
                if p.z == self.moons[i].z && v.z == 0 {
                    z_count += 1;
                }
            }

            if x_count == count && axes[0] == 0 {
                axes[0] = step;
            }

            if y_count == count && axes[1] == 0 {
                axes[1] = step;
            }

            if z_count == count && axes[2] == 0 {
                axes[2] = step;
            }
        }
        axes.iter().fold(1, |acc, x| lcm(acc, *x)).to_string()
    }

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
        let mut simulation = Simulation::new(&p.moons);
        simulation.simulate(10);
        assert_eq!(simulation.total_energy(), 179);
    }

    #[test]
    fn aoc2019_12_case_1() {
        let inp = [
            "<x=-1, y=0, z=2>",
            "<x=2, y=-10, z=-7>",
            "<x=4, y=-8, z=8>",
            "<x=3, y=5, z=-1>",
        ];
        let p = AoC2019_12::with_lines(&inp);
        assert_eq!(p.part_two(), "2772")
    }

    #[test]
    fn aoc2019_12_case_2() {
        let inp = [
            "<x=-8, y=-10, z=0>",
            "<x=5, y=5, z=10>",
            "<x=2, y=-7, z=3>",
            "<x=9, y=-8, z=-3>",
        ];
        let p = AoC2019_12::with_lines(&inp);
        assert_eq!(p.part_two(), "4686774924")
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
        assert_eq!(sol.part_two(), "583523031727256");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_12> {
        AoC2019_12::new()
    }
}
