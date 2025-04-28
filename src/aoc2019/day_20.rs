use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet, VecDeque};
use std::io;

const PORTAL_IN: &str = "AA";
const PORTAL_OUT: &str = "ZZ";

type Int = usize;
type Point = Point2d<Int>;
type PortalMap = HashMap<String, Vec<Point>>;

enum MazeElement {
    Open,
    Portal(String),
}

struct Maze {
    points: HashMap<Point, MazeElement>,
    portals: PortalMap,
    rows: usize,
    cols: usize,
}

impl Maze {
    fn parse<T: AsRef<str>>(lines: &[T]) -> Self {
        let matrix = lines
            .iter()
            .map(|val| val.as_ref())
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut points = HashMap::new();
        let mut portals = PortalMap::new();
        for (y, row) in matrix.iter().enumerate() {
            for (x, ch) in row.iter().enumerate() {
                if *ch != '.' {
                    continue;
                }
                let point = Point::new(x, y);
                let Some(portal_name) = Self::parse_portal_name(&matrix, &point) else {
                    points.insert(point, MazeElement::Open);
                    continue;
                };
                let entry = portals.entry(portal_name.clone()).or_default();
                entry.push(point);
                points.insert(point, MazeElement::Portal(portal_name));
            }
        }
        let rows = matrix.len();
        let cols = matrix[rows - 1].len();
        Self {
            points,
            portals,
            rows,
            cols,
        }
    }

    fn parse_portal_name<T: AsRef<[char]>>(matrix: &[T], point: &Point) -> Option<String> {
        Direction::all()
            .iter()
            .filter_map(|dir| {
                let pos_a = point.moved_by(dir);
                let pos_b = pos_a.moved_by(dir);

                let val = match dir {
                    Direction::Up | Direction::Left => [pos_b, pos_a],
                    _ => [pos_a, pos_b],
                };
                let val = val
                    .iter()
                    .map(|p| (matrix[p.y].as_ref())[p.x])
                    .collect::<Vec<_>>();
                if val.iter().all(|c| c.is_ascii_uppercase()) {
                    return Some(val.iter().collect::<String>());
                }
                None
            })
            .last()
    }

    fn shortest_distance_portals(&self) -> Option<usize> {
        let start = *self.portal_to_position(PORTAL_IN)?;
        let target = *self.portal_to_position(PORTAL_OUT)?;
        let mut steps = 0;
        let mut points = vec![start];
        let mut seen = HashSet::new();
        while !points.is_empty() {
            steps += 1;
            let mut next = HashSet::new();
            for point in points.iter() {
                if seen.contains(point) {
                    continue;
                }
                seen.insert(*point);
                let portals = self.adjacent_portal_points(point).unwrap_or_default();
                for p in Direction::all()
                    .iter()
                    .map(|dir| point.moved_by(dir))
                    .chain(portals.into_iter())
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

    fn adjacent_portal_points(&self, point: &Point) -> Option<Vec<Point>> {
        let elem = self.points.get(point)?;
        let MazeElement::Portal(portal) = elem else {
            return None;
        };
        let result = self
            .portals
            .get(portal)?
            .iter()
            .filter(|p| **p != *point)
            .copied()
            .collect::<Vec<_>>();
        Some(result)
    }

    fn portal_to_position(&self, portal_name: &str) -> Option<&Point> {
        let array = self.portals.get(portal_name)?;
        assert_eq!(array.len(), 1);
        array.last()
    }

    fn shortest_distance_recursive(&self) -> Option<usize> {
        let target = *self.portal_to_position(PORTAL_OUT)?;
        type TrackSet = HashSet<(Point, usize)>;
        let mut queue = VecDeque::new();
        let start = self.portal_to_position(PORTAL_IN)?;
        queue.push_back((*start, 0, 0));

        let mut seen = TrackSet::new();

        while let Some((pos, level, steps)) = queue.pop_front() {
            seen.insert((pos, level));

            let next_steps = steps + 1;
            for p in Direction::all()
                .iter()
                .map(|dir| pos.moved_by(dir))
                .filter(|p| {
                    let Some(elem) = self.points.get(&p) else {
                        return false;
                    };
                    match elem {
                        MazeElement::Portal(name) if name == PORTAL_OUT => level == 0,
                        MazeElement::Portal(name) if name == PORTAL_IN => level == 0,
                        MazeElement::Portal(_) if level == 0 => self.is_inner(p),
                        _ => true,
                    }
                })
                .filter(|p| !seen.contains(&(*p, level)))
            {
                if p == target {
                    return Some(next_steps);
                }
                queue.push_back((p, level, next_steps));
            }

            match self.points.get(&pos).expect("Unreachable (1)") {
                MazeElement::Open => {
                    continue;
                }
                MazeElement::Portal(name) if name == PORTAL_IN => {
                    continue;
                }
                MazeElement::Portal(name) if name == PORTAL_OUT => {
                    assert!(level > 0);
                    continue;
                }
                MazeElement::Portal(name) => {
                    let next_pos = self
                        .portals
                        .get(name)
                        .expect("Unreachable (2)")
                        .iter()
                        .filter(|val| **val != pos)
                        .last()
                        .expect("Unreachable (3)");
                    let is_next_inner = self.is_inner(next_pos);
                    let next_level = if is_next_inner { level - 1 } else { level + 1 };
                    if seen.contains(&(*next_pos, next_level)) {
                        continue;
                    }
                    queue.push_back((*next_pos, next_level, next_steps));
                }
            }
        }
        None
    }

    fn is_inner(&self, point: &Point) -> bool {
        let offset = 3;
        (offset..self.rows - offset).contains(&point.y)
            && (offset..self.cols - offset).contains(&point.x)
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
            .shortest_distance_portals()
            .expect("Not found")
            .to_string()
    }

    fn part_two(&self) -> String {
        self.maze
            .shortest_distance_recursive()
            .expect("Not found")
            .to_string()
    }

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
        assert!(!sol.maze.portals.is_empty());

        assert!(sol.maze.portals.contains_key(PORTAL_IN));
        assert!(sol.maze.portals.contains_key(PORTAL_OUT));
        Ok(())
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
        assert_eq!(maze.shortest_distance_portals(), Some(23));
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
        assert_eq!(sol.part_two(), "6706");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_20> {
        AoC2019_20::new()
    }

    #[test]
    fn aoc2019_20_case_2() {
        let puzzle = AoC2019_20::with_lines(&make_test_input());
        assert_eq!(puzzle.part_two(), "396")
    }

    fn make_test_input() -> Vec<&'static str> {
        [
            "             Z L X W       C                 ",
            "             Z P Q B       K                 ",
            "  ###########.#.#.#.#######.###############  ",
            "  #...#.......#.#.......#.#.......#.#.#...#  ",
            "  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  ",
            "  #.#...#.#.#...#.#.#...#...#...#.#.......#  ",
            "  #.###.#######.###.###.#.###.###.#.#######  ",
            "  #...#.......#.#...#...#.............#...#  ",
            "  #.#########.#######.#.#######.#######.###  ",
            "  #...#.#    F       R I       Z    #.#.#.#  ",
            "  #.###.#    D       E C       H    #.#.#.#  ",
            "  #.#...#                           #...#.#  ",
            "  #.###.#                           #.###.#  ",
            "  #.#....OA                       WB..#.#..ZH",
            "  #.###.#                           #.#.#.#  ",
            "CJ......#                           #.....#  ",
            "  #######                           #######  ",
            "  #.#....CK                         #......IC",
            "  #.###.#                           #.###.#  ",
            "  #.....#                           #...#.#  ",
            "  ###.###                           #.#.#.#  ",
            "XF....#.#                         RF..#.#.#  ",
            "  #####.#                           #######  ",
            "  #......CJ                       NM..#...#  ",
            "  ###.#.#                           #.###.#  ",
            "RE....#.#                           #......RF",
            "  ###.###        X   X       L      #.#.#.#  ",
            "  #.....#        F   Q       P      #.#.#.#  ",
            "  ###.###########.###.#######.#########.###  ",
            "  #.....#...#.....#.......#...#.....#.#...#  ",
            "  #####.#.###.#######.#######.###.###.#.#.#  ",
            "  #.......#.......#.#.#.#.#...#...#...#.#.#  ",
            "  #####.###.#####.#.#.#.#.###.###.#.###.###  ",
            "  #.......#.....#.#...#...............#...#  ",
            "  #############.#.#.###.###################  ",
            "               A O F   N                     ",
            "               A A D   M                     ",
        ]
        .to_vec()
    }
}
