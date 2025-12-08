use crate::solution::Solution;
use crate::utils::hyper_point::HyperPoint;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

type Int = isize;
type Point = HyperPoint<Int>;

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let coordinates = value
            .split(',')
            .map(|x| x.parse::<Int>().expect("Invalid coordinate format"))
            .collect::<Vec<_>>();
        Self::from(coordinates)
    }
}

impl Point {
    fn dist_sqr(&self, other: &Self) -> usize {
        assert_eq!(self.dimension(), other.dimension());
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| a.abs_diff(*b).pow(2))
            .sum()
    }
}

pub struct AoC2025_08 {
    input: Vec<Point>,
}

impl AoC2025_08 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2025_08")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(Point::from)
            .collect::<Vec<_>>();
        Self { input }
    }

    fn part_one_ex(&self, iterations: usize) -> String {
        let mut engine = Engine::with(&self.input);
        for _ in 0..iterations {
            _ = engine.make_connection();
        }

        // calc chain's sizes
        let mut size_map = HashMap::new();
        engine.chain_mapping.iter().for_each(|x| {
            let entry = size_map.entry(*x).or_insert(0usize);
            *entry += 1;
        });
        let mut arr = size_map.values().collect::<Vec<_>>();
        arr.sort();
        arr.into_iter().rev().take(3).product::<usize>().to_string()
    }
}

impl Solution for AoC2025_08 {
    fn part_one(&self) -> String {
        self.part_one_ex(1000)
    }

    fn part_two(&self) -> String {
        let mut engine = Engine::with(&self.input);
        let (mut first, mut second) = (0usize, 0usize);
        while let Some(value) = engine.make_connection() {
            first = value.first;
            second = value.second;

            let mut iter = engine.chain_mapping.iter();
            let first = iter.next().expect("Chain map shouldn't be empty");
            if iter.all(|val| val == first) {
                break;
            }
        }
        assert_ne!(first, second);
        let p1 = &self.input[first];
        let p2 = &self.input[second];
        (p1.0[0] * p2.0[0]).to_string()
    }

    fn description(&self) -> String {
        "Day 8: Playground".to_string()
    }
}

struct Engine {
    chain_mapping: Vec<usize>,
    distances: Vec<DistanceData>,
}

impl Engine {
    fn with(points: &[Point]) -> Self {
        let chain_mapping = (0..points.len()).collect::<Vec<_>>();
        let distances = Self::precalculate_distances(points);
        Self {
            chain_mapping,
            distances,
        }
    }

    fn precalculate_distances(points: &[Point]) -> Vec<DistanceData> {
        let mut array = Vec::new();
        for (i, first) in points.iter().enumerate() {
            for (j, second) in points.iter().enumerate().skip(i + 1) {
                let distance = first.dist_sqr(second);
                let data = DistanceData {
                    first: i,
                    second: j,
                    distance,
                };
                array.push(data);
            }
        }
        array.sort_by_key(|val| val.distance);
        array.into_iter().rev().collect()
    }

    fn make_connection(&mut self) -> Option<DistanceData> {
        let data = self.distances.pop()?;
        // merge chains
        let chain_1 = self.chain_mapping[data.first];
        let chain_2 = self.chain_mapping[data.second];
        self.chain_mapping
            .iter_mut()
            .filter(|x| **x == chain_2)
            .for_each(|x| *x = chain_1);
        Some(data)
    }
}

#[derive(Debug)]
struct DistanceData {
    first: usize,
    second: usize,
    distance: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2025_08_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2025_08_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "97384");
        Ok(())
    }

    #[test]
    fn aoc2025_08_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "9003685096");
        Ok(())
    }

    #[test]
    fn aoc2025_08_case_1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one_ex(10), "40");
    }

    #[test]
    fn aoc2025_08_case_2() {
        let sol = make_test_solution();
        assert_eq!(sol.part_two(), "25272");
    }

    fn make_test_solution() -> AoC2025_08 {
        let input = [
            "162,817,812",
            "57,618,57",
            "906,360,560",
            "592,479,940",
            "352,342,300",
            "466,668,158",
            "542,29,236",
            "431,825,988",
            "739,650,466",
            "52,470,668",
            "216,146,977",
            "819,987,18",
            "117,168,530",
            "805,96,715",
            "346,949,466",
            "970,615,88",
            "941,993,340",
            "862,61,35",
            "984,92,344",
            "425,690,689",
        ];
        AoC2025_08::parse_lines(&input)
    }

    fn make_solution() -> io::Result<AoC2025_08> {
        AoC2025_08::new()
    }
}
