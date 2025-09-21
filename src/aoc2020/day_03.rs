use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Position = Point2d<usize>;

pub struct AoC2020_03 {
    input: Vec2<char>,
}

impl AoC2020_03 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2020_03")?;
        Ok(Self::parse(&lines))
    }

    fn parse<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect();
        Self { input }
    }

    fn trees_count(&self, step: &Position) -> usize {
        let mut current = Position::zero();
        let mut trees = 0usize;
        while current.y < self.input.len() {
            let width = self.input[current.y].len();
            if self.input[current.y][current.x % width] == '#' {
                trees += 1;
            }
            current = current.add(step);
        }
        trees
    }
}

impl Solution for AoC2020_03 {
    fn part_one(&self) -> String {
        let step = Position::new(3, 1);
        self.trees_count(&step).to_string()
    }

    fn part_two(&self) -> String {
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|(x, y)| Position::new(*x, *y))
            .map(|step| self.trees_count(&step))
            .product::<usize>()
            .to_string()
    }

    fn description(&self) -> String {
        "Day 3: Toboggan Trajectory".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_03_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2020_03_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "223");
        Ok(())
    }

    #[test]
    fn aoc2020_03_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "3517401300");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_03> {
        AoC2020_03::new()
    }
}
