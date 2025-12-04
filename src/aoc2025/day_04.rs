use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2025_04 {
    input: Vec2<char>,
}

impl AoC2025_04 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2025_04")?;
        Ok(Self::parse(&lines))
    }

    fn parse<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec2<_>>();
        Self { input }
    }
}

impl Solution for AoC2025_04 {
    fn part_one(&self) -> String {
        reachable_positions(&self.input).len().to_string()
    }

    fn part_two(&self) -> String {
        let mut diagram = self.input.clone();
        let mut count = 0;
        loop {
            let positions = reachable_positions(&diagram);
            let len = positions.len();
            if len == 0 {
                break;
            }
            count += len;
            for p in positions {
                diagram[p.y][p.x] = DIAGRAM_EMPTY;
            }
        }
        count.to_string()
    }

    fn description(&self) -> String {
        "Day 4: Printing Department".to_string()
    }
}

const DIAGRAM_ROLL: char = '@';
const DIAGRAM_EMPTY: char = '.';

type Position = Point2d<usize>;

fn reachable_positions(diagram: &Vec2<char>) -> Vec<Position> {
    let mut positions = Vec::new();
    let directions = Direction::circular_directions();
    for (row, arr) in diagram.iter().enumerate() {
        for (col, ch) in arr.iter().enumerate() {
            if *ch != DIAGRAM_ROLL {
                continue;
            }
            let p = Point2d::new(col, row);
            let adjacent_count = directions
                .iter()
                .filter_map(|dir| p.safe_moved_with_dirs(dir))
                .filter(|adj| {
                    let Some(val) = diagram.get(adj.y).and_then(|v| v.get(adj.x)) else {
                        return false;
                    };
                    *val == DIAGRAM_ROLL
                })
                .count();
            if adjacent_count < 4 {
                positions.push(p);
            }
        }
    }
    positions
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2025_04_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2025_04_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "1367");
        Ok(())
    }

    #[test]
    fn aoc2025_04_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "9144");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2025_04> {
        AoC2025_04::new()
    }
}
