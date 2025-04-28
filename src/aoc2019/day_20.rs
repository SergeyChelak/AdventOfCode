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
        let element_filter = |_p: &Point, _elem: &MazeElement, _level: usize| -> bool { true };
        let level_calculator = |_p: &Point, _level: usize| 0;
        self.search(element_filter, level_calculator)
    }

    fn shortest_distance_recursive(&self) -> Option<usize> {
        let element_filter = |p: &Point, elem: &MazeElement, level: usize| -> bool {
            match elem {
                MazeElement::Portal(name) if name == PORTAL_OUT => level == 0,
                MazeElement::Portal(name) if name == PORTAL_IN => level == 0,
                MazeElement::Portal(_) if level == 0 => self.is_inner(p),
                _ => true,
            }
        };

        let level_calculator = |p: &Point, level: usize| {
            if self.is_inner(p) {
                level - 1
            } else {
                level + 1
            }
        };
        self.search(element_filter, level_calculator)
    }

    fn search<EF, LC>(&self, element_filter: EF, level_calculator: LC) -> Option<usize>
    where
        EF: Fn(&Point, &MazeElement, usize) -> bool,
        LC: Fn(&Point, usize) -> usize,
    {
        let target = *self.portal_to_position(PORTAL_OUT)?;
        let start = self.portal_to_position(PORTAL_IN)?;

        let mut queue = VecDeque::new();
        queue.push_back((*start, 0, 0));

        let mut seen = HashSet::new();

        while let Some((pos, level, steps)) = queue.pop_front() {
            seen.insert((pos, level));

            let next_steps = steps + 1;
            for p in Direction::all()
                .iter()
                .map(|dir| pos.moved_by(dir))
                .filter(|p| {
                    let Some(elem) = self.points.get(p) else {
                        return false;
                    };
                    element_filter(p, elem, level)
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
                    let next_level = level_calculator(next_pos, level);
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

    fn portal_to_position(&self, portal_name: &str) -> Option<&Point> {
        let array = self.portals.get(portal_name)?;
        assert_eq!(array.len(), 1);
        array.last()
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
