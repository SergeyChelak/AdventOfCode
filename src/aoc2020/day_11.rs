use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io;

pub struct AoC2020_11 {
    input: Vec2<char>,
}

impl AoC2020_11 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2020_11")?;
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

    fn occupied_count(
        &self,
        criteria: impl Fn(&Vec2<char>, Position) -> usize,
        tolerance: usize,
    ) -> String {
        let mut hashes = HashSet::new();
        let mut matrix = self.input.clone();
        loop {
            let mut hasher = std::hash::DefaultHasher::new();
            matrix.hash(&mut hasher);
            let hash = hasher.finish();
            if hashes.contains(&hash) {
                break;
            }
            hashes.insert(hash);
            matrix = next_generation(&matrix, &criteria, tolerance);
        }
        total_occupied_count(&matrix).to_string()
    }
}

impl Solution for AoC2020_11 {
    fn part_one(&self) -> String {
        self.occupied_count(close_adjacent_occupied, 4).to_string()
    }

    fn part_two(&self) -> String {
        self.occupied_count(far_adjacent_occupied, 5).to_string()
    }

    fn description(&self) -> String {
        "Day 11: Seating System".to_string()
    }
}

type Position = Point2d<usize>;

const POSITION_EMPTY: char = 'L';
const POSITION_OCCUPIED: char = '#';
const POSITION_FLOOR: char = '.';

fn next_generation(
    matrix: &Vec2<char>,
    adj_count: &impl Fn(&Vec2<char>, Position) -> usize,
    tolerance: usize,
) -> Vec2<char> {
    let mut new = matrix.clone();
    for (i, row) in matrix.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == POSITION_FLOOR {
                continue;
            }
            let pos = Position::new(j, i);
            let occupied = adj_count(matrix, pos);
            let val_next = match (*val, occupied) {
                (POSITION_EMPTY, 0) => POSITION_OCCUPIED,
                (POSITION_OCCUPIED, x) if x >= tolerance => POSITION_EMPTY,
                (x, _) => x,
            };
            new[i][j] = val_next;
        }
    }
    new
}

fn close_adjacent_occupied(matrix: &Vec2<char>, pos: Position) -> usize {
    Direction::circular_directions()
        .iter()
        .filter_map(|dir| pos.safe_moved_with_dirs(dir))
        .filter_map(|p| matrix.get(p.y)?.get(p.x))
        .filter(|ch| **ch == POSITION_OCCUPIED)
        .count()
}

fn far_adjacent_occupied(matrix: &Vec2<char>, pos: Position) -> usize {
    let mut count = 0;
    for dir in Direction::circular_directions() {
        let mut tmp = pos;
        while let Some(next) = tmp.safe_moved_with_dirs(&dir) {
            let Some(ch) = matrix.get(next.y).and_then(|row| row.get(next.x)) else {
                break;
            };
            match *ch {
                POSITION_FLOOR => {
                    tmp = next;
                    continue;
                }
                x => {
                    if x == POSITION_OCCUPIED {
                        count += 1;
                    }
                    break;
                }
            };
        }
    }

    count
}

fn total_occupied_count(matrix: &Vec2<char>) -> usize {
    matrix
        .iter()
        .flatten()
        .filter(|ch| **ch == POSITION_OCCUPIED)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_11_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2020_11_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "2321");
        Ok(())
    }

    #[test]
    fn aoc2020_11_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "2102");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_11> {
        AoC2020_11::new()
    }
}
