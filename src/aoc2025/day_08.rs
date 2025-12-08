use crate::solution::Solution;
use crate::utils::hyper_point::HyperPoint;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
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
}

impl Solution for AoC2025_08 {
    fn part_one(&self) -> String {
        compute(&self.input, 1000).to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 8: Playground".to_string()
    }
}

fn compute(points: &[Point], connections: usize) -> usize {
    let count = points.len();

    let mut chain_mapping = (0..count).collect::<Vec<_>>();
    let mut pairs: HashSet<IndexPair> = HashSet::new();

    let mut next_id = count + 100;

    for _ in 0..connections {
        // find closest without connection
        let mut candidate = (IndexPair::new(count + 1, count + 1), usize::MAX);
        for (i, a) in points.iter().enumerate() {
            let Some(search_result) = points
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(j, val)| (IndexPair::new(i, j), val))
                .filter(|(pair, _)| !pairs.contains(pair))
                .map(|(pair, b)| (pair, b.dist_sqr(a)))
                .min_by_key(|x| x.1)
            else {
                unreachable!("Unexpected case, will think alter...");
            };
            if search_result.1 < candidate.1 {
                candidate = search_result;
            }
        }

        // merge chains
        let pair = candidate.0;
        pairs.insert(pair);
        let chain_1 = chain_mapping[pair.first];
        let chain_2 = chain_mapping[pair.second];
        chain_mapping
            .iter_mut()
            .filter(|x| **x == chain_1 || **x == chain_2)
            .for_each(|x| *x = next_id);
        next_id += 1;
        // println!("{:?}", chain_mapping);
    }

    // calc chain's sizes
    let mut size_map = HashMap::new();
    chain_mapping.iter().for_each(|x| {
        let entry = size_map.entry(*x).or_insert(0usize);
        *entry += 1;
    });
    let mut arr = size_map.values().collect::<Vec<_>>();
    arr.sort();
    arr = arr.into_iter().rev().collect();
    assert!(arr.len() > 2);

    arr[0] * arr[1] * arr[2]
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct IndexPair {
    first: usize,
    second: usize,
}

impl IndexPair {
    fn new(a: usize, b: usize) -> Self {
        Self {
            first: a.min(b),
            second: a.max(b),
        }
    }
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
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2025_08_case_1() {
        let sol = make_test_solution();
        assert_eq!(compute(&sol.input, 10), 40);
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
