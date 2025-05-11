use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
use std::io;

type Int = i32;

#[derive(Debug, Clone)]
struct Point3d {
    x: Int,
    y: Int,
    z: Int,
}

impl Point3d {
    fn from_csv(s: &str) -> Self {
        let values = s
            .split(',')
            .map(|x| {
                x.parse::<Int>()
                    .expect("Integer value as coordinate is expected")
            })
            .collect::<Vec<_>>();
        assert_eq!(values.len(), 3, "Incorrect number of coordinates");
        Self {
            x: values[0],
            y: values[1],
            z: values[2],
        }
    }
}

#[derive(Debug, Clone)]
struct Brick(Point3d, Point3d);

impl Brick {
    fn altitude_add(&mut self, value: Int) {
        self.0.z += value;
        self.1.z += value;
    }

    fn lowest_altitude(&self) -> Int {
        let altitude = self.0.z;
        assert!(altitude <= self.1.z);
        altitude
    }

    fn highest_altitude(&self) -> Int {
        let altitude = self.1.z;
        assert!(altitude >= self.0.z);
        altitude
    }

    fn is_touching(&self, other: &Self) -> bool {
        self.lowest_altitude() == 1 + other.highest_altitude()
            && self.x_interval().has_intersection(&other.x_interval())
            && self.y_interval().has_intersection(&other.y_interval())
    }

    fn x_interval(&self) -> PlainInterval<Int> {
        assert!(self.0.x <= self.1.x);
        PlainInterval::new(self.0.x, self.1.x)
    }

    fn y_interval(&self) -> PlainInterval<Int> {
        assert!(self.0.y <= self.1.y);
        PlainInterval::new(self.0.y, self.1.y)
    }
}

#[derive(Default)]
struct Strut {
    top: HashSet<usize>,
    bottom: HashSet<usize>,
}

pub struct AoC2023_22 {
    bricks: Vec<Brick>,
}

impl AoC2023_22 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_22")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let bricks = lines
            .iter()
            .map(|s| s.split_once('~').expect("Delimiter not found"))
            .map(|(a, b)| Brick(Point3d::from_csv(a), Point3d::from_csv(b)))
            .collect::<Vec<_>>();
        Self { bricks }
    }
}

impl Solution for AoC2023_22 {
    fn part_one(&self) -> String {
        let mut bricks = self.bricks.clone();
        bricks.sort_by_key(|a| a.lowest_altitude());

        let mut max_height = 0;
        let mut stacked: Vec<Brick> = Vec::new();
        let mut struts: HashMap<usize, Strut> = HashMap::new();
        for mut brick in bricks.into_iter() {
            let steps = max_height + 1 - brick.lowest_altitude();
            if steps < 0 {
                brick.altitude_add(steps);
            }
            while brick.lowest_altitude() > 0 {
                brick.altitude_add(-1);
                let top_idx = stacked.len();
                let mut touched = false;
                for (bottom_idx, other) in stacked.iter().enumerate() {
                    if !brick.is_touching(other) {
                        continue;
                    }
                    // update upper strut set for bottom_idx
                    struts.entry(bottom_idx).or_default().top.insert(top_idx);
                    // update strut for current brick
                    struts.entry(top_idx).or_default().bottom.insert(bottom_idx);
                    touched = true;
                }
                if touched {
                    break;
                }
            }
            max_height = max_height.max(1 + brick.highest_altitude());
            stacked.push(brick);
        }

        let mut count = 0;
        for idx in 0..stacked.len() {
            let Some(strut) = struts.get(&idx) else {
                count += 1;
                continue;
            };
            if strut.top.is_empty() {
                count += 1;
                continue;
            }
            let can_disintegrate = strut
                .top
                .iter()
                .filter_map(|idx| struts.get(idx))
                .all(|s| s.bottom.len() > 1);
            if can_disintegrate {
                count += 1;
            }
        }
        count.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 22: Sand Slabs".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_22_input_load_test() -> io::Result<()> {
        let sol = AoC2023_22::new()?;
        assert!(!sol.bricks.is_empty());
        assert_eq!(sol.bricks.len(), 1485);
        Ok(())
    }

    #[test]
    fn aoc2023_22_ex1() {
        let input = [
            "1,0,1~1,2,1",
            "0,0,2~2,0,2",
            "0,2,3~2,2,3",
            "0,0,4~0,2,4",
            "2,0,5~2,2,5",
            "0,1,6~2,1,6",
            "1,1,8~1,1,9",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let puzzle = AoC2023_22::with_lines(&input);
        assert_eq!(puzzle.part_one(), "5");
    }

    #[test]
    fn aoc2023_22_correctness() -> io::Result<()> {
        let sol = AoC2023_22::new()?;
        assert_eq!(sol.part_one(), "495");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
