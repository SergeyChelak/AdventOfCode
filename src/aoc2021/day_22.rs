use crate::solution::Solution;
use crate::utils::*;

use std::io;

#[derive(Debug, Clone)]
struct Interval3<T> {
    x: PlainInterval<T>,
    y: PlainInterval<T>,
    z: PlainInterval<T>,
}

type Int = isize;
type Cube = Interval3<Int>;

impl From<&str> for Cube {
    fn from(value: &str) -> Self {
        let arr = value
            .split(',')
            .map(|s| s.split_once('=').expect("Invalid interval format").1)
            .map(|s| PlainInterval::parse(s, "..").expect("Invalid interval"))
            .inspect(|i| assert!(i.begin <= i.end))
            .collect::<Vec<_>>();
        assert_eq!(3, arr.len());
        Self {
            x: arr[0],
            y: arr[1],
            z: arr[2],
        }
    }
}

impl Cube {
    fn is_small(&self) -> bool {
        [&self.x, &self.y, &self.z]
            .iter()
            .all(|i| i.begin >= -50 && i.end <= 50)
    }

    fn intersection(&self, other: &Self) -> Option<Cube> {
        let x = self.x.intersection(&other.x)?;
        let y = self.y.intersection(&other.y)?;
        let z = self.z.intersection(&other.z)?;
        let cube = Self { x, y, z };
        Some(cube)
    }

    fn square(&self) -> Int {
        [&self.x, &self.y, &self.z]
            .iter()
            .map(|int| int.length())
            .product()
    }
}

impl PlainInterval<Int> {
    fn length(&self) -> isize {
        self.end - self.begin + 1
    }
}

#[derive(Debug, Clone)]
struct Cuboid {
    cube: Cube,
    weight: Int,
}

impl From<&str> for Cuboid {
    fn from(value: &str) -> Self {
        let (is_on, interval) = value.split_once(' ').expect("Invalid entry format");
        let cube = Interval3::from(interval);
        let weight = if is_on == "on" { 1 } else { -1 };
        Self { weight, cube }
    }
}

pub struct AoC2021_22 {
    input: Vec<Cuboid>,
}

impl AoC2021_22 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2021_22")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(Cuboid::from)
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2021_22 {
    fn part_one(&self) -> String {
        calculate(
            self.input
                .iter()
                .filter(|val| val.cube.is_small())
                .cloned()
                .collect(),
        )
        .to_string()
    }

    fn part_two(&self) -> String {
        calculate(self.input.clone()).to_string()
    }

    fn description(&self) -> String {
        "Day 22: Reactor Reboot".to_string()
    }
}

fn calculate(input: Vec<Cuboid>) -> Int {
    let mut elements = input.into_iter().rev().collect::<Vec<_>>();
    let mut aligned = Vec::<Cuboid>::new();
    while let Some(n) = elements.pop() {
        let mut intersections = aligned
            .iter()
            .filter_map(|c| {
                let inter = c.cube.intersection(&n.cube)?;
                let next = Cuboid {
                    cube: inter,
                    weight: -c.weight,
                };
                Some(next)
            })
            .collect::<Vec<_>>();
        if n.weight > 0 {
            aligned.push(n);
        }
        aligned.append(&mut intersections);
    }

    aligned
        .into_iter()
        .map(|val| val.weight * val.cube.square())
        .sum::<Int>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_22_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2021_22_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "642125");
        Ok(())
    }

    #[test]
    fn aoc2021_22_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "1235164413198198");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2021_22> {
        AoC2021_22::new()
    }
}
