use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
use std::io;

type Coordinate = Point2d<isize>;

type CoordinateSet = HashSet<Coordinate>;
type CharCoordinateMap = HashMap<char, Vec<Coordinate>>;

pub struct AoC2024_08 {
    input: Vec2<char>,
}

impl AoC2024_08 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2024_08")?;
        Ok(Self::with_strings(&lines))
    }

    fn with_strings<T: AsRef<str>>(arr: &[T]) -> Self {
        let input = arr
            .iter()
            .map(|s| s.as_ref().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2024_08 {
    fn part_one(&self) -> String {
        let map = make_char_coordinate_map(&self.input);
        let mut locations = CoordinateSet::new();
        for (_, entry) in map.iter() {
            if entry.len() < 2 {
                continue;
            }
            let rows = self.input.len() as isize;
            let arr = entry.iter().collect::<Vec<_>>();
            for (i, a) in arr.iter().enumerate() {
                let cols = self.input[i].len() as isize;
                for b in arr.iter().skip(i + 1) {
                    let d_row = b.y - a.y;
                    let d_col = b.x - a.x;
                    [
                        Coordinate::new(a.y - d_row, a.x - d_col),
                        Coordinate::new(b.y + d_row, b.x + d_col),
                    ]
                    .iter()
                    .filter(|coord| is_valid(coord, rows, cols))
                    .for_each(|coord| {
                        locations.insert(*coord);
                    });
                }
            }
        }
        locations.len().to_string()
    }

    fn part_two(&self) -> String {
        let map = make_char_coordinate_map(&self.input);
        let mut locations = CoordinateSet::new();

        for (_, entry) in map.iter() {
            if entry.len() < 2 {
                continue;
            }
            let rows = self.input.len() as isize;
            let arr = entry.iter().collect::<Vec<_>>();
            for (i, a) in arr.iter().enumerate() {
                let cols = self.input[i].len() as isize;
                let mut append_from_line = |coord: &Coordinate, d_row: isize, d_col: isize| {
                    for k in 0.. {
                        let coord = Coordinate::new(coord.y - k * d_row, coord.x - k * d_col);
                        if !is_valid(&coord, rows, cols) {
                            break;
                        }
                        locations.insert(coord);
                    }
                };
                for b in arr.iter().skip(i + 1) {
                    let d_row = b.y - a.y;
                    let d_col = b.x - a.x;
                    append_from_line(a, -d_row, -d_col);
                    append_from_line(b, d_row, d_col);
                }
            }
        }
        locations.len().to_string()
    }

    fn description(&self) -> String {
        "2024/Day 8: Resonant Collinearity".to_string()
    }
}

fn make_char_coordinate_map(input: &[Vec<char>]) -> CharCoordinateMap {
    let mut map = CharCoordinateMap::new();
    for (row, arr) in input.iter().enumerate() {
        for (col, ch) in arr.iter().enumerate() {
            if *ch == '.' {
                continue;
            }
            let entry = map.entry(*ch).or_default();
            entry.push(Coordinate::new(col as isize, row as isize));
        }
    }
    map
}

fn is_valid(coord: &Coordinate, rows: isize, cols: isize) -> bool {
    coord.y >= 0 && coord.y < rows && coord.x >= 0 && coord.x < cols
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_08_input_load_test() -> io::Result<()> {
        let sol = AoC2024_08::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2024_08_case_1() {
        let puzzle = make_puzzle();
        assert_eq!(puzzle.part_one(), "14")
    }

    #[test]
    fn aoc2024_08_case_2() {
        let puzzle = make_puzzle();
        assert_eq!(puzzle.part_two(), "34")
    }

    fn make_puzzle() -> AoC2024_08 {
        let input = [
            "............",
            "........0...",
            ".....0......",
            ".......0....",
            "....0.......",
            "......A.....",
            "............",
            "............",
            "........A...",
            ".........A..",
            "............",
            "............",
        ];
        AoC2024_08::with_strings(&input)
    }

    #[test]
    fn aoc2024_08_correctness() -> io::Result<()> {
        let sol = AoC2024_08::new()?;
        assert_eq!(sol.part_one(), "265");
        assert_eq!(sol.part_two(), "962");
        Ok(())
    }
}
