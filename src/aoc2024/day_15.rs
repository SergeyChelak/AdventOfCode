use crate::solution::Solution;
use crate::utils::{Direction, Position2, Vec2};

use std::fs::read_to_string;
use std::io;

#[derive(Debug, Clone, Copy)]
enum Element {
    Empty,
    Wall,
    Box,
}

type Position = Position2<usize>;

pub struct AoC2024_15 {
    maze: Vec2<Element>,
    robot_position: Position,
    path: Vec<Direction>,
}

impl AoC2024_15 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2024_15")?;
        Ok(Self::with_string(&input))
    }

    fn with_string<T: AsRef<str>>(input: T) -> Self {
        let (maze, path) = input.as_ref().split_once("\n\n").expect("Invalid input");
        let (maze, pos) = Self::parse_maze(maze);
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
        Self {
            maze,
            robot_position: pos,
            path,
        }
    }

    fn parse_maze(input: &str) -> (Vec2<Element>, Position) {
        let mut maze = Vec::new();
        let mut position: Option<Position> = None;
        for (row, arr) in input.split('\n').enumerate() {
            let mut tmp = Vec::new();
            for (col, ch) in arr.chars().enumerate() {
                match ch {
                    '#' => tmp.push(Element::Wall),
                    'O' => tmp.push(Element::Box),
                    _ => {
                        if ch == '@' {
                            position = Some(Position::new(row, col))
                        }
                        tmp.push(Element::Empty);
                    }
                }
            }
            maze.push(tmp);
        }
        (maze, position.expect("robot position not found"))
    }
}

impl Solution for AoC2024_15 {
    fn part_one(&self) -> String {
        let mut maze = self.maze.clone();
        let mut pos = self.robot_position;
        // dump(&maze, pos);

        for dir in &self.path {
            // println!();
            process_move(&mut maze, &mut pos, *dir);
            // dump(&maze, pos);
        }
        calc_gps_sum(&maze).to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "2024/Day 15: Warehouse Woes".to_string()
    }
}

fn dump(maze: &[Vec<Element>], pos: Position) {
    for (r, row) in maze.iter().enumerate() {
        for (c, val) in row.iter().enumerate() {
            if pos.row == r && pos.col == c {
                print!("@");
                continue;
            }
            let ch = match val {
                Element::Box => 'O',
                Element::Empty => '.',
                Element::Wall => '#',
            };
            print!("{ch}");
        }
        println!();
    }
}

fn process_move(maze: &mut [Vec<Element>], pos: &mut Position, direction: Direction) {
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
        use Element::*;
        match maze[current.row][current.col] {
            Wall => {
                return;
            }
            Box => {
                if box_position.is_none() {
                    box_position = Some(current);
                }
            }
            Empty => {
                break;
            }
        }
    }
    maze[pos.row][pos.col] = Element::Empty;
    if let Some(bp) = box_position {
        *pos = bp;
        maze[current.row][current.col] = Element::Box;
    } else {
        *pos = current;
    }
    maze[pos.row][pos.col] = Element::Empty;
}

fn calc_gps_sum(data: &[Vec<Element>]) -> usize {
    let mut total = 0;
    for (r, row) in data.iter().enumerate() {
        for (c, elem) in row.iter().enumerate() {
            if !matches!(elem, Element::Box) {
                continue;
            }
            total += r * 100 + c;
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
        assert!(!sol.maze.is_empty());
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
}
