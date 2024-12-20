use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

type Int = usize;
type Position = Position2<Int>;
const WALL: char = '#';
const EMPTY: char = '.';

pub struct AoC2024_20 {
    map: Vec2<char>,
}

impl AoC2024_20 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2024_20")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let map = lines
            .iter()
            .map(|x| x.as_ref())
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self { map }
    }
}

impl Solution for AoC2024_20 {
    fn part_one(&self) -> String {
        let mut total = 0;
        let mut map = self.map.clone();
        let start = get_first_position(&map, 'S').expect("Start position not found");
        let end = get_first_position(&map, 'E').expect("End position not found");

        let longest = get_path_len(&map, start, end).expect("Path not exist");

        let rows = self.map.len();
        for row in 1..rows - 1 {
            let cols = self.map[row].len();
            for col in 1..cols - 1 {
                if map[row][col] != WALL {
                    continue;
                }
                map[row][col] = EMPTY;
                for (r, c) in [(row + 1, col), (row, col + 1)] {
                    if map[r][c] == WALL {
                        continue;
                    }
                    if let Some(len) = get_path_len(&map, start, end) {
                        assert!(len <= longest);
                        if longest - len >= 100 {
                            total += 1;
                        }
                    }
                }
                map[row][col] = WALL;
            }
        }

        total.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "2024/Day 20: Race Condition".to_string()
    }
}

fn get_path_len(map: &[Vec<char>], start: Position, end: Position) -> Option<usize> {
    let mut elems = vec![start];
    let mut visited = HashSet::new();
    let mut len = 0;
    while !elems.is_empty() {
        let mut next = Vec::new();
        for elem in elems {
            if visited.contains(&elem) {
                continue;
            }
            if elem == end {
                return Some(len);
            }
            visited.insert(elem);
            Direction::all()
                .iter()
                .map(|d| match d {
                    Direction::Up => Position::new(elem.row - 1, elem.col),
                    Direction::Down => Position::new(elem.row + 1, elem.col),
                    Direction::Left => Position::new(elem.row, elem.col - 1),
                    Direction::Right => Position::new(elem.row, elem.col + 1),
                })
                .filter(|p| map[p.row][p.col] != WALL)
                .for_each(|p| next.push(p));
        }
        elems = next;
        len += 1;
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_20_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.map.is_empty());
        assert!(get_first_position(&sol.map, 'S').is_some());
        assert!(get_first_position(&sol.map, 'E').is_some());
        Ok(())
    }

    #[test]
    fn aoc2024_20_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "1404");
        Ok(())
    }

    #[test]
    fn aoc2024_20_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2024_20> {
        AoC2024_20::new()
    }

    // fn make_test_solution() -> AoC2024_20 {
    //     let lines = [
    //         "###############",
    //         "#...#.........#",
    //         "#.#.#.#.#.###.#",
    //         "#S#...#.#.#...#",
    //         "#######.#.#.###",
    //         "#######.#.#...#",
    //         "#######.#.###.#",
    //         "###..E#...#...#",
    //         "###.#######.###",
    //         "#...###...#...#",
    //         "#.#####.#.###.#",
    //         "#.#...#.#.#...#",
    //         "#.#.#.#.#.#.###",
    //         "#...#...#...###",
    //         "###############",
    //     ];
    //     AoC2024_20::with_lines(&lines)
    // }
}
