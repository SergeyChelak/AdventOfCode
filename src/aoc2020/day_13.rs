use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = isize;

struct Element {
    id: Int,
    order: Int,
}

pub struct AoC2020_13 {
    estimate: Int,
    elements: Vec<Element>,
}

impl AoC2020_13 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2020_13")?;
        Ok(Self::parse(&lines))
    }

    fn parse<T: AsRef<str>>(lines: &[T]) -> Self {
        assert_eq!(lines.len(), 2);
        let estimate = lines[0]
            .as_ref()
            .parse::<Int>()
            .expect("Estimation must be integer");

        let elements = lines[1]
            .as_ref()
            .split(',')
            .enumerate()
            .filter_map(|(order, s)| {
                let id = s.parse::<Int>().ok()?;
                Some(Element {
                    id,
                    order: order as Int,
                })
            })
            .collect::<Vec<_>>();

        Self { estimate, elements }
    }
}

impl Solution for AoC2020_13 {
    fn part_one(&self) -> String {
        self.elements
            .iter()
            .map(|elem| elem.id)
            .map(|id| (id, id - self.estimate % id))
            .min_by_key(|a| a.1)
            .map(|(id, time)| (id * time).to_string())
            .unwrap_or(not_found())
    }

    fn part_two(&self) -> String {
        let pairs = self
            .elements
            .iter()
            .map(|elem| (elem.id, elem.id - elem.order))
            .collect::<Vec<_>>();
        crt(&pairs).to_string()
    }

    fn description(&self) -> String {
        "Day 13: Shuttle Search".to_string()
    }
}

fn egcd(a: Int, b: Int) -> (Int, Int, Int) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: Int, n: Int) -> Int {
    let (_, x, _) = egcd(x, n);
    (x % n + n) % n
}

fn crt(pairs: &[(Int, Int)]) -> Int {
    let product = pairs.iter().map(|(r, _)| r).product::<Int>();
    pairs.iter().fold(0, |acc, (i, j)| {
        let p = product / i;
        acc + j * mod_inv(p, *i) * p
    }) % product
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_13_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.elements.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2020_13_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "203");
        Ok(())
    }

    #[test]
    fn aoc2020_13_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "905694340256752");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_13> {
        AoC2020_13::new()
    }

    #[test]
    fn aoc2020_13_case1() {
        let sol = AoC2020_13::parse(&["939", "7,13,x,x,59,x,31,19"]);
        assert_eq!(sol.part_one(), "295")
    }

    #[test]
    fn aoc2020_13_case2() {
        let samples = [
            ("17,x,13,19", "3417"),
            ("67,7,59,61", "754018"),
            ("67,x,7,59,61", "779210"),
            ("67,7,x,59,61", "1261476"),
            ("1789,37,47,1889", "1202161486"),
        ];
        for (input, expected) in samples {
            let sol = AoC2020_13::parse(&["939", input]);
            assert_eq!(sol.part_two(), expected)
        }
    }
}
