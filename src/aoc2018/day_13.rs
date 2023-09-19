use crate::solution::Solution;
use crate::utils::*;

use std::io;

enum TrackError {
    UnexpectedValue(char),
}

impl From<TrackError> for String {
    fn from(value: TrackError) -> Self {
        match value {
            TrackError::UnexpectedValue(ch) => format!("Unexpected character {ch}"),
        }
    }
}

enum TrackElement {
    CurveLeft,
    CurveRight,
    HorizontalPath,
    VerticalPath,
    Intersection,
    None,
}

impl TryFrom<char> for TrackElement {
    type Error = TrackError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '/'  => Ok(Self::CurveRight),
            '\\' => Ok(Self::CurveLeft),
            '<' | '>' | '-' => Ok(Self::HorizontalPath),
            'v' | '^' | '|' => Ok(Self::VerticalPath),
            '+' => Ok(Self::Intersection),
            ' ' => Ok(Self::None),
            _ => Err(TrackError::UnexpectedValue(value)),
        }
    }
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = TrackError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            _ => Err(TrackError::UnexpectedValue(value)),
        }
    }
}

impl From<Direction> for TrackElement {
    fn from(val: Direction) -> Self {
        match val {
            Direction::Left | Direction::Right => TrackElement::HorizontalPath,
            Direction::Up | Direction::Down => TrackElement::VerticalPath,
        }
    }
}

type Coordinate = Point2d<usize>;
type Track = Vec<Vec<TrackElement>>;

struct Cart {
    direction: Direction,
    coordinate: Coordinate,
    phase: usize,
}

impl Cart {
    fn new(direction: Direction, coordinate: Coordinate) -> Self {
        Self { direction, coordinate, phase: 0 }
    }

    fn make_turn(&mut self, direction: &Direction) {
        self.direction = match (&self.direction, direction) {
            (Direction::Up, Direction::Left) => Direction::Left,
            (Direction::Up, Direction::Right) => Direction::Right,
            (Direction::Down, Direction::Left) => Direction::Right,
            (Direction::Down, Direction::Right) => Direction::Left,            
            _ => direction.clone()
        };
    }

    fn teak(element: &TrackElement) {
        match element {
            _ => todo!()
        }
    }
}

pub struct AoC2018_13 {
    track: Track,
    carts: Vec<Cart>,
}

impl AoC2018_13 {
    pub fn new() -> io::Result<Self> {
        let input = read_file_as_lines("input/aoc2018_13")?;
        let (track, carts) = Self::parse(&input)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, String::from(err)))?;
        Ok(Self { track, carts })
    }

    fn parse(input: &[String]) -> Result<(Track, Vec<Cart>), TrackError> {
        let mut carts: Vec<Cart> = Vec::new();
        let mut track: Track = Vec::new();
        for s in input {
            let mut row: Vec<TrackElement> = Vec::new();
            for ch in s.chars() {
                if let Ok(direction) = Direction::try_from(ch) {
                    row.push(TrackElement::from(direction.clone()));
                    let x = track.len();
                    let y = row.len();
                    let coordinate = Coordinate { x, y };
                    let cart = Cart::new(direction, coordinate);
                    carts.push(cart);
                } else if let Ok(elem) = TrackElement::try_from(ch) {
                    row.push(elem);
                } else {
                    return Err(TrackError::UnexpectedValue(ch));
                }
            }
            track.push(row);
        }
        Ok((track, carts))
    }
}

impl Solution for AoC2018_13 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 13: Mine Cart Madness".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_13_input_load_test() -> io::Result<()> {
        let sol = AoC2018_13::new()?;
        assert!(!sol.track.is_empty());
        assert!(!sol.carts.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_13_correctness() -> io::Result<()> {
        let sol = AoC2018_13::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
