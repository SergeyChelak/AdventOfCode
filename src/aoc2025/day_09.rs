use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = isize;
type Point = Point2d<Int>;

pub struct AoC2025_09 {
    input: Vec<Point>,
}

impl AoC2025_09 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2025_09")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|v| v.as_ref())
            .map(|s| Point::parse_csv(s).expect("Invalid input format"))
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2025_09 {
    fn part_one(&self) -> String {
        let mut result = 0;
        for (i, a) in self.input.iter().enumerate() {
            for b in self.input.iter().skip(i + 1) {
                result = result.max((1 + a.x.abs_diff(b.x)) * (1 + a.y.abs_diff(b.y)));
            }
        }
        result.to_string()
    }

    fn part_two(&self) -> String {
        let len = self.input.len();
        let intervals = self
            .input
            .iter()
            .enumerate()
            .map(|(i, cur)| {
                let next = self.input[(i + 1) % len];
                Interval2d::with(cur, &next)
            })
            .collect::<Vec<_>>();

        let mut result = 0;
        for (i, a) in self.input.iter().enumerate() {
            for b in self.input.iter().skip(i + 1) {
                let diagonal = Interval2d::with(a, b);
                let square = diagonal.square();
                if square <= result {
                    continue;
                }
                let mut is_valid = true;
                for segment in intervals.iter() {
                    let Some(intersection) = diagonal.intersection(segment) else {
                        continue;
                    };

                    // edge check
                    if intersection.x_interval.len() == 1 {
                        // if vertical
                        let x = intersection.x_interval.begin;
                        if diagonal.x_interval.is_edge(x) {
                            continue;
                        }
                    }
                    if intersection.y_interval.len() == 1 {
                        // if horizontal
                        let y = intersection.y_interval.begin;
                        if diagonal.y_interval.is_edge(y) {
                            continue;
                        }
                    }
                    is_valid = false;
                    break;
                }
                if is_valid {
                    result = square;
                }
            }
        }
        result.to_string()
    }

    fn description(&self) -> String {
        "Day 9: Movie Theater".to_string()
    }
}

struct Interval2d {
    x_interval: PlainInterval<Int>,
    y_interval: PlainInterval<Int>,
}

impl Interval2d {
    fn with(a: &Point, b: &Point) -> Self {
        Self {
            x_interval: PlainInterval::with_arbitrary(a.x, b.x),
            y_interval: PlainInterval::with_arbitrary(a.y, b.y),
        }
    }

    fn square(&self) -> usize {
        self.x_interval.len() * self.y_interval.len()
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        let x_interval = self.x_interval.intersection(&other.x_interval)?;
        let y_interval = self.y_interval.intersection(&other.y_interval)?;
        Some(Self {
            x_interval,
            y_interval,
        })
    }
}

impl PlainInterval<Int> {
    fn with_arbitrary(a: Int, b: Int) -> Self {
        Self::new(a.min(b), a.max(b))
    }

    fn is_edge(&self, value: Int) -> bool {
        value == self.begin || value == self.end
    }

    fn len(&self) -> usize {
        self.begin.abs_diff(self.end) + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2025_09_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2025_09_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "4737096935");
        Ok(())
    }

    #[test]
    fn aoc2025_09_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "1644094530");
        Ok(())
    }

    #[test]
    fn aoc2025_09_case_2() {
        let sol = make_test_solution();
        assert_eq!(sol.part_two(), "24");
    }

    fn make_solution() -> io::Result<AoC2025_09> {
        AoC2025_09::new()
    }

    fn make_test_solution() -> AoC2025_09 {
        let lines = ["7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3"];
        AoC2025_09::parse_lines(&lines)
    }
}
