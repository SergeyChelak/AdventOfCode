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
        calculate_price(&self.input).to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "2024/Day 12: Garden Groups".to_string()
    }
}

type Position = Position2<usize>;

struct PlotTraits {
    area: usize,
    perimeter: usize,
}

fn calculate_price(region: &[Vec<char>]) -> usize {
    let mut visited = HashSet::new();
    let mut total = 0;
    for (row, arr) in region.iter().enumerate() {
        for (col, plot) in arr.iter().enumerate() {
            let position = Position::new(row, col);
            if visited.contains(&position) {
                continue;
            }
            let PlotTraits { area, perimeter } = calc_plot_traits(region, position, &mut visited);
            // println!("Plot {plot}: area {area} * perimeter {perimeter}");
            total += area * perimeter;
        }
    }
    total
}

fn calc_plot_traits(
    region: &[Vec<char>],
    position: Position,
    visited: &mut HashSet<Position>,
) -> PlotTraits {
    let mut area = 0;
    let mut perimeter = 0;
    let plot_id = region[position.row][position.col];
    let mut cells = vec![position];
    let rows = region.len();
    while let Some(p) = cells.pop() {
        if visited.contains(&p) {
            continue;
        }
        area += 1;
        visited.insert(p);
        let Position { row, col } = p;
        let cols = region[row].len();
        if row == 0 || row == rows - 1 {
            perimeter += 1;
        }
        if col == 0 || col == cols - 1 {
            perimeter += 1;
        }
        let adjacent = Direction::all()
            .iter()
            .map(|dir| match dir {
                Direction::Down if row < rows - 1 => Position::new(row + 1, col),
                Direction::Up if row > 0 => Position::new(row - 1, col),
                Direction::Left if col > 0 => Position::new(row, col - 1),
                Direction::Right if col < cols - 1 => Position::new(row, col + 1),
                _ => p,
            })
            .collect::<Vec<_>>();

        perimeter += adjacent
            .iter()
            .filter(|p| region[p.row][p.col] != plot_id)
            .count();

        adjacent
            .iter()
            .filter(|p| region[p.row][p.col] == plot_id)
            .filter(|p| !visited.contains(p))
            .for_each(|p| {
                cells.push(*p);
            });
    }
    PlotTraits { area, perimeter }
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
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2024_12_case_1() {
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
        let puzzle = AoC2024_12::with_lines(&lines);
        assert_eq!(puzzle.part_one(), "1930");
    }

    fn make_solution() -> io::Result<AoC2024_12> {
        AoC2024_12::new()
    }
}
