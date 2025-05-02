use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

const TILE_EMPTY: char = '.';
const TILE_BUG: char = '#';

pub struct AoC2019_24 {
    input: Vec<Vec<char>>,
}

impl AoC2019_24 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2019_24")?;
        // #[rustfmt::skip]
        // let lines = [
        //     "....#",
        //     "#..#.",
        //     "#..##",
        //     "..#..",
        //     "#....",
        // ];
        Ok(Self::with_lines(&lines))
    }

    fn with_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|line| line.as_ref())
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2019_24 {
    fn part_one(&self) -> String {
        let mut seen = HashSet::new();
        let mut next = self.input.clone();
        let value = loop {
            let s = flat(&next);
            if seen.contains(&s) {
                break s;
            }
            seen.insert(s);
            next = next_area(&next);
        };

        let rating = value
            .chars()
            .enumerate()
            .filter(|(_, ch)| *ch == TILE_BUG)
            .map(|(pos, _)| 1 << pos)
            .sum::<usize>();

        rating.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 24: Planet of Discord".to_string()
    }
}

type Position = Point2d<usize>;

fn next_area<T: AsRef<[char]>>(input: &[T]) -> Vec2<char> {
    let mut result = Vec::new();
    let rows = input.len();
    for (y, row) in input.iter().enumerate() {
        let mut arr = Vec::new();
        let row = row.as_ref();
        let cols = row.len();
        for (x, ch) in row.iter().enumerate() {
            let pos = Position::new(x, y);
            let bugs = Direction::all()
                .iter()
                .filter_map(|dir| pos.safe_moved_by(dir))
                .filter(|p| p.x < cols && p.y < rows)
                .filter(|p| (input[p.y].as_ref())[p.x] == TILE_BUG)
                .count();
            let value = match *ch {
                TILE_BUG if bugs != 1 => TILE_EMPTY,
                TILE_EMPTY if bugs == 1 || bugs == 2 => TILE_BUG,
                _ => *ch,
            };
            arr.push(value);
        }
        result.push(arr);
    }
    result
}

fn flat<T: AsRef<[char]>>(input: &[T]) -> String {
    input.iter().flat_map(|r| r.as_ref()).collect::<String>()
}

fn _dump<T: AsRef<[char]>>(input: &[T]) {
    for row in input {
        for ch in row.as_ref() {
            print!("{ch}");
        }
        println!()
    }
    println!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_24_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_24_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "28778811");
        Ok(())
    }

    #[test]
    fn aoc2019_24_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_24> {
        AoC2019_24::new()
    }
}
