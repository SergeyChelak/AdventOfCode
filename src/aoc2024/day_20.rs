use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, VecDeque};
use std::io;

type Int = usize;
type Position = Position2<Int>;
const WALL: char = '#';

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
        let start = get_first_position(&self.map, 'S').expect("Start position not found");
        let distances = distances_with_bfs(&self.map, start);
        let mut total = 0;
        let rows = self.map.len();
        let cols = self.map[0].len();
        for r in 1..rows - 1 {
            for c in 1..cols - 1 {
                if self.map[r][c] == WALL {
                    continue;
                }
                let dist = *distances
                    .get(&Position::new(r, c))
                    .expect("Bug in code (2)");
                let mut positions = Vec::new();
                if r < rows - 2 {
                    positions.push(Position::new(r + 2, c));
                }
                if c < cols - 2 {
                    positions.push(Position::new(r, c + 2));
                }
                positions.push(Position::new(r + 1, c + 1));
                positions.push(Position::new(r - 1, c + 1));
                for p in positions {
                    if self.map[p.row][p.col] == WALL {
                        continue;
                    }
                    let val = *distances.get(&p).expect("Bug in code (3)");
                    if val.abs_diff(dist) >= 102 {
                        total += 1;
                    }
                }
            }
        }
        total.to_string()
    }

    fn part_two(&self) -> String {
        let start = get_first_position(&self.map, 'S').expect("Start position not found");
        let distances = distances_with_bfs(&self.map, start);
        let mut total = 0;
        for (pos1, dist1) in distances.iter() {
            for (pos2, dist2) in distances.iter() {
                if pos1 == pos2 {
                    continue;
                }
                let val = pos1.row.abs_diff(pos2.row) + pos1.col.abs_diff(pos2.col);
                if val > 20 {
                    continue;
                }
                if *dist1 + val + 100 <= *dist2 {
                    total += 1;
                }
            }
        }
        total.to_string()
    }

    fn description(&self) -> String {
        "2024/Day 20: Race Condition".to_string()
    }
}

fn distances_with_bfs(map: &[Vec<char>], start: Position) -> HashMap<Position, usize> {
    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut distances = HashMap::new();
    distances.insert(start, 0usize);

    let rows = map.len();
    let cols = map[0].len();

    while let Some(elem) = queue.pop_front() {
        let dist = *distances.get(&elem).expect("check code (1)");
        for dir in Direction::all() {
            let next = match dir {
                Direction::Up if elem.row > 0 => Position::new(elem.row - 1, elem.col),
                Direction::Down if elem.row < rows - 1 => Position::new(elem.row + 1, elem.col),
                Direction::Left if elem.col > 0 => Position::new(elem.row, elem.col - 1),
                Direction::Right if elem.col < cols - 1 => Position::new(elem.row, elem.col + 1),
                _ => continue,
            };
            if map[next.row][next.col] == WALL {
                continue;
            }
            if distances.contains_key(&next) {
                continue;
            }
            distances.insert(next, 1 + dist);
            queue.push_back(next);
        }
    }
    distances
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
        assert_eq!(sol.part_two(), "1010981");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2024_20> {
        AoC2024_20::new()
    }
}
