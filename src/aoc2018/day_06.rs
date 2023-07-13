use crate::solution::Solution;
use crate::utils::*;

use std::cmp::Ordering;
use std::collections::HashSet;
use std::io;

type Int = i32;
type UInt = u32;
type Point = Point2d<Int>;

impl Point {
    fn distance(&self, x: Int, y: Int) -> UInt {
        self.x.abs_diff(x) + self.y.abs_diff(y)
    }
}

/// Returns top left and bottom right points
fn boundaries(points: &[Point]) -> (Point, Point) {
    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();
    (Point { x: min_x, y: min_y }, Point { x: max_x, y: max_y })
}

#[derive(Copy, Clone)]
enum Cell {
    Inf(UInt),
    Owned(usize, UInt), // owner, distance
}

pub struct AoC2018_06 {
    points: Vec<Point>,
}

impl AoC2018_06 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2018_06")?;
        Ok(Self::from_lines(&lines))
    }

    fn from_lines(lines: &[String]) -> Self {
        let points = lines
            .iter()
            .map(|s| {
                Point::parse_csv(s).unwrap_or_else(|err| {
                    panic!("Failed to parse coordinate from '{s}' string, error: {err:?}")
                })
            })
            .collect::<Vec<Point>>();
        Self { points }
    }

    fn normalized_input(&self) -> (Vec<Point>, Point) {
        let (a, b) = boundaries(&self.points);
        let norm_points = self
            .points
            .iter()
            .map(|p| p.sub(&a))
            .collect::<Vec<Point>>();
        let dim = b.sub(&a).add(&Point { x: 1, y: 1 });
        (norm_points, dim)
    }
}

impl Solution for AoC2018_06 {
    fn part_one(&self) -> String {
        let (norm, dim) = self.normalized_input();
        let mut matrix = vec![vec![Cell::Inf(UInt::MAX); dim.y as usize]; dim.x as usize];
        norm.iter()
            .enumerate()
            .for_each(|(i, p)| matrix[p.x as usize][p.y as usize] = Cell::Owned(i, 0));

        norm.iter().enumerate().for_each(|(id, p)| {
            for (x, row) in matrix.iter_mut().enumerate() {
                for (y, val) in row.iter_mut().enumerate() {
                    let distance = p.distance(x as Int, y as Int);
                    match val {
                        Cell::Owned(other_id, other_dist) if *other_id != id => {
                            match distance.cmp(other_dist) {
                                Ordering::Equal => *val = Cell::Inf(distance),
                                Ordering::Less => *val = Cell::Owned(id, distance),
                                _ => {}
                            }
                        }
                        Cell::Inf(other_dist) => {
                            if distance < *other_dist {
                                *val = Cell::Owned(id, distance);
                            }
                        }
                        _ => {}
                    }
                }
            }
        });

        let mut infinites: HashSet<usize> = HashSet::new();
        let get_id = |cell: &Cell| -> Option<usize> {
            match cell {
                Cell::Owned(id, _) => Some(*id),
                _ => None,
            }
        };

        for i in 0..matrix.len() {
            if let Some(id) = get_id(&matrix[i][0]) {
                infinites.insert(id);
            }
            if let Some(id) = get_id(&matrix[i][matrix[i].len() - 1]) {
                infinites.insert(id);
            }
        }

        for i in 0..matrix[0].len() {
            if let Some(id) = get_id(&matrix[0][i]) {
                infinites.insert(id);
            }
            if let Some(id) = get_id(&matrix[matrix.len() - 1][i]) {
                infinites.insert(id);
            }
        }

        let mut squares = vec![0usize; norm.len()];
        for x in 0..matrix.len() {
            for y in 0..matrix[x].len() {
                match matrix[x][y] {
                    Cell::Owned(id, _) => squares[id] += 1,
                    _ => continue,
                }
            }
        }
        squares
            .iter()
            .enumerate()
            .filter(|(id, _)| !infinites.contains(id))
            .map(|(_, dist)| *dist)
            .max()
            .unwrap()
            .to_string()
    }

    fn part_two(&self) -> String {
        let (points, dim) = self.normalized_input();
        let max = 10000;
        let mut count = 0;
        for x in 0..=dim.x {
            for y in 0..=dim.y {
                let sum = points
                    .iter()
                    .fold(0, |acc, p| acc + p.distance(x, y));
                if sum < max {
                    count += 1;
                }
            }
        }
        count.to_string()
    }

    fn description(&self) -> String {
        "AoC 2018/Day 6: Chronal Coordinates".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_06_input_load_test() -> io::Result<()> {
        let sol = AoC2018_06::new()?;
        assert!(!sol.points.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_06_example1() {
        let lines = ["1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let sol = AoC2018_06::from_lines(&lines);
        assert_eq!(sol.part_one(), "17");
    }

    #[test]
    fn aoc2018_06_correctness() -> io::Result<()> {
        let sol = AoC2018_06::new()?;
        assert_eq!(sol.part_one(), "4754");
        assert_eq!(sol.part_two(), "42344");
        Ok(())
    }
}
