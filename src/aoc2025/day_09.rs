use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

type Int = isize;
type Point = Point2d<Int>;

impl Point {
    fn vector(begin: &Point, end: &Point) -> Point {
        Point::new(end.x - begin.x, end.y - begin.y)
    }

    fn cross_product(&self, other: &Point) -> Int {
        self.x * other.y - other.x * self.y
    }
}

fn cross_product(prev: &Point, current: &Point, next: &Point) -> Int {
    let v1 = Point::vector(prev, current);
    let v2 = Point::vector(current, next);
    v1.cross_product(&v2)
}

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
        let count = self.input.len();
        let cross_products = self
            .input
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let i_prev = (count + i - 1) % count;
                let i_next = (i + 1) % count;
                cross_product(&self.input[i_prev], p, &self.input[i_next])
            })
            .collect::<Vec<_>>();

        let concave_indices = cross_products
            .iter()
            .enumerate()
            .filter(|(_, val)| **val < 0)
            .map(|(idx, _)| idx)
            // .inspect(|idx| println!("{:?}", self.input[*idx]))
            .collect::<HashSet<_>>();

        let mut result = 0usize;
        let mut rect = Interval2d::with(&Point::zero(), &Point::zero());
        for (i, a) in self.input.iter().enumerate() {
            'j_loop: for (j, b) in self.input.iter().enumerate().skip(i + 1) {
                let candidate_interval = Interval2d::with(a, b);
                // don't consider worst options
                let square = candidate_interval.square();
                if square <= result {
                    continue;
                }
                // check if there is no concave vertex inside candidate
                for k in concave_indices.iter().copied() {
                    if k == i || k == j {
                        continue;
                    }
                    let v = self.input[k];
                    let v_next = self.input[(k + 1) % count];

                    let interval = Interval2d::with(&v, &v_next);
                    let Some(intersection) = candidate_interval.intersection(&interval) else {
                        continue;
                    };

                    if intersection.is_vertical() {
                        assert_eq!(intersection.x_interval.begin, intersection.x_interval.end);
                        if intersection.y_interval.len() == 1 {
                            continue;
                        }
                    } else {
                        assert_eq!(intersection.y_interval.begin, intersection.y_interval.end);
                        if intersection.x_interval.len() == 1 {
                            continue;
                        }
                    }
                    continue 'j_loop;
                }
                // update if all constraints were satisfied
                result = square;
                rect = candidate_interval;
            }
        }
        debug_print_rect(&rect);
        result.to_string()
    }

    fn description(&self) -> String {
        "Day 9: Movie Theater".to_string()
    }
}

fn debug_print_rect(rect: &Interval2d) {
    let x_int = rect.x_interval;
    let y_int = rect.y_interval;
    println!("{},{}", x_int.begin, y_int.begin,);
    println!("{},{}", x_int.end, y_int.begin,);
    println!("{},{}", x_int.end, y_int.end);
    println!("{},{}", x_int.begin, y_int.end,);
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

    fn is_vertical(&self) -> bool {
        self.x_interval.begin == self.x_interval.end
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
        assert_eq!(sol.part_two(), "");
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
