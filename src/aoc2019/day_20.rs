use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
use std::io;

const PORTAL_IN: &str = "AA";
const PORTAL_OUT: &str = "ZZ";

type Int = usize;
type Point = Point2d<Int>;
type PortalMap = HashMap<String, Point>;

enum MazeElement {
    Open,
    Portal(String),
}

struct Maze {
    points: HashMap<Point, MazeElement>,
    inner: PortalMap,
    outer: PortalMap,
}

impl Maze {
    fn parse<T: AsRef<str>>(lines: &[T]) -> Self {
        let matrix = lines
            .iter()
            .map(|val| val.as_ref())
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let rows = matrix.len();
        let cols = matrix[rows - 1].len();
        let is_inner = |point: &Point| -> bool {
            let offset = 3;
            (offset..rows - offset).contains(&point.y) && (offset..cols - offset).contains(&point.x)
        };
        let mut points = HashMap::new();
        let mut inner = PortalMap::new();
        let mut outer = PortalMap::new();
        for (y, row) in matrix.iter().enumerate() {
            for (x, ch) in row.iter().enumerate() {
                if *ch != '.' {
                    continue;
                }
                let point = Point::new(x, y);
                // check for portal name
                let portal_name = Direction::all()
                    .iter()
                    .filter_map(|dir| {
                        let pos_a = point.moved_by(dir);
                        let pos_b = pos_a.moved_by(dir);

                        let val = match dir {
                            Direction::Up | Direction::Left => [pos_b, pos_a],
                            _ => [pos_a, pos_b],
                        };
                        let val = val.iter().map(|p| matrix[p.y][p.x]).collect::<Vec<_>>();
                        if val.iter().all(|c| c.is_ascii_uppercase()) {
                            return Some(val.iter().collect::<String>());
                        }
                        None
                    })
                    .last();
                let Some(portal_name) = portal_name else {
                    points.insert(point, MazeElement::Open);
                    continue;
                };
                let map = if is_inner(&point) {
                    &mut inner
                } else {
                    &mut outer
                };
                map.insert(portal_name.clone(), point);
                points.insert(point, MazeElement::Portal(portal_name));
            }
        }
        Self {
            points,
            inner,
            outer,
        }
    }

    fn shortest_distance_portals(&self, start: &str, target: &str) -> Option<usize> {
        let start = self.outer.get(start)?;
        let target = self.outer.get(target)?;
        self.shortest_distance(*start, *target)
    }

    fn shortest_distance(&self, start: Point, target: Point) -> Option<usize> {
        let mut steps = 0;
        let mut points = vec![start];
        let mut seen = HashSet::new();

        while !points.is_empty() {
            steps += 1;
            let mut next = HashSet::new();
            for point in points.iter() {
                // println!("{:?} @ {steps}", point);
                if seen.contains(point) {
                    continue;
                }
                seen.insert(*point);

                let mut portal_point: Vec<Point> = Vec::new();
                if let Some(portal) = self.points.get(point).and_then(|val| {
                    let MazeElement::Portal(name) = val else {
                        return None;
                    };
                    Some(name)
                }) {
                    // println!("Found portal {}", portal);
                    let inner = self.inner.get(portal);
                    let outer = self.outer.get(portal);

                    if inner == Some(point) && outer.is_some() {
                        portal_point.push(*outer.unwrap());
                    }
                    if outer == Some(point) && inner.is_some() {
                        portal_point.push(*inner.unwrap());
                    }
                }

                for p in Direction::all()
                    .iter()
                    .map(|dir| point.moved_by(dir))
                    .chain(portal_point.into_iter())
                    .filter(|p| self.points.contains_key(p))
                    .filter(|p| !seen.contains(p))
                {
                    if p == target {
                        return Some(steps);
                    }
                    next.insert(p);
                }
            }
            points = next.into_iter().collect::<Vec<_>>();
        }
        None
    }
}

pub struct AoC2019_20 {
    maze: Maze,
}

impl AoC2019_20 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2019_20")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        Self {
            maze: Maze::parse(lines),
        }
    }
}

impl Solution for AoC2019_20 {
    fn part_one(&self) -> String {
        self.maze
            .shortest_distance_portals(PORTAL_IN, PORTAL_OUT)
            .expect("Not found")
            .to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 20: Donut Maze".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_20_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.maze.points.is_empty());
        assert!(!sol.maze.inner.is_empty());
        assert!(!sol.maze.outer.is_empty());

        assert!(sol.maze.outer.contains_key(PORTAL_IN));
        assert!(sol.maze.outer.contains_key(PORTAL_OUT));
        Ok(())
    }

    #[test]
    fn aoc2019_20_parser_test() {
        let input = [
            "         A           ",
            "         A           ",
            "  #######.#########  ",
            "  #######.........#  ",
            "  #######.#######.#  ",
            "  #######.#######.#  ",
            "  #######.#######.#  ",
            "  #####  B    ###.#  ",
            "BC...##  C    ###.#  ",
            "  ##.##     EE.##.#  ",
            "  ##...DE  F  ###..KK",
            "  #####    G  ###.#  ",
            "  #########.#####.#  ",
            "DE..#######...###.#  ",
            "  #.#########.###.#  ",
            "FG..#########.....#  ",
            "  ###########.#####  ",
            "             Z       ",
            "             Z       ",
        ];
        let maze = Maze::parse(&input);
        assert!(!maze.points.is_empty());
        assert!(!maze.inner.is_empty());
        assert!(!maze.outer.is_empty());

        assert!(maze.outer.contains_key("AA"));
        assert_eq!(maze.outer.get("AA"), Some(&Point::new(9, 2)));

        assert!(maze.outer.contains_key("ZZ"));
        assert_eq!(maze.outer.get("ZZ"), Some(&Point::new(13, 16)));

        assert!(maze.outer.contains_key("KK"));

        assert!(!maze.inner.contains_key("AA"));
        assert!(!maze.inner.contains_key("ZZ"));
        assert!(maze.inner.contains_key("EE"));
    }

    #[test]
    fn aoc2019_20_case_1() {
        let input = [
            "         A           ",
            "         A           ",
            "  #######.#########  ",
            "  #######.........#  ",
            "  #######.#######.#  ",
            "  #######.#######.#  ",
            "  #######.#######.#  ",
            "  #####  B    ###.#  ",
            "BC...##  C    ###.#  ",
            "  ##.##       ###.#  ",
            "  ##...DE  F  ###.#  ",
            "  #####    G  ###.#  ",
            "  #########.#####.#  ",
            "DE..#######...###.#  ",
            "  #.#########.###.#  ",
            "FG..#########.....#  ",
            "  ###########.#####  ",
            "             Z       ",
            "             Z       ",
        ];
        let maze = Maze::parse(&input);
        assert_eq!(
            maze.shortest_distance_portals(PORTAL_IN, PORTAL_OUT),
            Some(23)
        );
    }

    #[test]
    fn aoc2019_20_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "608");
        Ok(())
    }

    #[test]
    fn aoc2019_20_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_20> {
        AoC2019_20::new()
    }
}
