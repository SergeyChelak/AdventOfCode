use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;
use std::str::FromStr;

type Int = i32;

enum ParseError {
    UnknownDirection,
    InvalidValue,
}

#[derive(Debug, Clone, Copy)]
struct WireNode {
    value: Int,
    direction: Direction,
}

impl FromStr for WireNode {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match &s[..1] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(ParseError::UnknownDirection),
        };
        let value = s[1..]
            .parse::<Int>()
            .map_err(|_| ParseError::InvalidValue)?;
        Ok(Self { value, direction })
    }
}

type Filler = usize;

#[derive(Debug, Clone, Copy)]
enum GridElement {
    Empty,
    Color(Filler),
    Intersection,
}

type Position = Point2d<Int>;
type Grid = HashMap<Position, GridElement>;
type Wire = Vec<WireNode>;

fn fill_grid(grid: &mut Grid, wire: &Wire, filler: Filler) {
    let mut position = Position::new(0, 0);
    for node in wire {
        let times = node.value;
        for _ in 0..times {
            position = match node.direction {
                Direction::Up => position.up(),
                Direction::Down => position.down(),
                Direction::Left => position.left(),
                Direction::Right => position.right(),
            };
            let grid_value = grid.entry(position).or_insert(GridElement::Empty);
            match grid_value {
                GridElement::Empty => *grid_value = GridElement::Color(filler),
                GridElement::Color(color) if *color != filler => {
                    *grid_value = GridElement::Intersection
                }
                _ => continue,
            }
        }
    }
}

fn min_dist(grid: &Grid) -> Option<i32> {
    grid.iter()
        .filter(|(_, v)| matches!(v, GridElement::Intersection))
        .map(|(k, _)| k.x.abs() + k.y.abs())
        .min()
}

pub struct AoC2019_03 {
    wires: Vec<Wire>,
}

impl AoC2019_03 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2019_03")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let parse_wire_path = |s: &str| -> Result<Wire, ParseError> {
            s.split(',').map(WireNode::from_str).collect()
        };
        let wires = lines
            .iter()
            .map(|x| x.as_ref())
            .map(parse_wire_path)
            .map(|x| x.ok().expect("Failed to parse wire1 path"))
            .collect::<Vec<Wire>>();
        Self { wires }
    }
}

impl Solution for AoC2019_03 {
    fn part_one(&self) -> String {
        let mut grid = Grid::new();
        for (filler, wire) in self.wires.iter().enumerate() {
            fill_grid(&mut grid, wire, filler);
            // dump(&grid);
            // println!()
        }
        min_dist(&grid)
            .map(|x| x.to_string())
            .unwrap_or("Not found".to_string())
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 3: Crossed Wires".to_string()
    }
}

fn dump(grid: &Grid) {
    let min_x = grid.iter().map(|(k, _)| k.x).min().unwrap();
    let min_y = grid.iter().map(|(k, _)| k.y).min().unwrap();
    let max_x = grid.iter().map(|(k, _)| k.x).max().unwrap();
    let max_y = grid.iter().map(|(k, _)| k.y).max().unwrap();

    for row in min_y..=max_y {
        for col in min_x..=max_x {
            let pos = Position::new(col, row);
            match grid.get(&pos).unwrap_or(&GridElement::Empty) {
                GridElement::Empty => print!("."),
                GridElement::Color(clr) => print!("{clr}"),
                GridElement::Intersection => print!("+"),
            }
        }
        println!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_03_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.wires.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_03_case_1() -> io::Result<()> {
        let lines = ["R8,U5,L5,D3", "U7,R6,D4,L4"];
        let puzzle = AoC2019_03::with_lines(&lines);
        assert_eq!(puzzle.part_one(), "6");
        Ok(())
    }

    #[test]
    fn aoc2019_03_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "4981");
        Ok(())
    }

    #[test]
    fn aoc2019_03_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_03> {
        AoC2019_03::new()
    }
}
