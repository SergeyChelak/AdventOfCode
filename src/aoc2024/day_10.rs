use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

pub struct AoC2024_10 {
    map: Vec2<u8>,
}

impl AoC2024_10 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2024_10")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let map = lines
            .iter()
            .map(|x| x.as_ref())
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.chars()
                    .map(|ch| {
                        ch.to_digit(10)
                            .map(|x| x as u8)
                            .expect("non digit character found in the input data")
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { map }
    }
}

impl Solution for AoC2024_10 {
    fn part_one(&self) -> String {
        find_trailheads(&self.map)
            .iter()
            .map(|coord| trailhead_value(&self.map, *coord, SearchMode::Score))
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        find_trailheads(&self.map)
            .iter()
            .map(|coord| trailhead_value(&self.map, *coord, SearchMode::Rate))
            .sum::<usize>()
            .to_string()
    }

    fn description(&self) -> String {
        "Day 10: Hoof It".to_string()
    }
}

type Coordinate = Position2<usize>;

fn find_trailheads(map: &[Vec<u8>]) -> Vec<Coordinate> {
    let mut result = Vec::new();
    for (row, arr) in map.iter().enumerate() {
        for (col, val) in arr.iter().enumerate() {
            if *val == 0 {
                result.push(Coordinate::new(row, col));
            }
        }
    }
    result
}

enum SearchMode {
    Score,
    Rate,
}

fn trailhead_value(map: &[Vec<u8>], start: Coordinate, mode: SearchMode) -> usize {
    assert!(map[start.row][start.col] == 0);
    let mut stack = vec![start];
    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut count = 0;
    while let Some(coordinate) = stack.pop() {
        if matches!(mode, SearchMode::Score) {
            visited.insert(coordinate);
        }
        let Coordinate { row, col } = coordinate;
        if map[row][col] == 9 {
            count += 1;
            continue;
        }
        get_adjacent(map, coordinate)
            .iter()
            .filter(|c| match mode {
                SearchMode::Score => !visited.contains(c),
                SearchMode::Rate => true,
            })
            .for_each(|c| {
                stack.push(*c);
            });
    }
    count
}

fn get_adjacent(map: &[Vec<u8>], coordinate: Coordinate) -> Vec<Coordinate> {
    let Coordinate { row, col } = coordinate;
    let rows = map.len();
    let cols = map[row].len();
    let expected_value = map[row][col] + 1;
    Direction::all()
        .iter()
        .map(|dir| {
            use Direction::*;
            match dir {
                Left if col > 0 => Coordinate::new(row, col - 1),
                Right if col < cols - 1 => Coordinate::new(row, col + 1),
                Up if row > 0 => Coordinate::new(row - 1, col),
                Down if row < rows - 1 => Coordinate::new(row + 1, col),
                _ => coordinate,
            }
        })
        .filter(|c| map[c.row][c.col] == expected_value)
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_10_input_load_test() -> io::Result<()> {
        let sol = AoC2024_10::new()?;
        assert!(!sol.map.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2024_10_case_1() {
        let puzzle = make_puzzle();
        assert_eq!(puzzle.part_one(), "36")
    }

    #[test]
    fn aoc2024_10_case_2() {
        let puzzle = make_puzzle();
        assert_eq!(puzzle.part_two(), "81")
    }

    fn make_puzzle() -> AoC2024_10 {
        #[rustfmt::skip]
        let input = [
            "89010123",
            "78121874",
            "87430965",
            "96549874",
            "45678903",
            "32019012",
            "01329801",
            "10456732",
        ];
        AoC2024_10::with_lines(&input)
    }

    #[test]
    fn aoc2024_10_correctness() -> io::Result<()> {
        let sol = AoC2024_10::new()?;
        assert_eq!(sol.part_one(), "778");
        assert_eq!(sol.part_two(), "1925");
        Ok(())
    }
}
