use crate::solution::Solution;
use crate::utils::cartesian::RepeativeCartesianIter;
use crate::utils::hyper_point::HyperPoint;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

type Int = isize;
type Point = HyperPoint<Int>;
type Store = HashSet<Point>;

impl Point {
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

fn simulate(deltas: &[Point], store: &Store, new_store: &mut Store) {
    let mut seen = HashSet::new();
    let mut interested = store.clone();

    while !interested.is_empty() {
        let any = interested.iter().next().cloned().expect("Unreachable");
        interested.remove(&any);
        seen.insert(any.clone());

        let is_active = store.contains(&any);
        let mut count = 0usize;

        for adj in any.adjacent(deltas) {
            if store.contains(&adj) {
                count += 1;
            }
            if is_active && !seen.contains(&adj) {
                interested.insert(adj);
            }
        }
        match (is_active, count) {
            (true, 2) | (true, 3) | (false, 3) => {
                new_store.insert(any);
            }
            _ => {
                // no op
            }
        }
    }
}

pub struct AoC2020_17 {
    input: Store,
}

impl AoC2020_17 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2020_17")?;
        Ok(Self::parse(&lines))
    }

    fn parse<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut input = HashSet::new();
        for (row, s) in lines.iter().map(|x| x.as_ref()).enumerate() {
            for (col, ch) in s.chars().enumerate() {
                let point = Point::from(vec![col as isize, row as isize]);
                if ch == '#' {
                    input.insert(point);
                }
            }
        }
        Self { input }
    }

    fn simulate(&self, dimension: usize) -> String {
        assert!(dimension > 2);
        let mut store = self.expand_store(vec![0; dimension - 2]);
        let deltas = adjacent_deltas(dimension);

        for _ in 0..6 {
            let mut new_store = HashSet::new();
            simulate(&deltas, &store, &mut new_store);
            store = new_store;
        }
        store.len().to_string()
    }

    fn expand_store(&self, values: Vec<Int>) -> Store {
        let mut store = self.input.iter().cloned().collect::<Vec<_>>();
        store.iter_mut().for_each(|x| x.expand(values.clone()));
        store.into_iter().collect::<Store>()
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
