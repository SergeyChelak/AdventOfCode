use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

type Int = u32;

pub struct AoC2021_11 {
    input: Vec2<Int>,
}

impl AoC2021_11 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2021_11")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(|line| {
                line.chars()
                    .map(|ch| ch.to_digit(10).expect("Input must be digits only"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2021_11 {
    fn part_one(&self) -> String {
        let mut matrix = self.input.clone();
        let mut acc = 0;
        for _ in 0..100 {
            acc += simulate(&mut matrix);
        }
        acc.to_string()
    }

    fn part_two(&self) -> String {
        let mut matrix = self.input.clone();
        for step in 1..usize::MAX {
            let flashes = simulate(&mut matrix);
            if flashes == 100 {
                return step.to_string();
            }
        }
        not_found()
    }

    fn description(&self) -> String {
        "Day 11: Dumbo Octopus".to_string()
    }
}

type Point = Point2d<usize>;

fn simulate(input: &mut Vec2<Int>) -> usize {
    let mut flashes = HashSet::new();

    for (r, row) in input.iter_mut().enumerate() {
        for (c, val) in row.iter_mut().enumerate() {
            *val += 1;
            if *val > 9 {
                let point = Point::new(c, r);
                flashes.insert(point);
            }
        }
    }

    let mut seen = HashSet::new();
    while !flashes.is_empty() {
        let mut new_flashes = HashSet::new();
        for p in flashes.into_iter() {
            if seen.contains(&p) {
                continue;
            }
            Direction::circular_directions()
                .iter()
                .filter_map(|dirs| p.safe_moved_with_dirs(dirs))
                .for_each(|adj| {
                    if input.get(adj.y).and_then(|row| row.get(adj.x)).is_none() {
                        return;
                    }
                    input[adj.y][adj.x] += 1;
                    if input[adj.y][adj.x] > 9 {
                        new_flashes.insert(adj);
                    }
                });
            seen.insert(p);
        }
        flashes = new_flashes;
    }
    for p in seen.iter() {
        input[p.y][p.x] = 0;
    }
    seen.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_11_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2021_11_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "1694");
        Ok(())
    }

    #[test]
    fn aoc2021_11_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "346");
        Ok(())
    }

    #[test]
    fn aoc2021_11_case_1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "1656");
    }

    fn make_solution() -> io::Result<AoC2021_11> {
        AoC2021_11::new()
    }

    fn make_test_solution() -> AoC2021_11 {
        let lines = [
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526",
        ];
        AoC2021_11::parse_lines(&lines)
    }
}
