use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
use std::io;

type Coordinate = Position2<isize>;

type CoordinateSet = HashSet<Coordinate>;
type CharCoordinateMap = HashMap<char, Vec<Coordinate>>;

pub struct AoC2024_08 {
    input: Vec<Vec<char>>,
}

impl AoC2024_08 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2024_08")?;
        Ok(Self::with_strings(&lines))
    }

    fn with_strings(arr: &[String]) -> Self {
        let input = arr
            .iter()
            .map(|s| s.chars().collect::<Vec<_>>())
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
                    let d_row = b.row - a.row;
                    let d_col = b.col - a.col;
                    [
                        Coordinate::new(a.row - d_row, a.col - d_col),
                        Coordinate::new(b.row + d_row, b.col + d_col),
                    ]
                    .iter()
                    .filter(|coord| {
                        coord.row >= 0 && coord.row < rows && coord.col >= 0 && coord.col < cols
                    })
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
                for b in arr.iter().skip(i + 1) {
                    let d_row = b.row - a.row;
                    let d_col = b.col - a.col;
                    for k in 0.. {
                        let coord = Coordinate::new(a.row - k * d_row, a.col - k * d_col);
                        if coord.row >= 0 && coord.row < rows && coord.col >= 0 && coord.col < cols
                        {
                            locations.insert(coord);
                        } else {
                            break;
                        }
                    }

                    for k in 0.. {
                        let coord = Coordinate::new(b.row + k * d_row, b.col + k * d_col);
                        if coord.row >= 0 && coord.row < rows && coord.col >= 0 && coord.col < cols
                        {
                            locations.insert(coord);
                        } else {
                            break;
                        }
                    }
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
            entry.push(Coordinate::new(row as isize, col as isize));
        }
    }
    map
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
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
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
