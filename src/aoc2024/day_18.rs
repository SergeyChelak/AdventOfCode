use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

type Int = isize;
type Position = Position2<Int>;

pub struct AoC2024_18 {
    coordinates: Vec<Position>,
    rows: Int,
    cols: Int,
}

impl AoC2024_18 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2024_18")?;
        Ok(Self::with_lines(&lines, 70, 70))
    }

    fn with_lines<T: AsRef<str>>(input: &[T], rows: Int, cols: Int) -> Self {
        let coordinates = input
            .iter()
            .map(|s| s.as_ref())
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.split_once(',').expect("Invalid input format"))
            .map(|(r, c)| {
                let row = r.parse::<Int>().expect("Invalid row value");
                let col = c.parse::<Int>().expect("Invalid col value");
                Position { row, col }
            })
            .collect::<Vec<_>>();
        Self {
            coordinates,
            rows,
            cols,
        }
    }
}

impl Solution for AoC2024_18 {
    fn part_one(&self) -> String {
        dfs(&self.coordinates, 1024, self.rows, self.cols)
            .map(|x| x.to_string())
            .unwrap_or("Not found".to_string())
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "2024/Day 18: RAM Run".to_string()
    }
}

fn dfs(positions: &[Position], limit: usize, rows: Int, cols: Int) -> Option<usize> {
    let limit = limit.min(positions.len());
    let mut elements = vec![Position::new(0, 0)];
    let mut visited = HashSet::new();
    let target = Position::new(rows, cols);
    let mut step = 0;
    while !elements.is_empty() {
        let mut next = Vec::new();
        for pos in elements {
            if pos == target {
                return Some(step);
            }
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);
            [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .iter()
                .map(|(dr, dc)| Position::new(pos.row + *dr, pos.col + *dc))
                .filter(|&p| p.row >= 0 && p.col >= 0 && p.row <= rows && p.col <= cols)
                .filter(|p| !positions[..limit].contains(p))
                .for_each(|p| {
                    next.push(p);
                });
        }
        elements = next;
        step += 1;
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_18_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.coordinates.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2024_18_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "408");
        Ok(())
    }

    #[test]
    fn aoc2024_18_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2024_18> {
        AoC2024_18::new()
    }

    #[test]
    fn aoc2024_18_case_1() {
        let solution = make_test_solution();
        assert_eq!(
            dfs(&solution.coordinates, 12, solution.rows, solution.cols),
            Some(22)
        );
    }

    fn make_test_solution() -> AoC2024_18 {
        let input = [
            "5,4", "4,2", "4,5", "3,0", "2,1", "6,3", "2,4", "1,5", "0,6", "3,3", "2,6", "5,1",
            "1,2", "5,5", "2,5", "6,5", "1,4", "0,4", "6,4", "1,1", "6,1", "1,0", "0,5", "1,6",
            "2,0",
        ];
        AoC2024_18::with_lines(&input, 6, 6)
    }
}
