use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::fmt::Display;
use std::io;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> Point3<T>
where
    T: Copy,
{
    pub fn modify_component(&self, operation: impl Fn(T, T) -> T, other: &Self) -> Self {
        Self {
            x: operation(self.x, other.x),
            y: operation(self.y, other.y),
            z: operation(self.z, other.z),
        }
    }
}

impl<T> Point3<T>
where
    T: Copy + Add<Output = T>,
{
    pub fn add(&self, other: &Self) -> Self {
        self.modify_component(|a, b| a + b, other)
    }
}

impl<T> Point3<T>
where
    T: Copy + Sub<Output = T>,
{
    pub fn subtract(&self, other: &Self) -> Self {
        self.modify_component(|a, b| a - b, other)
    }
}

impl<T: Display> Display for Point3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{x: {}, y: {}, z: {}}}", self.x, self.y, self.z)
    }
}

type Int = isize;
type Point = Point3<Int>;

impl Point {
    fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    fn adjacent(&self) -> Vec<Point> {
        let delta = [-1, 0, 1];
        let mut result = Vec::with_capacity(26);
        for dx in delta.iter() {
            for dy in delta.iter() {
                for dz in delta.iter() {
                    if *dx == 0 && *dy == 0 && *dz == 0 {
                        continue;
                    }
                    let delta = Point3::new(*dx, *dy, *dz);
                    result.push(self.add(&delta));
                }
            }
        }
        assert_eq!(26, result.len());
        assert!(!result.contains(&self));
        result
    }
}

pub struct AoC2020_17 {
    input: HashSet<Point>,
    from: Point,
    to: Point,
}

impl AoC2020_17 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2020_17")?;
        // let input = std::fs::read_to_string("input/aoc2020_17")?;
        Ok(Self::parse(&lines))
    }

    fn parse<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut to = Point::zero();
        let mut input = HashSet::new();
        for (row, s) in lines.iter().map(|x| x.as_ref()).enumerate() {
            for (col, ch) in s.chars().enumerate() {
                to = Point::new(col as isize, row as isize, 0);
                if ch == '#' {
                    input.insert(to);
                }
            }
        }
        Self {
            input,
            from: Point3::zero(),
            to,
        }
    }
}

impl Solution for AoC2020_17 {
    fn part_one(&self) -> String {
        let mut from = self.from;
        let mut to = self.to;

        let mut store = self.input.clone();

        let one = Point::new(1, 1, 1);
        for _ in 0..6 {
            let mut new_store = HashSet::new();
            from = from.subtract(&one);
            to = to.add(&one);
            for x in from.x..=to.x {
                for y in from.y..=to.y {
                    for z in from.z..=to.z {
                        let p = Point::new(x, y, z);
                        let is_active = store.contains(&p);
                        let adj_count = p.adjacent().iter().filter(|x| store.contains(*x)).count();
                        match (is_active, adj_count) {
                            (true, 2) | (true, 3) => {
                                new_store.insert(p);
                            }
                            (false, 3) => {
                                new_store.insert(p);
                            }
                            _ => {
                                // no op
                            }
                        }
                    }
                }
            }
            store = new_store;
        }

        store.len().to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 17: Conway Cubes".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_17_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        assert_ne!(sol.to, Point::zero());
        Ok(())
    }

    #[test]
    fn aoc2020_17_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "384");
        Ok(())
    }

    #[test]
    fn aoc2020_17_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_17> {
        AoC2020_17::new()
    }

    #[test]
    fn aoc2020_17_case1() {
        let input = [".#.", "..#", "###"];
        let sol = AoC2020_17::parse(&input);
        assert_eq!(sol.part_one(), "112");
    }
}
