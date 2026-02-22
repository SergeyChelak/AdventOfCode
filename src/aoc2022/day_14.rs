use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

type Int = isize;
type Point = Point2d<Int>;

type Rocks = HashSet<Point>;

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
        let rock_map = make_map(&self.input);
        let bounds = bounds(&rock_map.iter().cloned().collect::<Vec<_>>()).expect("Incorrect map");
        simulate(|p| rock_map.contains(p), |p| p.y <= bounds.high.y).to_string()
    }

    fn part_two(&self) -> String {
        let rock_map = make_map(&self.input);
        let bounds = bounds(&rock_map.iter().cloned().collect::<Vec<_>>()).expect("Incorrect map");
        simulate(
            |p| p.y == bounds.high.y + 2 || rock_map.contains(p),
            |_| true,
        )
        .to_string()
    }

    fn description(&self) -> String {
        "Day 14: Regolith Reservoir".to_string()
    }
}

fn simulate(is_rock: impl Fn(&Point) -> bool, in_range: impl Fn(&Point) -> bool) -> usize {
    let mut sands = HashSet::<Point>::new();
    let mut units = 0;
    let sand_source = Point::new(500, 0);
    'outer: loop {
        let mut position = sand_source;
        let mut is_in_range = true;

        'movement: loop {
            let down = position.moved_by(&Direction::Down);
            let down_left = down.moved_by(&Direction::Left);
            let down_right = down.moved_by(&Direction::Right);

            for next in [down, down_left, down_right] {
                if !is_in_range {
                    break;
                }
                let is_blocked = is_rock(&next) || sands.contains(&next);
                if !is_blocked {
                    position = next;
                    is_in_range = in_range(&position);
                    continue 'movement;
                }
            }
            if position == sand_source {
                units += 1;
                break 'outer;
            }

            sands.insert(position);

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

fn make_map(input: &Vec2<Point>) -> Rocks {
    let mut map = Rocks::new();
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
                map.insert(p);
                p.x += dx;
                p.y += dy;
            }
        }
    }
    map
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
    fn aoc2022_14_example() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "24");
        assert_eq!(sol.part_two(), "93");
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
