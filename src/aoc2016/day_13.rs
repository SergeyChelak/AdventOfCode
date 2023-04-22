use crate::solution::Solution;

use std::{collections::HashSet, io};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Position(u32, u32);

impl Position {
    fn is_equal(&self, other: &Option<Position>) -> bool {
        if let Some(pos) = other {
            self == pos
        } else {
            false
        }
    }
}

struct Maze {
    fav_number: u32,
}

impl Maze {
    fn with_fav_number(fav_number: u32) -> Self {
        Self { fav_number }
    }

    fn is_open(&self, pos: &Position) -> bool {
        let (x, y) = (pos.0, pos.1);
        let val = x * x + 3 * x + 2 * x * y + y + y * y + self.fav_number;
        let mut mask = 1u32;
        let mut bits = 0;
        while mask > 0 {
            if val & mask > 0 {
                bits += 1;
            }
            mask <<= 1;
        }
        bits % 2 == 0
    }

    fn min_steps(&self, x: u32, y: u32) -> Option<u32> {
        self.search(Some(Position(x, y)), None).map(|v| v.0)
    }

    fn distinct_locations(&self, steps: u32) -> Option<u32> {
        self.search(None, Some(steps)).map(|v| v.1)
    }

    fn search(
        &self,
        target_pos: Option<Position>,
        target_steps: Option<u32>,
    ) -> Option<(u32, u32)> {
        let mut positions = Vec::new();
        let mut visited = HashSet::new();
        {
            let pos = Position(1, 1);
            positions.push(pos);
            visited.insert(pos);
        }
        let mut steps = 0;
        let mut locations = 1;
        while !positions.is_empty() {
            if let Some(val) = target_steps {
                if val == steps {
                    return Some((steps, locations));
                }
            }
            let mut next_positions = Vec::with_capacity(4 * positions.len());
            for pos in positions {
                if pos.is_equal(&target_pos) {
                    return Some((steps, locations));
                } else {
                    let (i, j) = (pos.0, pos.1);
                    let mut adj = Vec::with_capacity(4);
                    if i > 0 {
                        adj.push(Position(i - 1, j));
                    }
                    if j > 0 {
                        adj.push(Position(i, j - 1));
                    }
                    adj.push(Position(i + 1, j));
                    adj.push(Position(i, j + 1));
                    for p in adj.iter() {
                        if !visited.contains(p) && self.is_open(p) {
                            visited.insert(*p);
                            next_positions.push(*p);
                            locations += 1;
                        }
                    }
                }
            }
            steps += 1;
            positions = next_positions;
        }
        None
    }
}

pub struct AoC2016_13 {
    input: u32,
}

impl AoC2016_13 {
    pub fn new() -> io::Result<Self> {
        Ok(Self { input: 1350 })
    }
}

impl Solution for AoC2016_13 {
    fn part_one(&self) -> String {
        Maze::with_fav_number(self.input)
            .min_steps(31, 39)
            .expect("Position isn't reachable")
            .to_string()
    }

    fn part_two(&self) -> String {
        Maze::with_fav_number(self.input)
            .distinct_locations(50)
            .expect("Can't reachable specified amount of locations")
            .to_string()
    }

    fn description(&self) -> String {
        "AoC 2016/Day 13: A Maze of Twisty Little Cubicles".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_13_correctness() -> io::Result<()> {
        let sol = AoC2016_13::new()?;
        assert_eq!(sol.part_one(), "92");
        assert_eq!(sol.part_two(), "124");
        Ok(())
    }

    #[test]
    fn aoc2016_13_demo() {
        let value = Maze::with_fav_number(10)
            .min_steps(7, 4)
            .expect("should be found");
        assert_eq!(value, 11);
    }
}
