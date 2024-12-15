use crate::solution::Solution;
use crate::utils::{Direction, Position2, Vec2};

use std::fs::read_to_string;
use std::io;

type Position = Position2<usize>;

const ROBOT: char = '@';
const WALL: char = '#';
const EMPTY: char = '.';
const BOX: char = 'O';

const BOX_L: char = '[';
const BOX_R: char = ']';

pub struct AoC2024_15 {
    map: Vec2<char>,
    path: Vec<Direction>,
}

impl AoC2024_15 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2024_15")?;
        Ok(Self::with_string(&input))
    }

    fn with_string<T: AsRef<str>>(input: T) -> Self {
        let (map, path) = input.as_ref().split_once("\n\n").expect("Invalid input");
        let map = map
            .split('\n')
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let path = path
            .split('\n')
            .map(|s| s.trim())
            .flat_map(|s| s.chars())
            .map(|ch| match ch {
                'v' => Direction::Down,
                '>' => Direction::Right,
                '^' => Direction::Up,
                '<' => Direction::Left,
                _ => panic!("unexpected path character {ch}"),
            })
            .collect::<Vec<_>>();
        Self { map, path }
    }
}

impl Solution for AoC2024_15 {
    fn part_one(&self) -> String {
        let mut map = self.map.clone();
        let mut pos = get_robot_position(&map).expect("robot position not found");
        map[pos.row][pos.col] = EMPTY;
        for dir in &self.path {
            simple_move(&mut map, &mut pos, *dir);
        }
        calc_gps_sum(&map).to_string()
    }

    fn part_two(&self) -> String {
        let mut map = expand_map(&self.map);
        let mut pos = get_robot_position(&map).expect("robot position not found");
        map[pos.row][pos.col] = EMPTY;
        for dir in &self.path {
            wide_move(&mut map, &mut pos, *dir);
        }
        todo!()
    }

    fn description(&self) -> String {
        "2024/Day 15: Warehouse Woes".to_string()
    }
}

fn expand_map(map: &[Vec<char>]) -> Vec2<char> {
    let mut result = Vec::new();
    for row in map.iter() {
        let mut tmp = Vec::new();
        for elem in row {
            match *elem {
                EMPTY => {
                    tmp.push(EMPTY);
                    tmp.push(EMPTY);
                }
                BOX => {
                    tmp.push(BOX_L);
                    tmp.push(BOX_R);
                }
                ROBOT => {
                    tmp.push(ROBOT);
                    tmp.push(EMPTY);
                }
                WALL => {
                    tmp.push(WALL);
                    tmp.push(WALL);
                }
                _ => unreachable!("failed to expand the map"),
            }
        }
        result.push(tmp);
    }
    result
}

fn get_robot_position(map: &[Vec<char>]) -> Option<Position> {
    for (r, row) in map.iter().enumerate() {
        for (c, elem) in row.iter().enumerate() {
            if *elem == ROBOT {
                return Some(Position::new(r, c));
            }
        }
    }
    None
}

fn simple_move(map: &mut [Vec<char>], pos: &mut Position, direction: Direction) {
    let next = |p: Position| -> Position {
        use Direction::*;
        match direction {
            Left => Position::new(p.row, p.col - 1),
            Right => Position::new(p.row, p.col + 1),
            Up => Position::new(p.row - 1, p.col),
            Down => Position::new(p.row + 1, p.col),
        }
    };
    let mut current = *pos;
    let mut box_position: Option<Position> = None;
    loop {
        current = next(current);
        match map[current.row][current.col] {
            WALL => {
                return;
            }
            BOX => {
                if box_position.is_none() {
                    box_position = Some(current);
                }
            }
            EMPTY => {
                break;
            }
            _ => unreachable!("???"),
        }
    }
    map[pos.row][pos.col] = EMPTY;
    if let Some(bp) = box_position {
        *pos = bp;
        map[current.row][current.col] = BOX;
    } else {
        *pos = current;
    }
    map[pos.row][pos.col] = EMPTY;
}

fn wide_move(map: &mut [Vec<char>], pos: &mut Position, direction: Direction) {
    todo!()
}

fn calc_gps_sum(map: &[Vec<char>]) -> usize {
    let mut total = 0;
    for (r, row) in map.iter().enumerate() {
        for (c, elem) in row.iter().enumerate() {
            total += match *elem {
                BOX | BOX_L => r * 100 + c,
                // this should match to condition:
                // >> For these larger boxes, distances are measured from
                // >> the edge of the map to the closest edge of the box in question
                // but is it doesn't fit to the expected sum gps coordinates in example
                // Also, the statement above means that the rule should be applied to the rows as well
                // BOX_L => r * 100 + c.min(cols - c),
                _ => 0,
            }
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_15_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.map.is_empty());
        assert!(!sol.path.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2024_15_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "1509074");
        Ok(())
    }

    #[test]
    fn aoc2024_15_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2024_15> {
        AoC2024_15::new()
    }

    #[test]
    fn aoc2024_15_small_case_1() {
        let puzzle = make_small_test_solution();
        assert_eq!("2028", puzzle.part_one());
    }

    fn make_small_test_solution() -> AoC2024_15 {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        AoC2024_15::with_string(input)
    }

    #[test]
    fn aoc2024_15_large_gps_check() {
        let map = "####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################"
            .split('\n')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(calc_gps_sum(&map), 9021);
    }
}
