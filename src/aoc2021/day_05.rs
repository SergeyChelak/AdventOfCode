use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

type Int = isize;
type Point = Point2d<Int>;

struct Line {
    begin: Point,
    end: Point,
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.begin.x == self.end.x
    }

    fn is_horizontal(&self) -> bool {
        self.begin.y == self.end.y
    }
}

pub struct AoC2021_05 {
    input: Vec<Line>,
}

impl AoC2021_05 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2021_05")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|s| s.as_ref())
            .map(Line::from)
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2021_05 {
    fn part_one(&self) -> String {
        calculate(&self.input, |line| {
            line.is_horizontal() || line.is_vertical()
        })
        .to_string()
    }

    fn part_two(&self) -> String {
        calculate(&self.input, |_| true).to_string()
    }

    fn description(&self) -> String {
        "Day 5: Hydrothermal Venture".to_string()
    }
}

fn calculate(lines: &[Line], crit: impl Fn(&Line) -> bool) -> usize {
    let mut area = HashMap::<Point, usize>::new();
    for line in lines.iter() {
        if crit(line) {
            fill(&mut area, line);
        }
    }
    area.values().filter(|x| **x > 1).count()
}

fn fill(area: &mut HashMap<Point, usize>, line: &Line) {
    let dx = line.end.x - line.begin.x;
    let dy = line.end.y - line.begin.y;
    let steps = dx.abs().max(dy.abs());
    let delta = Point::new(dx.signum(), dy.signum());
    let mut p = line.begin;
    for _ in 0..=steps {
        let entry = area.entry(p).or_default();
        *entry += 1;
        p = p.add(&delta);
    }
}

impl From<&str> for Line {
    fn from(value: &str) -> Self {
        let (begin, end) = value.split_once(" -> ").expect("Invalid input format");
        let begin = Point::parse_csv(begin).expect("Failed to parse begin point");
        let end = Point::parse_csv(end).expect("Failed to parse end point");
        Self { begin, end }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_05_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2021_05_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "4421");
        Ok(())
    }

    #[test]
    fn aoc2021_05_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "18674");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2021_05> {
        AoC2021_05::new()
    }
}
