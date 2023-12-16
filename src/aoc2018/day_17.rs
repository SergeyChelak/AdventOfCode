use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
use std::io;

type Int = i32;

type Coordinate = Point2d<Int>;

#[derive(Clone, Copy)]
enum HorizontalDirection {
    Left,
    Right,
}

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

    fn horizontal_move(&self, dir: HorizontalDirection) -> Self {
        match dir {
            HorizontalDirection::Left => self.left(),
            HorizontalDirection::Right => self.right(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Scan {
    Clay,
    StillWater,
    FlowingWater,
    Sand,
}

type GroundMap = HashMap<Coordinate, Scan>;

pub struct AoC2018_17 {
    clay: HashSet<Coordinate>,
    max_x: Int,
    min_x: Int,
    max_y: Int,
}

impl AoC2018_17 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2018_17")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let mut clay = HashSet::new();
        let mut max_y: Int = 0;
        let mut max_x: Int = 0;
        let mut min_x: Int = Int::MAX;
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
                clay.insert(coord);
                max_y = max_y.max(y);
                max_x = max_x.max(x);
                min_x = min_x.min(x);
            }
        }
        Self {
            clay,
            min_x,
            max_x,
            max_y,
        }
    }

    fn start_coord() -> Coordinate {
        Coordinate { x: 500, y: 0 }
    }

    fn make_map(&self) -> GroundMap {
        self.clay.iter().map(|coord| (*coord, Scan::Clay)).collect()
    }

    fn reached(&self, map: &GroundMap) -> usize {
        let mut total = 0;
        for x in self.min_x..=self.max_x {
            for y in 1..=self.max_y {
                let coord = Coordinate { x, y };
                let Some(scan) = map.get(&coord) else {
                    continue;
                };
                total += match scan {
                    Scan::FlowingWater | Scan::StillWater => 1,
                    _ => 0,
                }
            }
        }
        total
    }

    fn dump(&self, map: &GroundMap) {
        for y in 0..=self.max_y {
            for x in self.min_x..=self.max_x {
                let coord = Coordinate { x, y };
                let ch = if let Some(scan) = map.get(&coord) {
                    match scan {
                        Scan::Clay => '#',
                        Scan::FlowingWater => '|',
                        Scan::StillWater => '~',
                        Scan::Sand => '.',
                    }
                } else {
                    '.'
                };
                print!("{ch}")
            }
            println!();
        }
    }
}

impl Solution for AoC2018_17 {
    fn part_one(&self) -> String {
        let mut map = self.make_map();
        let mut flow = Vec::from([Self::start_coord()]);
        while !flow.is_empty() {
            let position = *flow.last().expect("Unreachable (1)");
            // println!(">> {}, {}", position.x, position.y);
            if position.y > self.max_y {
                _ = flow.pop();
                continue;
            }
            let down = position.down();
            match map.get(&down).unwrap_or(&Scan::Sand) {
                Scan::Sand => {
                    flow.push(down);
                    map.insert(down, Scan::FlowingWater);
                }
                Scan::FlowingWater => {
                    _ = flow.pop();
                }
                _ => {
                    let mut edges = 0;
                    let mut new_flows = 0;
                    for dir in [HorizontalDirection::Left, HorizontalDirection::Right] {
                        let result = horizontal_flow(position, dir, &map);
                        match result {
                            FlowResult::FlowWater(tiles) => {
                                tiles.iter().for_each(|t| {
                                    map.insert(*t, Scan::FlowingWater);
                                    if !flow.contains(t) {
                                        new_flows += 1;
                                        flow.push(*t);
                                    }
                                });
                            }
                            FlowResult::StillWater(tiles) => {
                                edges += 1;
                                tiles.iter().for_each(|t| {
                                    map.insert(*t, Scan::StillWater);
                                    // if let Some(idx) = flow.iter().position(|x| *x == *t) {
                                    //     flow.remove(idx);
                                    // }
                                });
                            }
                        }
                    }
                    if edges == 2 {
                        map.insert(position, Scan::StillWater);
                        _ = flow.pop();
                    } else if new_flows == 0 {
                        _ = flow.pop();
                    }
                }
            }
            // self.dump(&map);
            // println!();
        }
        self.dump(&map);

        self.reached(&map).to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 17: Reservoir Research".to_string()
    }
}

enum FlowResult {
    StillWater(Vec<Coordinate>),
    FlowWater(Vec<Coordinate>),
}

fn horizontal_flow(
    position: Coordinate,
    direction: HorizontalDirection,
    map: &GroundMap,
) -> FlowResult {
    let mut cur = position;
    let mut tiles = Vec::new();
    loop {
        let next = cur.horizontal_move(direction);
        let scan = map.get(&next).unwrap_or(&Scan::Sand);
        let mut is_clay = false;
        match scan {
            Scan::StillWater => {
                return FlowResult::StillWater(tiles);
            }
            Scan::FlowingWater => {
                return FlowResult::FlowWater(tiles);
            }
            Scan::Clay => {
                is_clay = true;
            }
            _ => {
                tiles.push(next);
            }
        }
        // can move down?
        let down = next.down();
        let scan = map.get(&down).unwrap_or(&Scan::Sand);
        let can_flow_down = matches!(scan, Scan::Sand | Scan::FlowingWater);

        if can_flow_down {
            tiles.push(down);
            return FlowResult::FlowWater(tiles);
        }

        if is_clay {
            return FlowResult::StillWater(tiles);
        }
        cur = next;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_17_input_load_test() -> io::Result<()> {
        let sol = AoC2018_17::new()?;
        assert!(!sol.clay.is_empty());
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
