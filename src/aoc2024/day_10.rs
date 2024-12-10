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

    fn with_lines(lines: &[String]) -> Self {
        let map = lines
            .iter()
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
            .map(|coord| trailhead_score(&self.map, *coord))
            .sum::<usize>()
            .to_string()
    }

    // fn part_two(&self) -> String {
    // }

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

fn trailhead_score(map: &[Vec<u8>], start: Coordinate) -> usize {
    assert!(map[start.row][start.col] == 0);
    let mut stack = vec![start];
    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut count = 0;
    let rows = map.len();
    while let Some(coordinate) = stack.pop() {
        visited.insert(coordinate);
        let Coordinate { row, col } = coordinate;
        if map[row][col] == 9 {
            count += 1;
            continue;
        }
        let cols = map[coordinate.row].len();
        let expected_value = map[row][col] + 1;
        let mut adjacent = Direction::all()
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
            .filter(|c| !visited.contains(c))
            .filter(|c| map[c.row][c.col] == expected_value)
            .collect::<Vec<_>>();
        stack.append(&mut adjacent);
    }
    count
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
        ].as_strings();
        AoC2024_10::with_lines(&input)
    }

    #[test]
    fn aoc2024_10_correctness() -> io::Result<()> {
        let sol = AoC2024_10::new()?;
        assert_eq!(sol.part_one(), "778");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
