use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

type Int = isize;
type Point = Point2d<Int>;

#[derive(Debug, Clone)]
enum Element {
    Air,
    Rock,
    Sand,
}

type Map = HashMap<Point, Element>;

pub struct AoC2022_14 {
    input: Vec2<Point>,
}

impl AoC2022_14 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2022_14")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        Self::parse_lines(&data.lines().collect::<Vec<_>>())
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(|s| {
                s.split(" -> ")
                    .map(|p| {
                        let (x, y) = p.split_once(',').expect("invalid point format");
                        let x = x.parse::<Int>().expect("x must be integer");
                        let y = y.parse::<Int>().expect("y must be integer");
                        Point::new(x, y)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2022_14 {
    fn part_one(&self) -> String {
        let mut map = make_map(&self.input);
        simulate(&mut map).to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 14: Regolith Reservoir".to_string()
    }
}

fn simulate(map: &mut Map) -> usize {
    let bounds = bounds(&map.keys().cloned().collect::<Vec<_>>()).expect("Incorrect map");
    let in_range = |p: &Point| -> bool {
        // p.x <= bounds.high.x &&
        p.y <= bounds.high.y

        // (bounds.low.x..=bounds.high.x).contains(&p.x)
        //     && (bounds.low.y..=bounds.high.y).contains(&p.y)
    };

    let is_blocked = |map: &Map, p: &Point| -> bool {
        match map.get(p).unwrap_or(&Element::Air) {
            Element::Air => false,
            _ => true,
        }
    };

    let mut units = 0;
    'outer: loop {
        let mut position = Point::new(500, 0);
        let mut is_in_range = true;

        'movement: loop {
            let down = position.moved_by(&Direction::Down);
            let down_left = down.moved_by(&Direction::Left);
            let down_right = down.moved_by(&Direction::Right);

            for next in [down, down_left, down_right] {
                if !is_in_range {
                    break;
                }
                if !is_blocked(&map, &next) {
                    position = next;
                    is_in_range = in_range(&position);
                    continue 'movement;
                }
            }
            map.insert(position, Element::Sand);

            {
                debug_print(map);
                println!();
            }

            if is_in_range {
                break;
            } else {
                break 'outer;
            }
        }
        units += 1;
    }

    units
}

fn make_map(input: &Vec2<Point>) -> Map {
    let mut map = Map::new();
    for line in input {
        for window in line.windows(2) {
            let (dx, dy, steps) = if window[0].x == window[1].x {
                let steps = window[1].y - window[0].y;
                (0, steps.signum(), steps.abs())
            } else if window[0].y == window[1].y {
                let steps = window[1].x - window[0].x;
                (steps.signum(), 0, steps.abs())
            } else {
                panic!("Unexpected points combination: {window:?}")
            };
            let mut p = window[0];
            for _ in 0..=steps {
                map.insert(p, Element::Rock);
                p.x += dx;
                p.y += dy;
            }
        }
    }
    map
}

fn debug_print(map: &Map) {
    let Some(bounds) = bounds(&map.keys().cloned().collect::<Vec<_>>()) else {
        return;
    };
    for y in bounds.low.y - 1..=bounds.high.y + 1 {
        for x in bounds.low.x - 1..=bounds.high.x + 1 {
            let p = Point::new(x, y);
            let ch = match map.get(&p) {
                None | Some(Element::Air) => '.',
                Some(Element::Rock) => '#',
                Some(Element::Sand) => 'o',
            };
            print!("{ch}");
        }
        println!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_14_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2022_14_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "692");
        Ok(())
    }

    #[test]
    fn aoc2022_14_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2022_14_case_1() {
        let sol = make_test_solution();
        let map = make_map(&sol.input);
        debug_print(&map);
        assert_eq!(sol.part_one(), "24");
    }

    fn make_solution() -> io::Result<AoC2022_14> {
        AoC2022_14::new()
    }

    fn make_test_solution() -> AoC2022_14 {
        let input = [
            "498,4 -> 498,6 -> 496,6",
            "503,4 -> 502,4 -> 502,9 -> 494,9",
        ];
        AoC2022_14::parse_lines(&input)
    }
}
