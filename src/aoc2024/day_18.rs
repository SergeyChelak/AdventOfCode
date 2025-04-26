use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

type Int = isize;
type Position = Point2d<Int>;

pub struct AoC2024_18 {
    coordinates: Vec<Position>,
    target: Position,
    limit: usize,
}

impl AoC2024_18 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2024_18")?;
        Ok(Self::with_lines(&lines, Position::new(70, 70), 1024))
    }

    fn with_lines<T: AsRef<str>>(input: &[T], target: Position, limit: usize) -> Self {
        let coordinates = input
            .iter()
            .map(|s| s.as_ref())
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.split_once(',').expect("Invalid input format"))
            .map(|(r, c)| {
                let y = r.parse::<Int>().expect("Invalid row value");
                let x = c.parse::<Int>().expect("Invalid col value");
                Position { y, x }
            })
            .collect::<Vec<_>>();
        Self {
            coordinates,
            target,
            limit,
        }
    }

    fn make_map(&self, limit: usize) -> Vec2<bool> {
        build_map(
            &self.coordinates,
            limit,
            self.target.y + 1,
            self.target.x + 1,
        )
    }
}

impl Solution for AoC2024_18 {
    fn part_one(&self) -> String {
        let map = self.make_map(self.limit);
        dfs(&map, self.target)
            .map(|x| x.to_string())
            .unwrap_or("Not found".to_string())
    }

    fn part_two(&self) -> String {
        let mut left = self.limit;
        let mut right = self.coordinates.len() - 1;
        while left < right {
            let mid = (left + right) >> 1;
            let map = self.make_map(mid);
            if dfs(&map, self.target).is_none() {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        let p = self.coordinates[left - 1];
        format!("{},{}", p.y, p.x)
    }

    fn description(&self) -> String {
        "2024/Day 18: RAM Run".to_string()
    }
}

fn build_map(positions: &[Position], limit: usize, rows: Int, cols: Int) -> Vec2<bool> {
    let limit = limit.min(positions.len());
    let set = positions[..limit].iter().collect::<HashSet<_>>();
    let mut map = Vec::new();
    for row in 0..rows {
        let mut arr = Vec::new();
        for col in 0..cols {
            let p = Position { y: row, x: col };
            arr.push(set.contains(&p));
        }
        map.push(arr);
    }
    map
}

fn dfs(map: &[Vec<bool>], target: Position) -> Option<usize> {
    let mut elements = vec![Position::new(0, 0)];
    let mut visited = HashSet::new();
    let mut step = 0;
    let rows = map.len() as isize;
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
            let cols = map[pos.y as usize].len() as isize;
            Direction::all()
                .iter()
                .map(|dir| pos.moved_by(dir))
                .filter(|p| (0..rows).contains(&p.y) && (0..cols).contains(&p.x))
                .filter(|p| !map[p.y as usize][p.x as usize])
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
        assert_eq!(sol.part_two(), "45,16");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2024_18> {
        AoC2024_18::new()
    }

    #[test]
    fn aoc2024_18_case_1() {
        let solution = make_test_solution();
        assert_eq!(solution.part_one(), "22");
    }

    #[test]
    fn aoc2024_18_case_2() {
        let solution = make_test_solution();
        assert_eq!(solution.part_two(), "6,1");
    }

    fn make_test_solution() -> AoC2024_18 {
        let input = [
            "5,4", "4,2", "4,5", "3,0", "2,1", "6,3", "2,4", "1,5", "0,6", "3,3", "2,6", "5,1",
            "1,2", "5,5", "2,5", "6,5", "1,4", "0,4", "6,4", "1,1", "6,1", "1,0", "0,5", "1,6",
            "2,0",
        ];
        AoC2024_18::with_lines(&input, Position::new(6, 6), 12)
    }
}
