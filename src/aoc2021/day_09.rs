use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

type Int = u32;
type Point = Point2d<usize>;

pub struct AoC2021_09 {
    input: Vec2<Int>,
}

impl AoC2021_09 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2021_09")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(|row| {
                row.chars()
                    .map(|ch| ch.to_digit(10).expect("input must contain digits only"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2021_09 {
    fn part_one(&self) -> String {
        let mut acc = 0;
        for (row, arr) in self.input.iter().enumerate() {
            for (col, val) in arr.iter().enumerate() {
                let pos = Point::new(col, row);
                let low_point = Direction::all()
                    .iter()
                    .filter_map(|dir| pos.safe_moved_by(dir))
                    .filter_map(|adj| self.input.get(adj.y).and_then(|r| r.get(adj.x)))
                    .all(|adj| adj > val);
                if low_point {
                    acc += 1 + val;
                }
            }
        }
        acc.to_string()
    }

    fn part_two(&self) -> String {
        let mut seen = HashSet::new();
        let mut result = Vec::new();
        for (row, arr) in self.input.iter().enumerate() {
            for (col, val) in arr.iter().enumerate() {
                if *val == 9 {
                    continue;
                }
                let start = Point::new(col, row);
                if seen.contains(&start) {
                    continue;
                }
                let size = search(&self.input, start, &mut seen);
                result.push(size);
            }
        }
        result.sort();
        result.iter().rev().take(3).product::<usize>().to_string()
    }

    fn description(&self) -> String {
        "Day 9: Smoke Basin".to_string()
    }
}

fn search(matrix: &Vec2<Int>, start: Point, seen: &mut HashSet<Point>) -> usize {
    let mut size = 0;
    let mut coordinates = vec![start];

    while let Some(p) = coordinates.pop() {
        if seen.contains(&p) {
            continue;
        }
        seen.insert(p);
        let mut adjacent = Direction::all()
            .iter()
            .filter_map(|dir| p.safe_moved_by(dir))
            .filter(|adj| !seen.contains(adj))
            .filter(|adj| {
                let Some(adj_val) = matrix.get(adj.y).and_then(|r| r.get(adj.x)) else {
                    return false;
                };
                if *adj_val == 9 {
                    return false;
                }
                true
            })
            .collect::<Vec<_>>();
        coordinates.append(&mut adjacent);
        size += 1;
    }
    size
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_09_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2021_09_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "522");
        Ok(())
    }

    #[test]
    fn aoc2021_09_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "916688");
        Ok(())
    }

    #[test]
    fn aoc2021_09_case_1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "15");
    }

    #[test]
    fn aoc2021_09_case_2() {
        let sol = make_test_solution();
        assert_eq!(sol.part_two(), "1134");
    }

    fn make_solution() -> io::Result<AoC2021_09> {
        AoC2021_09::new()
    }

    fn make_test_solution() -> AoC2021_09 {
        let input = [
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ];
        AoC2021_09::parse_lines(&input)
    }
}
