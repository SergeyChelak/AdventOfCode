use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

const MAP_ASTEROID: char = '#';

type Int = isize;
type Point = Point2d<Int>;

pub struct AoC2019_10 {
    points: HashSet<Point>,
    max_x: Int,
    max_y: Int,
}

impl AoC2019_10 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2019_10")?;
        Ok(Self::from_lines(&lines))
    }

    fn from_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let max_y = lines.len() - 1;
        let mut max_x = 0;
        let mut points: HashSet<Point> = HashSet::new();
        for (row, line) in lines.iter().map(|x| x.as_ref()).enumerate() {
            for (col, ch) in line.chars().enumerate() {
                max_x = max_x.max(col);
                if ch != MAP_ASTEROID {
                    continue;
                }
                let point = Point::new(col as Int, row as Int);
                points.insert(point);
            }
        }
        Self {
            points,
            max_x: max_x as Int,
            max_y: max_y as Int,
        }
    }
}

impl Solution for AoC2019_10 {
    fn part_one(&self) -> String {
        let in_range = |p: &Point| -> bool {
            if p.x < 0 || p.y < 0 || p.x > self.max_x || p.y > self.max_y {
                return false;
            }
            true
        };
        let mut result = 0;
        for first in self.points.iter() {
            let mut count = 0;
            let mut visited = HashSet::<Point>::new();
            for second in self.points.iter() {
                if visited.contains(second) {
                    continue;
                }
                let dx = second.x - first.x;
                let dy = second.y - first.y;
                if dx == 0 && dy == 0 {
                    continue;
                }
                let divider = gcd(dx.abs_diff(0), dy.abs_diff(0)) as Int;
                let dx = dx / divider;
                let dy = dy / divider;
                let mut ray = *first;
                loop {
                    ray.x += dx;
                    ray.y += dy;
                    if !in_range(&ray) {
                        break;
                    }
                    visited.insert(ray);
                }
                count += 1;
            }
            result = result.max(count);
        }
        result.to_string()
    }

    // fn part_two(&self) -> String {
    // }

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
        assert!(sol.max_x > 0);
        assert!(sol.max_y > 0);
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
        let puzzle = AoC2019_10::from_lines(&lines);
        assert_eq!(puzzle.part_one(), "210")
    }

    #[test]
    fn aoc2019_10_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_10> {
        AoC2019_10::new()
    }
}
