use crate::{
    solution::Solution,
    utils::{Direction, Point2d},
};

use std::{collections::HashMap, io};

type Int = i32;
type Point = Point2d<Int>;
type Map = HashMap<Point, char>;

enum RouteElement {
    Move(usize),
    Left,
    Right,
}
type Route = Vec<RouteElement>;
type MapRange = HashMap<Int, (Int, Int)>;

struct MapData {
    map: Map,
    col_range: MapRange,
    row_range: MapRange,
    start: Point,
}

const WALL: char = '#';
const TILE: char = '.';

pub struct AoC2022_22 {
    map_data: MapData,
    route: Route,
}

impl AoC2022_22 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2022_22")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        let (map, route) = data.split_once("\n\n").expect("Invalid input format");
        let map_data = MapData::from(map);
        let route = parse_route(route);
        Self { map_data, route }
    }
}

impl Solution for AoC2022_22 {
    fn part_one(&self) -> String {
        simulate_movement(&self.map_data, &self.route, |point, dir| {
            let data = &self.map_data;
            let mut next = point.moved_by(dir);
            if !data.map.contains_key(&next) {
                let (min_x, max_x) = data.col_range.get(&point.y).expect("x range is missing");
                let (min_y, max_y) = data.row_range.get(&point.x).expect("y range is missing");
                match dir {
                    Direction::Up => next.y = *max_y,
                    Direction::Down => next.y = *min_y,
                    Direction::Left => next.x = *max_x,
                    Direction::Right => next.x = *min_x,
                }
            }
            (next, *dir)
        })
        .to_string()
    }

    fn part_two(&self) -> String {
        simulate_movement(&self.map_data, &self.route, |point, dir| {
            let mut next = point.moved_by(dir);
            let mut next_dir = *dir;

            if !self.map_data.map.contains_key(&next) {
                let x = point.x;
                let y = point.y;
                match dir {
                    Direction::Right => {
                        if y < 50 {
                            // Face 2 -> Face 5 (Upside down)
                            next = Point::new(99, 149 - y);
                            next_dir = Direction::Left;
                        } else if y < 100 {
                            // Face 3 -> Face 2
                            next = Point::new(100 + (y - 50), 49);
                            next_dir = Direction::Up;
                        } else if y < 150 {
                            // Face 5 -> Face 2 (Upside down)
                            next = Point::new(149, 149 - y);
                            next_dir = Direction::Left;
                        } else {
                            // Face 6 -> Face 5
                            next = Point::new(50 + (y - 150), 149);
                            next_dir = Direction::Up;
                        }
                    }
                    Direction::Left => {
                        if y < 50 {
                            // Face 1 -> Face 4 (Upside down)
                            next = Point::new(0, 149 - y);
                            next_dir = Direction::Right;
                        } else if y < 100 {
                            // Face 3 -> Face 4
                            next = Point::new(y - 50, 100);
                            next_dir = Direction::Down;
                        } else if y < 150 {
                            // Face 4 -> Face 1 (Upside down)
                            next = Point::new(50, 149 - y);
                            next_dir = Direction::Right;
                        } else {
                            // Face 6 -> Face 1
                            next = Point::new(50 + (y - 150), 0);
                            next_dir = Direction::Down;
                        }
                    }
                    Direction::Up => {
                        if x < 50 {
                            // Face 4 -> Face 3
                            next = Point::new(50, 50 + x);
                            next_dir = Direction::Right;
                        } else if x < 100 {
                            // Face 1 -> Face 6
                            next = Point::new(0, 150 + (x - 50));
                            next_dir = Direction::Right;
                        } else {
                            // Face 2 -> Face 6
                            next = Point::new(x - 100, 199);
                            next_dir = Direction::Up;
                        }
                    }
                    Direction::Down => {
                        if x < 50 {
                            // Face 6 -> Face 2
                            next = Point::new(100 + x, 0);
                            next_dir = Direction::Down;
                        } else if x < 100 {
                            // Face 5 -> Face 6
                            next = Point::new(49, 150 + (x - 50));
                            next_dir = Direction::Left;
                        } else {
                            // Face 2 -> Face 3
                            next = Point::new(99, 50 + (x - 100));
                            next_dir = Direction::Left;
                        }
                    }
                }
            }
            (next, next_dir)
        })
        .to_string()
    }

    fn description(&self) -> String {
        "Day 22: Monkey Map".to_string()
    }
}

fn simulate_movement(
    data: &MapData,
    route: &Route,
    movement: impl Fn(&Point, &Direction) -> (Point, Direction),
) -> Int {
    let mut point = data.start;
    let mut dir = Direction::Right;

    for elem in route {
        match elem {
            RouteElement::Left => dir = dir.turn_left(),
            RouteElement::Right => dir = dir.turn_right(),
            RouteElement::Move(times) => {
                for _ in 0..*times {
                    let (next, next_dir) = movement(&point, &dir);
                    assert!(data.map.contains_key(&next));
                    if Some(&WALL) == data.map.get(&next) {
                        break;
                    }
                    point = next;
                    dir = next_dir;
                }
            }
        }
    }
    password(&point, &dir)
}

fn password(point: &Point, dir: &Direction) -> Int {
    let dir_val = match dir {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    };
    (1 + point.y) * 1000 + (1 + point.x) * 4 + dir_val
}

impl From<&str> for MapData {
    fn from(value: &str) -> Self {
        let mut start: Option<Point> = None;
        let mut map = Map::new();
        let mut col_range = MapRange::new();
        let mut row_range = MapRange::new();
        let allowed = [WALL, TILE];
        for (row, s) in value.split("\n").enumerate() {
            for (col, ch) in s.chars().enumerate() {
                if !allowed.contains(&ch) {
                    continue;
                }

                let point = Point::new(col as Int, row as Int);
                if ch == TILE && start.is_none() {
                    start = Some(point)
                }
                map.insert(point, ch);

                {
                    let entry = col_range.entry(point.y).or_insert((point.x, point.x));
                    entry.0 = entry.0.min(point.x);
                    entry.1 = entry.1.max(point.x);
                }

                {
                    let entry = row_range.entry(point.x).or_insert((point.y, point.y));
                    entry.0 = entry.0.min(point.y);
                    entry.1 = entry.1.max(point.y);
                }
            }
        }
        Self {
            map,
            col_range,
            row_range,
            start: start.expect("Start point not found"),
        }
    }
}

fn parse_route(input: &str) -> Route {
    let mut output = Vec::new();
    let mut acc = String::new();

    let mut iter = input.trim().chars().peekable();

    while let Some(ch) = iter.next() {
        if ch == 'L' {
            output.push(RouteElement::Left);
            continue;
        }
        if ch == 'R' {
            output.push(RouteElement::Right);
            continue;
        }

        assert!(ch.is_ascii_digit());
        acc.push(ch);

        if iter.peek().map(|x| x.is_ascii_digit()).unwrap_or(false) {
            continue;
        }
        let value = acc.parse::<usize>().expect("Invalid path value");
        acc.clear();
        output.push(RouteElement::Move(value));
    }

    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_22_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.map_data.map.is_empty());
        assert!(!sol.map_data.row_range.is_empty());
        assert!(!sol.map_data.col_range.is_empty());
        assert!(!sol.route.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2022_22_case_1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "6032");
    }

    #[test]
    fn aoc2022_22_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "56372");
        Ok(())
    }

    #[test]
    fn aoc2022_22_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "197047");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2022_22> {
        AoC2022_22::new()
    }

    fn make_test_solution() -> AoC2022_22 {
        let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";
        AoC2022_22::parse_data(input)
    }
}
