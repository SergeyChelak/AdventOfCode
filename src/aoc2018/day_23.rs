use regex::Regex;

use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = isize;
#[derive(Debug, Clone, Copy)]
struct Point3d<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Point3d<T> {
    fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

type Coordinate3d = Point3d<Int>;

impl Coordinate3d {
    fn zero() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    fn distance(&self, other: &Self) -> Int {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    fn length(&self) -> Int {
        self.distance(&Self::zero())
    }
}

#[derive(Debug, Clone, Copy)]
struct Nanobot {
    point: Point3d<Int>,
    radius: Int,
}

impl Nanobot {
    fn dist(&self, other: &Nanobot) -> Int {
        self.point.distance(&other.point)
    }

    fn is_reachable(&self, other: &Nanobot) -> bool {
        self.radius >= self.dist(other)
    }
}

struct Parser {
    regex: Regex,
}

impl Parser {
    fn new() -> Option<Self> {
        let Ok(regex) = Regex::new(r"-?\d+") else {
            return None;
        };
        Some(Self { regex })
    }

    fn parse(&self, value: &str) -> Nanobot {
        let values = self
            .regex
            .find_iter(value)
            .filter_map(|x| x.as_str().parse::<Int>().ok())
            .collect::<Vec<Int>>();
        if values.len() != 4 {
            panic!("Invalid input: {value}");
        }
        let point = Point3d::new(values[0], values[1], values[2]);
        Nanobot {
            point,
            radius: values[3],
        }
    }
}

pub struct AoC2018_23 {
    bots: Vec<Nanobot>,
}

impl AoC2018_23 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2018_23")?;
        let parser = Parser::new().expect("Failed to create parser");
        let bots = lines.iter().map(|s| parser.parse(s.as_str())).collect();
        Ok(Self { bots })
    }

    fn reached_bots(&self, bot: &Nanobot) -> usize {
        self.bots.iter().filter(|x| bot.is_reachable(x)).count()
    }
}

impl Solution for AoC2018_23 {
    fn part_one(&self) -> String {
        let strongest = self
            .bots
            .iter()
            .max_by(|a, b| a.radius.cmp(&b.radius))
            .unwrap();

        self.reached_bots(strongest).to_string()
    }

    fn part_two(&self) -> String {
        find_distance(&self.bots)
            .expect("Input shouldn't be empty")
            .to_string()
    }

    fn description(&self) -> String {
        "AoC 2018/Day 23: Experimental Emergency Teleportation".to_string()
    }
}

fn find_distance(bots: &[Nanobot]) -> Option<isize> {
    let (mut x_min, mut x_max) = range(bots, |b| b.point.x)?;
    let (mut y_min, mut y_max) = range(bots, |b| b.point.y)?;
    let (mut z_min, mut z_max) = range(bots, |b| b.point.z)?;

    let largest_range = x_max
        .abs_diff(x_min)
        .max(y_max.abs_diff(y_min))
        .max(z_max.abs_diff(z_min));
    let mut step = 1;
    while step < largest_range {
        step <<= 1;
    }
    let shift_range = |value: Int, step: usize| -> (Int, Int) {
        let step = step as Int;
        (value - step, value + step)
    };
    let mut result = Coordinate3d::zero();
    while step > 1 {
        let mut best = Coordinate3d::zero();
        let mut max_count = 0;
        for x in (x_min..=x_max).step_by(step) {
            for y in (y_min..=y_max).step_by(step) {
                for z in (z_min..=z_max).step_by(step) {
                    let coordinate = Coordinate3d::new(x, y, z);
                    let count = bots
                        .iter()
                        .filter(|bot| bot.point.distance(&coordinate) <= bot.radius)
                        .count();
                    if count > max_count {
                        best = coordinate;
                        max_count = count;
                    }
                    if count == max_count && coordinate.length() < best.length() {
                        best = coordinate;
                    }
                }
            }
        }
        (x_min, x_max) = shift_range(best.x, step);
        (y_min, y_max) = shift_range(best.y, step);
        (z_min, z_max) = shift_range(best.z, step);
        step >>= 1;
        result = best;
    }
    Some(result.length())
}

fn range(bots: &[Nanobot], predicate: impl Fn(&Nanobot) -> Int) -> Option<(Int, Int)> {
    let min = bots.iter().map(&predicate).min()?;
    let max = bots.iter().map(&predicate).max()?;
    Some((min, max))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_23_input_load_test() -> io::Result<()> {
        let sol = AoC2018_23::new()?;
        assert_eq!(sol.bots.len(), 1000);
        Ok(())
    }

    #[test]
    fn aoc2018_23_ex_1() {
        let puzzle = puzzle_factory(&[
            "pos=<0,0,0>, r=4",
            "pos=<1,0,0>, r=1",
            "pos=<4,0,0>, r=3",
            "pos=<0,2,0>, r=1",
            "pos=<0,5,0>, r=3",
            "pos=<0,0,3>, r=1",
            "pos=<1,1,1>, r=1",
            "pos=<1,1,2>, r=1",
            "pos=<1,3,1>, r=1",
        ]);
        assert_eq!(puzzle.part_one(), "7")
    }

    #[test]
    fn aoc2018_23_ex_2() {
        let puzzle = puzzle_factory(&[
            "pos=<10,12,12>, r=2",
            "pos=<12,14,12>, r=2",
            "pos=<16,12,12>, r=4",
            "pos=<14,14,14>, r=6",
            "pos=<50,50,50>, r=200",
            "pos=<10,10,10>, r=5",
        ]);
        assert_eq!(puzzle.part_two(), "36")
    }

    fn puzzle_factory(input: &[&str]) -> AoC2018_23 {
        let parser = Parser::new().unwrap();
        let bots = input.iter().map(|s| parser.parse(s)).collect();
        AoC2018_23 { bots }
    }

    #[test]
    fn aoc2018_23_parser_nanobot() -> Result<(), String> {
        let inp = "pos=<123,234,345>, r=456";
        let parser = Parser::new().unwrap();
        let nanobot = parser.parse(inp);
        assert_eq!(nanobot.point.x, 123);
        assert_eq!(nanobot.point.y, 234);
        assert_eq!(nanobot.point.z, 345);
        assert_eq!(nanobot.radius, 456);
        Ok(())
    }

    #[test]
    fn aoc2018_23_parser_nanobot_negative_coordinates() -> Result<(), String> {
        let inp = "pos=<-123,234,-345>, r=456";
        let parser = Parser::new().unwrap();
        let nanobot = parser.parse(inp);
        assert_eq!(nanobot.point.x, -123);
        assert_eq!(nanobot.point.y, 234);
        assert_eq!(nanobot.point.z, -345);
        assert_eq!(nanobot.radius, 456);
        Ok(())
    }

    #[test]
    fn aoc2018_23_correctness() -> io::Result<()> {
        let sol = AoC2018_23::new()?;
        assert_eq!(sol.part_one(), "164");
        assert_eq!(sol.part_two(), "122951778");
        Ok(())
    }
}
