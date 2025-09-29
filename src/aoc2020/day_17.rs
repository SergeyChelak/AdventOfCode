use crate::solution::Solution;
use crate::utils::cartesian::RepeativeCartesianIter;
use crate::utils::*;

use std::collections::HashSet;
use std::io;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct HyperPoint<T>(Vec<T>);

impl<T> HyperPoint<T> {
    fn expand(&mut self, value: T) {
        self.0.push(value);
    }

    fn dimension(&self) -> usize {
        self.0.len()
    }
}

impl<T> From<Vec<T>> for HyperPoint<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<T> HyperPoint<T>
where
    T: Copy,
{
    pub fn binary_operation(&self, operation: impl Fn(T, T) -> T, other: &Self) -> Self {
        assert_eq!(self.dimension(), other.dimension());
        Self(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(a, b)| operation(*a, *b))
                .collect(),
        )
    }
}

impl<T> HyperPoint<T>
where
    T: Copy + Add<Output = T>,
{
    pub fn add(&self, other: &Self) -> Self {
        self.binary_operation(|a, b| a + b, other)
    }
}

impl<T> HyperPoint<T>
where
    T: Copy + Sub<Output = T>,
{
    pub fn subtract(&self, other: &Self) -> Self {
        self.binary_operation(|a, b| a - b, other)
    }
}

type Int = isize;
type Point = HyperPoint<Int>;
type Store = HashSet<Point>;

impl Point {
    fn fill(value: Int, dim: usize) -> Self {
        vec![value; dim].into()
    }

    fn is_zero(&self) -> bool {
        self.0.iter().all(|x| *x == 0)
    }

    fn adjacent(&self, deltas: &[Point]) -> Vec<Point> {
        deltas.iter().map(|hp| self.add(hp)).collect::<Vec<_>>()
    }
}

fn adjacent_deltas(dimension: usize) -> Vec<Point> {
    [-1, 0, 1]
        .cartesian_iter(dimension)
        .map(Point::from)
        .filter(|p| !p.is_zero())
        .collect::<Vec<_>>()
}

pub struct AoC2020_17 {
    input: Store,
    from: Point,
    to: Point,
}

impl AoC2020_17 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2020_17")?;
        Ok(Self::parse(&lines))
    }

    fn parse<T: AsRef<str>>(lines: &[T]) -> Self {
        let from = HyperPoint::fill(0, 2);
        let mut to = from.clone();
        let mut input = HashSet::new();
        for (row, s) in lines.iter().map(|x| x.as_ref()).enumerate() {
            for (col, ch) in s.chars().enumerate() {
                to = Point::from(vec![col as isize, row as isize]);
                if ch == '#' {
                    input.insert(to.clone());
                }
            }
        }
        Self { input, from, to }
    }

    fn simulate(&self, dimension: usize) -> String {
        assert!(dimension == 3 || dimension == 4);

        let is_3d = dimension == 3;
        let (mut store, mut from, mut to) = self.prepare_data(dimension);
        let one = Point::fill(1, dimension);

        let process = |p: Point, deltas: &[Point], current_store: &Store, new_store: &mut Store| {
            let is_active = current_store.contains(&p);
            let adj_count = p
                .adjacent(deltas)
                .iter()
                .filter(|x| current_store.contains(*x))
                .count();
            match (is_active, adj_count) {
                (true, 2) | (true, 3) | (false, 3) => {
                    new_store.insert(p);
                }
                _ => {
                    // no op
                }
            }
        };

        let deltas = adjacent_deltas(dimension);

        for _ in 0..6 {
            let mut new_store = HashSet::new();
            from = from.subtract(&one);
            to = to.add(&one);

            for x in from.0[0]..=to.0[0] {
                for y in from.0[1]..=to.0[1] {
                    for z in from.0[2]..=to.0[2] {
                        if is_3d {
                            let p: Point = vec![x, y, z].into();
                            process(p, &deltas, &store, &mut new_store);
                        } else {
                            for w in from.0[3]..=to.0[3] {
                                let p: Point = vec![x, y, z, w].into();
                                process(p, &deltas, &store, &mut new_store);
                            }
                        }
                    }
                }
            }
            store = new_store;
        }
        store.len().to_string()
    }

    fn prepare_data(&self, dim: usize) -> (Store, Point, Point) {
        assert_eq!(self.from.dimension(), self.to.dimension());
        assert!(self.from.dimension() < dim);
        let mut from = self.from.clone();
        let mut to = self.to.clone();
        let mut store = self.input.iter().cloned().collect::<Vec<_>>();

        while from.dimension() < dim {
            from.expand(0);
            to.expand(0);
            store.iter_mut().for_each(|x| x.expand(0));
        }

        (store.into_iter().collect::<Store>(), from, to)
    }
}

impl Solution for AoC2020_17 {
    fn part_one(&self) -> String {
        self.simulate(3)
    }

    fn part_two(&self) -> String {
        self.simulate(4)
    }

    fn description(&self) -> String {
        "Day 17: Conway Cubes".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_17_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        assert!(!sol.to.is_zero());
        Ok(())
    }

    #[test]
    fn aoc2020_17_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "384");
        Ok(())
    }

    #[test]
    fn aoc2020_17_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "2012");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_17> {
        AoC2020_17::new()
    }

    #[test]
    fn aoc2020_17_case1() {
        let input = [".#.", "..#", "###"];
        let sol = AoC2020_17::parse(&input);
        assert_eq!(sol.part_one(), "112");
    }
}
