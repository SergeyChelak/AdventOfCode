use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

pub struct AoC2024_12 {
    input: Vec2<char>,
}

impl AoC2024_12 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2024_12")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|row| row.as_ref().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2024_12 {
    fn part_one(&self) -> String {
        let price =
            |region: &[Vec<char>], position: Position, visited: &mut HashSet<Position>| -> usize {
                let area = find_area(region, position, visited);
                find_perimeter(&area) * area.len()
            };
        calculate(&self.input, price).to_string()
    }

    fn part_two(&self) -> String {
        let price =
            |region: &[Vec<char>], position: Position, visited: &mut HashSet<Position>| -> usize {
                let area = find_area(region, position, visited);
                find_sides(&area) * area.len()
            };
        calculate(&self.input, price).to_string()
    }

    fn description(&self) -> String {
        "2024/Day 12: Garden Groups".to_string()
    }
}

type Position = Point2d<usize>;

fn calculate(
    region: &[Vec<char>],
    f: impl Fn(&[Vec<char>], Position, &mut HashSet<Position>) -> usize,
) -> usize {
    let mut visited = HashSet::new();
    let mut total = 0;
    for (row, arr) in region.iter().enumerate() {
        for (col, _) in arr.iter().enumerate() {
            let position = Position::new(col, row);
            if visited.contains(&position) {
                continue;
            }
            total += f(region, position, &mut visited);
        }
    }
    total
}

fn find_sides(area: &HashSet<Position>) -> usize {
    let area = area
        .iter()
        .map(|p| (p.y as isize * 10, p.x as isize * 10))
        .collect::<HashSet<_>>();
    let mut all_corners = HashSet::new();
    let offsets = [(-5, -5), (5, -5), (5, 5), (-5, 5)];
    for (row, col) in &area {
        for (dr, dc) in offsets {
            let r = row + dr;
            let c = col + dc;
            all_corners.insert((r, c));
        }
    }
    let mut total = 0;
    for (r, c) in &all_corners {
        let config = offsets
            .iter()
            .map(|(dr, dc)| (r + dr, c + dc))
            .map(|p| area.contains(&p))
            .collect::<Vec<_>>();
        let count = config.iter().filter(|x| **x).count();
        match count {
            1 => total += 1,
            2 if config == vec![true, false, true, false]
                || config == vec![false, true, false, true] =>
            {
                total += 2
            }
            3 => total += 1,
            _ => (),
        }
    }
    total
}

fn find_perimeter(area: &HashSet<Position>) -> usize {
    let mut perimeter = 0;
    for pos in area {
        // let Position { y: row, x: col } = *pos;
        perimeter += 4 - Direction::all()
            .iter()
            .filter_map(|dir| pos.safe_moved_by(dir))
            .filter(|p| area.contains(p))
            .count();
    }
    perimeter
}

fn find_area(
    region: &[Vec<char>],
    from: Position,
    visited: &mut HashSet<Position>,
) -> HashSet<Position> {
    let mut area = HashSet::new();
    let plot_id = region[from.y][from.x];
    let mut queue = vec![from];
    let rows = region.len();
    while let Some(pos) = queue.pop() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        area.insert(pos);
        let cols = region[pos.y].len();
        for adj in Direction::all()
            .iter()
            .filter_map(|dir| pos.safe_moved_by(dir))
            .filter(|p| p.y < rows && p.x < cols)
        {
            if region[adj.y][adj.x] != plot_id || visited.contains(&adj) {
                continue;
            }
            queue.push(adj);
        }
    }
    area
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_12_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2024_12_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "1415378");
        Ok(())
    }

    #[test]
    fn aoc2024_12_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "862714");
        Ok(())
    }

    #[test]
    fn aoc2024_12_case_1() {
        let puzzle = make_test_solution();
        assert_eq!(puzzle.part_one(), "1930");
    }

    #[test]
    fn aoc2024_12_case_2() {
        let puzzle = make_test_solution();
        assert_eq!(puzzle.part_two(), "1206");
    }

    fn make_test_solution() -> AoC2024_12 {
        let lines = [
            "RRRRIICCFF",
            "RRRRIICCCF",
            "VVRRRCCFFF",
            "VVRCCCJFFF",
            "VVVVCJJCFE",
            "VVIVCCJJEE",
            "VVIIICJJEE",
            "MIIIIIJJEE",
            "MIIISIJEEE",
            "MMMISSJEEE",
        ];
        AoC2024_12::with_lines(&lines)
    }

    fn make_solution() -> io::Result<AoC2024_12> {
        AoC2024_12::new()
    }
}
