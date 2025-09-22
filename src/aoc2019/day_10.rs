use crate::solution::Solution;
use crate::utils::*;

use std::cmp::Ordering;
use std::collections::HashSet;
use std::f64::consts::PI;
use std::io;

const MAP_ASTEROID: char = '#';

type Int = isize;
type Point = Point2d<Int>;

pub struct AoC2019_10 {
    points: HashSet<Point>,
}

impl AoC2019_10 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2019_10")?;
        Ok(Self::from_lines(&lines))
    }

    fn from_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let points = lines
            .iter()
            .map(|x| x.as_ref())
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, ch)| *ch == MAP_ASTEROID)
                    .map(|(col, _)| Point::new(col as Int, row as Int))
                    .collect::<HashSet<_>>()
            })
            .collect();
        Self { points }
    }

    fn monitoring_location(&self) -> (Point, usize) {
        let mut result = (Point::zero(), 0);
        for first in self.points.iter() {
            let mut slope = HashSet::<(Int, Int)>::new();
            for second in self.points.iter() {
                let dx = second.x - first.x;
                let dy = second.y - first.y;
                if dx == 0 && dy == 0 {
                    continue;
                }
                let divider = gcd(dx.abs_diff(0), dy.abs_diff(0)) as Int;
                let dx = dx / divider;
                let dy = dy / divider;
                slope.insert((dx, dy));
            }
            let count = slope.len();
            if count > result.1 {
                result.1 = count;
                result.0 = *first;
            }
        }
        result
    }

    fn points_by_angle(&self, center: &Point) -> Vec<(f64, Point)> {
        let dist = |a: &Point, b: &Point| -> usize {
            let dx = a.x.abs_diff(b.x);
            let dy = a.y.abs_diff(b.y);
            dx * dx + dy * dy
        };
        let get_angle = |p: &Point| -> f64 {
            let dx = (p.x - center.x) as f64;
            let dy = (p.y - center.y) as f64;
            dy.atan2(dx)
        };
        let base = Point::new(center.x, center.y - 1);
        let base_angle = get_angle(&base);
        let mut arr = self
            .points
            .iter()
            .filter(|p| *p != center)
            .map(|p| {
                let angle = (2.0 * PI + get_angle(p) - base_angle) % (2.0 * PI);
                (angle, *p)
            })
            .collect::<Vec<_>>();
        arr.sort_by(|a, b| {
            if a.0 < b.0 {
                return Ordering::Less;
            }
            if a.0 > b.0 {
                return Ordering::Greater;
            }
            dist(&base, &a.1).cmp(&dist(&base, &b.1))
        });
        arr
    }
}

impl Solution for AoC2019_10 {
    fn part_one(&self) -> String {
        self.monitoring_location().1.to_string()
    }

    fn part_two(&self) -> String {
        let center = self.monitoring_location().0;
        let ordered_points = self.points_by_angle(&center);
        let count = ordered_points.len();
        let mut vaporized = Vec::<Point>::new();
        let mut visited: HashSet<usize> = HashSet::new();
        'main: while visited.len() < count {
            let mut prev = f64::MIN;
            for (i, (angle, point)) in ordered_points.iter().enumerate() {
                if visited.contains(&i) {
                    continue;
                }
                if *angle > prev {
                    prev = *angle;
                    visited.insert(i);
                    vaporized.push(*point);
                }
                if vaporized.len() == 200 {
                    break 'main;
                }
            }
        }
        vaporized
            .get(199)
            .map(|p| p.x * 100 + p.y)
            .map(|x| x.to_string())
            .unwrap_or(not_found())
    }

    fn description(&self) -> String {
        "Day 10: Monitoring Station".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_10_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.points.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_10_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "260");
        Ok(())
    }

    #[test]
    fn aoc2019_10_case_1() {
        #[rustfmt::skip]
        let lines = [
            ".#..#",
            ".....",
            "#####",
            "....#",
            "...##",

        ];
        let puzzle = AoC2019_10::from_lines(&lines);
        assert_eq!(puzzle.part_one(), "8")
    }

    #[test]
    fn aoc2019_10_case_2() {
        #[rustfmt::skip]
        let lines = [
            "......#.#.",
            "#..#.#....",
            "..#######.",
            ".#.#.###..",
            ".#..#.....",
            "..#....#.#",
            "#..#....#.",
            ".##.#..###",
            "##...#..#.",
            ".#....####",
        ];
        let puzzle = AoC2019_10::from_lines(&lines);
        assert_eq!(puzzle.part_one(), "33")
    }

    #[test]
    fn aoc2019_10_case_3() {
        #[rustfmt::skip]
        let lines = [
            "#.#...#.#.",
            ".###....#.",
            ".#....#...",
            "##.#.#.#.#",
            "....#.#.#.",
            ".##..###.#",
            "..#...##..",
            "..##....##",
            "......#...",
            ".####.###.",
        ];
        let puzzle = AoC2019_10::from_lines(&lines);
        assert_eq!(puzzle.part_one(), "35")
    }

    #[test]
    fn aoc2019_10_case_5() {
        let puzzle = make_large_puzzle();
        assert_eq!(puzzle.part_one(), "210")
    }

    #[test]
    fn aoc2019_10_case_5_2() {
        let puzzle = make_large_puzzle();
        assert_eq!(puzzle.part_two(), "802")
    }

    fn make_large_puzzle() -> AoC2019_10 {
        #[rustfmt::skip]
        let lines = [
            ".#..##.###...#######",
            "##.############..##.",
            ".#.######.########.#",
            ".###.#######.####.#.",
            "#####.##.#.##.###.##",
            "..#####..#.#########",
            "####################",
            "#.####....###.#.#.##",
            "##.#################",
            "#####.##.###..####..",
            "..######..##.#######",
            "####.##.####...##..#",
            ".#####..#.######.###",
            "##...#.##########...",
            "#.##########.#######",
            ".####.#.###.###.#.##",
            "....##.##.###..#####",
            ".#.#.###########.###",
            "#.#.#.#####.####.###",
            "###.##.####.##.#..##",
        ];
        AoC2019_10::from_lines(&lines)
    }

    #[test]
    fn aoc2019_10_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "608");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_10> {
        AoC2019_10::new()
    }
}
