use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashSet, VecDeque};
use std::io;

type Int = i32;

type Coordinate = Point2d<Int>;

impl Coordinate {
    fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            ..*self
        }
    }

    fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            ..*self
        }
    }

    fn down(&self) -> Self {
        Self {
            y: self.y + 1,
            ..*self
        }
    }
}

pub struct AoC2018_17 {
    map: HashSet<Coordinate>,
    max_y: Int,
}

impl AoC2018_17 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2018_17")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let mut map = HashSet::new();
        let mut max_y: Int = 0;
        for line in lines {
            let (part1, part2) = line.split_once(", ").expect("Invalid input string");
            let (axe, val_1) = part1.split_once('=').expect("The '=' is expected (1)");
            let (_, val_2) = part2.split_once('=').expect("The '=' is expected (2)");
            let (range_start, range_end) = val_2.split_once("..").expect("Invalid range format");
            let start = range_start.parse::<Int>().expect("Range start isn't int");
            let end = range_end.parse::<Int>().expect("Range end isn't int");
            let val = val_1.parse::<Int>().expect("First value isn't it");
            for i in start..=end {
                let (x, y) = if axe == "x" { (val, i) } else { (i, val) };
                let coord = Coordinate { x, y };
                map.insert(coord);
                max_y = max_y.max(y);
            }
        }
        Self { map, max_y }
    }

    fn start_coord() -> Coordinate {
        Coordinate { x: 500, y: 0 }
    }
}

impl Solution for AoC2018_17 {
    fn part_one(&self) -> String {
        let mut seen: HashSet<Coordinate> = HashSet::new();

        let mut deque = VecDeque::from([Self::start_coord()]);
        let mut vertical: Vec<Coordinate> = Vec::new();
        while !deque.is_empty() {
            let pos = deque.pop_front().expect("Deque shouldn't be empty");
            // println!("{},{}", pos.x, pos.y);
            if pos.y > self.max_y {
                continue;
            }
            let next = [pos.down(), pos.left(), pos.right()];
            for (i, item) in next.iter().enumerate() {
                let is_acceptable = !self.map.contains(item) && !seen.contains(item);
                if is_acceptable {
                    seen.insert(*item);
                    deque.push_back(*item);
                }
                // don't go further if there is way down
                if is_acceptable && i == 0 {
                    vertical.push(*item);
                    break;
                }
            }
            if deque.is_empty() {
                vertical.pop();
                if let Some(item) = vertical.last() {
                    deque.push_back(*item);
                }
            }
        }
        seen.iter()
            .filter(|p| (1..=self.max_y).contains(&p.y))
            .count()
            .to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 17: Reservoir Research".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_17_input_load_test() -> io::Result<()> {
        let sol = AoC2018_17::new()?;
        assert!(!sol.map.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_17_ex1() {
        let input = [
            "x=495, y=2..7",
            "y=7, x=495..501",
            "x=501, y=3..7",
            "x=498, y=2..4",
            "x=506, y=1..2",
            "x=498, y=10..13",
            "x=504, y=10..13",
            "y=13, x=498..504",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let sol = AoC2018_17::with_lines(&input);
        assert_eq!(sol.max_y, 13);
        assert_eq!(sol.part_one(), "57")
    }

    #[test]
    fn aoc2018_17_correctness() -> io::Result<()> {
        let sol = AoC2018_17::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
