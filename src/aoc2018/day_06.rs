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
        let bounds = bounds(&self.points).expect("Bounds input is empty");
        let norm_points = normalize_with_bounds(&self.points, &bounds);
        let dim = bounds.high.sub(&bounds.low).add(&Point { x: 1, y: 1 });
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

        let mut squares = vec![0usize; norm.len()];
        for row in matrix.iter() {
            for val in row.iter() {
                if let Cell::Owned(id, _) = val {
                    squares[*id] += 1;
                }
            }
        }

        let infinites = edge_values(&matrix);
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
                let sum = points.iter().fold(0, |acc, p| acc + p.distance(x, y));
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

fn edge_values(matrix: &[Vec<Cell>]) -> HashSet<usize> {
    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }
    let mut infinites: HashSet<usize> = HashSet::new();
    let instructions = [
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Up,
    ];
    let (mut x, mut y) = (0usize, 0usize);
    let mut ptr = 0usize;
    while ptr < instructions.len() {
        match instructions[ptr] {
            Direction::Right => {
                if x == matrix.len() - 1 {
                    ptr += 1;
                    continue;
                }
                x += 1;
            }
            Direction::Left => {
                if x == 0 {
                    ptr += 1;
                    continue;
                }
                x -= 1;
            }
            Direction::Down => {
                if y == matrix[x].len() - 1 {
                    ptr += 1;
                    continue;
                }
                y += 1;
            }
            Direction::Up => {
                if y == 0 {
                    ptr += 1;
                    continue;
                }
                y -= 1;
            }
        }
        if let Cell::Owned(id, _) = matrix[x][y] {
            infinites.insert(id);
        }
    }
    infinites
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
