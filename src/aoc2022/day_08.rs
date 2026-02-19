use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = u32;
type Point = Point2d<usize>;

pub struct AoC2022_08 {
    input: Vec2<Int>,
}

impl AoC2022_08 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2022_08")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(|x| {
                x.chars()
                    .map(|ch| ch.to_digit(10).expect("only digits allowed in the input"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2022_08 {
    fn part_one(&self) -> String {
        let mut total = 0;
        for (y, row) in self.input.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                let point = Point::new(x, y);
                if is_visible(&self.input, &point) {
                    total += 1;
                }
            }
        }
        total.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 8: Treetop Tree House".to_string()
    }
}

fn is_visible(map: &Vec2<Int>, point: &Point) -> bool {
    // edge check
    if point.x == 0 || point.y == 0 || point.y == map.len() - 1 || point.x == map[point.y].len() - 1
    {
        return true;
    }

    let rows = map.len();
    let cols = map[point.y].len();

    let current = map[point.y][point.x];

    for dir in Direction::all() {
        let mut p = *point;
        let mut is_lower = true;
        while let Some(next) = p.safe_moved_by(&dir) {
            if next.y >= rows || next.x >= cols {
                break;
            }
            let val = map[next.y][next.x];
            is_lower = val < current;
            if !is_lower {
                break;
            }
            p = next;
        }
        if is_lower {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_08_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2022_08_case_1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "21");
    }

    #[test]
    fn aoc2022_08_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "1840");
        Ok(())
    }

    #[test]
    fn aoc2022_08_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2022_08> {
        AoC2022_08::new()
    }

    fn make_test_solution() -> AoC2022_08 {
        let input = ["30373", "25512", "65332", "33549", "35390"];
        AoC2022_08::parse_lines(&input)
    }
}
