use crate::solution::Solution;
use crate::utils::hyper_point::HyperPoint;

use std::collections::{HashSet, VecDeque};
use std::io;

type Int = i32;
type Point = HyperPoint<Int>;

pub struct AoC2022_18 {
    input: Vec<Point>,
}

impl AoC2022_18 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2022_18")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        Self::parse_lines(&data.lines().collect::<Vec<_>>())
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(|x| Point::from_csv(x).expect("Invalid input format"))
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2022_18 {
    fn part_one(&self) -> String {
        let all_points = self.input.iter().cloned().collect::<HashSet<_>>();
        let mut surface = 0;
        for point in self.input.iter() {
            surface += 6 - deltas()
                .iter()
                .map(|delta| point.add(delta))
                .filter(|p| all_points.contains(p))
                .count();
        }
        surface.to_string()
    }

    fn part_two(&self) -> String {
        let (min_x, max_x) = (
            get_min(&self.input, |p| p.0[0]) - 1,
            get_max(&self.input, |p| p.0[0]) + 1,
        );
        let (min_y, max_y) = (
            get_min(&self.input, |p| p.0[1]) - 1,
            get_max(&self.input, |p| p.0[1]) + 1,
        );
        let (min_z, max_z) = (
            get_min(&self.input, |p| p.0[2]) - 1,
            get_max(&self.input, |p| p.0[2]) + 1,
        );

        let all_points = self.input.iter().cloned().collect::<HashSet<_>>();
        let start = Point::from(vec![min_x, min_y, min_z]);
        let mut seen = HashSet::new();
        seen.insert(start.clone());

        let mut queue = VecDeque::new();
        queue.push_back(start);

        let mut surface = 0usize;
        while let Some(point) = queue.pop_back() {
            deltas()
                .iter()
                .map(|delta| point.add(delta))
                .filter(|p| {
                    // if point in allowed range
                    (min_x..=max_x).contains(&p.0[0])
                        && (min_y..=max_y).contains(&p.0[1])
                        && (min_z..=max_z).contains(&p.0[2])
                })
                .for_each(|p| {
                    if all_points.contains(&p) {
                        surface += 1;
                    } else if seen.insert(p.clone()) {
                        queue.push_front(p);
                    }
                });
        }

        surface.to_string()
    }

    fn description(&self) -> String {
        "Day 18: Boiling Boulders".to_string()
    }
}

fn get_min(input: &[Point], transform: impl Fn(&Point) -> Int) -> Int {
    input.iter().map(transform).min().unwrap()
}

fn get_max(input: &[Point], transform: impl Fn(&Point) -> Int) -> Int {
    input.iter().map(transform).max().unwrap()
}

fn deltas() -> [Point; 6] {
    [
        Point::from(vec![1, 0, 0]),
        Point::from(vec![-1, 0, 0]),
        Point::from(vec![0, 1, 0]),
        Point::from(vec![0, -1, 0]),
        Point::from(vec![0, 0, 1]),
        Point::from(vec![0, 0, -1]),
    ]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_18_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2022_18_case_1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "64")
    }

    #[test]
    fn aoc2022_18_case_2() {
        let sol = make_test_solution();
        assert_eq!(sol.part_two(), "58")
    }

    #[test]
    fn aoc2022_18_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "4604");
        Ok(())
    }

    #[test]
    fn aoc2022_18_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "2604");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2022_18> {
        AoC2022_18::new()
    }

    fn make_test_solution() -> AoC2022_18 {
        AoC2022_18::parse_lines(&[
            "2,2,2", "1,2,2", "3,2,2", "2,1,2", "2,3,2", "2,2,1", "2,2,3", "2,2,4", "2,2,6",
            "1,2,5", "3,2,5", "2,1,5", "2,3,5",
        ])
    }
}
