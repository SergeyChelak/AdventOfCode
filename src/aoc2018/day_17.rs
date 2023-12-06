use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
use std::io;

type Int = i32;

#[derive(Copy, Clone)]
enum MapElement {
    Clay,
    Water,
}

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
    map: HashMap<Coordinate, MapElement>,
    max_y: Int,
}

impl AoC2018_17 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2018_17")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let mut map = HashMap::new();
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
                map.insert(coord, MapElement::Clay);
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
        let mut map = self.map.clone();
        let mut reached = HashSet::new();
        traverse(&mut map, Self::start_coord(), &mut reached, self.max_y);
        reached.len().to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 17: Reservoir Research".to_string()
    }
}

fn traverse(
    map: &mut HashMap<Coordinate, MapElement>,
    cur: Coordinate,
    reached: &mut HashSet<Coordinate>,
    max_y: Int,
) {
    if cur.y > max_y {
        return;
    }
    let bottom = cur.down();
    for c in &[bottom, cur.left(), cur.right()] {
        if let Some(MapElement::Clay) = map.get(c) {
            reached.insert(*c);
        }
    }
    map.insert(cur, MapElement::Water);

    if map.get(&bottom).is_none() {
        traverse(map, bottom, reached, max_y);
    }

    let mut is_right_deadend = false;
    let mut is_left_deadend = false;
    let mut left = cur;
    let mut right = cur;
    loop {
        left = {
            let tmp = left.left();
            if map.get(&tmp).is_none() {
                tmp
            } else {
                left
            }
        };
        right = {
            let tmp = right.right();
            if map.get(&tmp).is_none() {
                tmp
            } else {
                right
            }
        }
    }

    todo!()
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
        let input = vec![];
        let sol = AoC2018_17::with_lines(&input);
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
