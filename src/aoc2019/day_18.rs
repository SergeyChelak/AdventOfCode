use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
use std::io;

type Int = isize;
type Maze = HashMap<Point, char>;
type Point = Point2d<Int>;

const TILE_OPEN: char = '.';
const TILE_WALL: char = '#';
const TILE_START: char = '@';

pub struct AoC2019_18 {
    maze: Maze,
    start: Point,
}

impl AoC2019_18 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2019_18")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines<T: AsRef<str>>(input: &[T]) -> Self {
        let mut maze = HashMap::new();
        let mut start: Option<Point> = None;
        for (row, line) in input.iter().map(|x| x.as_ref()).enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let point = Point::new(col as Int, row as Int);
                let mut value = ch;
                if ch == TILE_START {
                    start = Some(point);
                    value = TILE_OPEN;
                };
                maze.insert(point, value);
            }
        }
        Self {
            maze,
            start: start.expect("Initial position not found"),
        }
    }
}

impl Solution for AoC2019_18 {
    fn part_one(&self) -> String {
        shortest_path_len(&self.maze, &[self.start]).to_string()
    }

    fn part_two(&self) -> String {
        let adjacent = Direction::all()
            .iter()
            .map(|dir| self.start.moved_by(dir))
            .collect::<Vec<_>>();
        let start_points = [
            self.start
                .moved_by(&Direction::Up)
                .moved_by(&Direction::Left),
            self.start
                .moved_by(&Direction::Up)
                .moved_by(&Direction::Right),
            self.start
                .moved_by(&Direction::Down)
                .moved_by(&Direction::Left),
            self.start
                .moved_by(&Direction::Down)
                .moved_by(&Direction::Right),
        ];

        let is_valid_area = adjacent.iter().chain(start_points.iter()).all(|p| {
            let Some(ch) = self.maze.get(p) else {
                return false;
            };
            *ch != TILE_WALL
        });
        assert!(is_valid_area);

        let mut maze = self.maze.clone();
        adjacent.iter().for_each(|p| {
            maze.insert(*p, TILE_WALL);
        });

        shortest_path_len(&maze, &start_points).to_string()
    }

    fn description(&self) -> String {
        "Day 18: Many-Worlds Interpretation".to_string()
    }
}

type Keys = [bool; 26];
type PointData = (Point, usize);
type PointDataSet = HashSet<PointData>;
type MemoKey = (Point, Keys);
type Memo = HashMap<MemoKey, PointDataSet>;

#[derive(Clone)]
struct StackElement {
    positions: Vec<Point>,
    keys: Keys,
    distance: usize,
}

impl StackElement {
    fn with_points(points: &[Point]) -> Self {
        Self {
            positions: points.to_owned(),
            keys: [false; 26],
            distance: 0,
        }
    }
}

fn shortest_path_len(map: &Maze, start: &[Point]) -> usize {
    let mut stack = vec![StackElement::with_points(start)];

    let mut min_distance = usize::MAX;
    let mut distances = HashMap::new();
    // no reason to insert initial position as it doesn't impact the search

    let mut memo = Memo::new();

    while !stack.is_empty() {
        let data = stack.last().unwrap().clone();

        let mut vault_points = Vec::new();
        let mut available_keys_count = 0;
        for memo_key in data.positions.iter().map(|p| (*p, data.keys)) {
            let available = memo
                .entry(memo_key)
                .or_insert_with(|| get_available_points(map, memo_key.0, &memo_key.1));
            available_keys_count += available.len();
            vault_points.push(available.clone());
        }

        let mut has_next = false;
        if available_keys_count == 0 {
            min_distance = min_distance.min(data.distance);
        } else {
            for (vault, points) in vault_points.iter().enumerate() {
                for (point, dist) in points.iter() {
                    let acc_distance = dist + data.distance;
                    if acc_distance >= min_distance {
                        continue;
                    }

                    let distance_key = (*point, data.keys);
                    let existing_distance = distances.get(&distance_key).unwrap_or(&usize::MAX);
                    if *existing_distance <= acc_distance {
                        continue;
                    }
                    distances.insert(distance_key, acc_distance);

                    let mut keys = data.keys;
                    let index = key_index(map, *point).expect("Missing value");
                    keys[index] = true;

                    let mut positions = data.positions.clone();
                    positions[vault] = *point;

                    let elem = StackElement {
                        positions,
                        keys,
                        distance: acc_distance,
                    };
                    stack.push(elem);
                    has_next = true;
                }
            }
        }
        if !has_next {
            stack.pop();
        }
    }

    min_distance
}

fn get_available_points(map: &Maze, start: Point, keys: &Keys) -> PointDataSet {
    let mut result = PointDataSet::new();
    let mut current = vec![start];
    let mut seen = HashSet::new();
    let mut step = 0;
    while !current.is_empty() {
        step += 1;
        let mut next = HashSet::new();
        for point in &current {
            seen.insert(*point);
            for dir in Direction::all() {
                let adjacent = point.moved_by(&dir);
                if seen.contains(&adjacent) {
                    continue;
                }
                let Some(ch) = map.get(&adjacent) else {
                    continue;
                };
                if *ch == TILE_WALL {
                    continue;
                }
                if is_key(ch) && !keys[char_to_index(*ch)] {
                    result.insert((adjacent, step));
                    continue;
                }
                if is_door(ch) && !keys[char_to_index(ch.to_ascii_lowercase())] {
                    continue;
                }
                next.insert(adjacent);
            }
        }
        current = next.into_iter().collect::<Vec<_>>();
    }
    result
}

fn char_to_index(ch: char) -> usize {
    (ch as u8 - b'a') as usize
}

fn key_index(map: &Maze, position: Point) -> Option<usize> {
    let ch = map.get(&position)?;
    if !is_key(ch) {
        return None;
    }
    Some(char_to_index(*ch))
}

fn is_key(ch: &char) -> bool {
    ch.is_ascii_lowercase()
}

fn is_door(ch: &char) -> bool {
    ch.is_ascii_uppercase()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_18_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.maze.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_18_case_1() {
        #[rustfmt::skip]
        let input = [
            "#########",
            "#b.A.@.a#",
            "#########",
        ];
        let puzzle = AoC2019_18::with_lines(&input);
        assert_eq!(puzzle.part_one(), "8");
    }

    #[test]
    fn aoc2019_18_case_2() {
        #[rustfmt::skip]
        let input = [
            "########################",
            "#f.D.E.e.C.b.A.@.a.B.c.#",
            "######################.#",
            "#d.....................#",
            "########################",
        ];
        let puzzle = AoC2019_18::with_lines(&input);
        assert_eq!(puzzle.part_one(), "86");
    }

    #[test]
    fn aoc2019_18_case_3() {
        #[rustfmt::skip]
        let input = [
            "########################",
            "#...............b.C.D.f#",
            "#.######################",
            "#.....@.a.B.c.d.A.e.F.g#",
            "########################",
        ];
        let puzzle = AoC2019_18::with_lines(&input);
        assert_eq!(puzzle.part_one(), "132");
    }

    #[test]
    fn aoc2019_18_case_4() {
        #[rustfmt::skip]
        let input = [
            "#################",
            "#i.G..c...e..H.p#",
            "########.########",
            "#j.A..b...f..D.o#",
            "########@########",
            "#k.E..a...g..B.n#",
            "########.########",
            "#l.F..d...h..C.m#",
            "#################",
        ];
        let puzzle = AoC2019_18::with_lines(&input);
        assert_eq!(puzzle.part_one(), "136");
    }

    #[test]
    fn aoc2019_18_case_pt2_1() {
        #[rustfmt::skip]
        let input = [
            "#######",
            "#a.#Cd#",
            "##...##",
            "##.@.##",
            "##...##",
            "#cB#Ab#",
            "#######",
        ];
        let puzzle = AoC2019_18::with_lines(&input);
        assert_eq!(puzzle.part_two(), "8");
    }

    #[test]
    fn aoc2019_18_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "3866");
        Ok(())
    }

    #[test]
    fn aoc2019_18_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "1842");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_18> {
        AoC2019_18::new()
    }
}
