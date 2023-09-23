use crate::solution::Solution;
use crate::utils::*;

use std::io;

enum TrackError {
    UnexpectedValue(char),
}

impl From<TrackError> for String {
    fn from(value: TrackError) -> Self {
        match value {
            TrackError::UnexpectedValue(ch) => format!("Unexpected character #{}", ch as u8),
        }
    }
}

#[derive(Debug, PartialEq)]
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
            '/' => Ok(Self::CurveRight),
            '\\' => Ok(Self::CurveLeft),
            '<' | '>' | '-' => Ok(Self::HorizontalPath),
            'v' | '^' | '|' => Ok(Self::VerticalPath),
            '+' => Ok(Self::Intersection),
            ' ' => Ok(Self::None),
            _ => Err(TrackError::UnexpectedValue(value)),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
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

#[derive(Clone)]
struct Cart {
    direction: Direction,
    coordinate: Coordinate,
    prev: Coordinate,
    phase: usize,
}

impl Cart {
    fn new(direction: Direction, coordinate: Coordinate) -> Self {
        Self {
            direction,
            coordinate,
            prev: coordinate,
            phase: 0,
        }
    }

    fn teak(&mut self, element: &TrackElement) {
        let prev_direction = self.direction.clone();
        self.prev = self.coordinate;
        let mut is_step_needed = false;
        match (element, &self.direction) {
            (TrackElement::HorizontalPath, Direction::Left) => {
                self.coordinate.y -= 1;
            }
            (TrackElement::HorizontalPath, Direction::Right) => {
                self.coordinate.y += 1;
            }
            (TrackElement::VerticalPath, Direction::Up) => {
                self.coordinate.x -= 1;
            }
            (TrackElement::VerticalPath, Direction::Down) => {
                self.coordinate.x += 1;
            }
            (TrackElement::CurveRight, Direction::Up) => {
                self.direction = Direction::Right;
            }
            (TrackElement::CurveRight, Direction::Down) => {
                self.direction = Direction::Left;
            }
            (TrackElement::CurveRight, Direction::Left) => {
                self.direction = Direction::Down;
            }
            (TrackElement::CurveRight, Direction::Right) => {
                self.direction = Direction::Up;
            }
            (TrackElement::CurveLeft, Direction::Up) => {
                self.direction = Direction::Left;
            }
            (TrackElement::CurveLeft, Direction::Down) => {
                self.direction = Direction::Right;
            }
            (TrackElement::CurveLeft, Direction::Left) => {
                self.direction = Direction::Up;
            }
            (TrackElement::CurveLeft, Direction::Right) => {
                self.direction = Direction::Down;
            }
            (TrackElement::Intersection, _) => {
                let pos = self.phase % 3;
                let new_direction = match pos {
                    0 => Some(Direction::Left),
                    2 => Some(Direction::Right),
                    _ => None,
                };
                if let Some(direction) = new_direction {
                    self.direction = match (&self.direction, &direction) {
                        (Direction::Up, Direction::Left) => Direction::Left,
                        (Direction::Up, Direction::Right) => Direction::Right,
                        (Direction::Down, Direction::Left) => Direction::Right,
                        (Direction::Down, Direction::Right) => Direction::Left,

                        (Direction::Left, Direction::Right) => Direction::Up,
                        (Direction::Left, Direction::Left) => Direction::Down,

                        (Direction::Right, Direction::Left) => Direction::Up,
                        (Direction::Right, Direction::Right) => Direction::Down,

                        _ => panic!("Shouldn't reach this branch"),
                    };
                } else {
                    is_step_needed = true;
                }
                self.phase += 1;
            }
            _ => panic!(
                "Invalid state: direction = {:?}, elem = {:?}",
                self.direction, element
            ),
        }
        is_step_needed |= prev_direction != self.direction;
        if is_step_needed {
            self.teak(&self.direction.clone().into());
        }
    }
}

struct Collider<'a> {
    track: &'a Track,
    carts: Vec<Cart>,
}

impl<'a> Collider<'a> {
    fn new(track: &'a Track, carts: Vec<Cart>) -> Self {
        Self { track, carts }
    }

    fn get_collisions(&self) -> Vec<Coordinate> {
        let len = self.carts.len();
        let mut result = Vec::new();
        for (i, a) in self.carts.iter().enumerate().take(len - 1) {
            for b in self.carts.iter().skip(i + 1) {
                let coord_a = a.coordinate;
                let coord_b = b.coordinate;
                if coord_a == coord_b {
                    result.push(coord_a);
                } else {
                    let prev_a = a.prev;
                    let prev_b = b.prev;
                    if coord_a == prev_b && coord_b == prev_a {
                        result.push(coord_a);
                        result.push(coord_b);
                    }
                }
            }
        }
        result
    }

    fn remove_collisions(&mut self) {
        let collisions = self.get_collisions();
        if collisions.is_empty() {
            return;
        }
        self.carts = self
            .carts
            .iter()
            .filter(|&cart| !collisions.contains(&cart.coordinate))
            .cloned()
            .collect::<Vec<Cart>>();
    }

    fn can_collide(&self) -> bool {
        self.carts.len() > 1
    }

    fn teak(&mut self) {
        self.carts.iter_mut().for_each(|cart| {
            let coord = cart.coordinate;
            let element = &self.track[coord.x][coord.y];
            cart.teak(element);
        });
    }

    fn get_cart(&self) -> Cart {
        self.carts.first().unwrap().clone()
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
                    let x = track.len();
                    let y = row.len();
                    let coordinate = Coordinate { x, y };
                    row.push(TrackElement::from(direction.clone()));
                    let cart = Cart::new(direction, coordinate);
                    carts.push(cart);
                } else {
                    let elem = TrackElement::try_from(ch)?;
                    row.push(elem);
                }
            }
            track.push(row);
        }
        Ok((track, carts))
    }
}

impl Solution for AoC2018_13 {
    fn part_one(&self) -> String {
        let mut collider = Collider::new(&self.track, self.carts.clone());
        loop {
            let collisions = collider.get_collisions();
            if let Some(coord) = collisions.first() {
                assert_eq!(collisions.len(), 1);
                break format!("{},{}", coord.y, coord.x);
            }
            collider.teak();
        }
    }

    fn part_two(&self) -> String {
        let mut collider = Collider::new(&self.track, self.carts.clone());
        while collider.can_collide() {
            collider.teak();
            collider.remove_collisions();
        }
        let cart = collider.get_cart();
        let coord = cart.coordinate;
        format!("{},{}", coord.y, coord.x)
    }

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

        sol.carts
            .iter()
            .map(|cart| cart.coordinate)
            .for_each(|coord| {
                assert_ne!(
                    sol.track[coord.x][coord.y],
                    TrackElement::None,
                    "x: {}, y: {}",
                    coord.x,
                    coord.y
                );
            });
        Ok(())
    }

    #[test]
    fn aoc2018_13_example1() -> io::Result<()> {
        let input = [
            "/->-\\        ",
            "|   |  /----\\",
            "| /-+--+-\\  |",
            "| | |  | v  |",
            "\\-+-/  \\-+--/",
            "  \\------/   ",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        let (track, carts) = AoC2018_13::parse(&input)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, String::from(err)))?;
        let sol = AoC2018_13 { track, carts };
        assert_eq!(sol.part_one(), "7,3");
        Ok(())
    }

    #[test]
    fn aoc2018_13_example2() -> io::Result<()> {
        let input = [
            "/>-<\\  ", "|   |  ", "| /<+-\\", "| | | v", "\\>+</ |", "  |   ^", "  \\<->/",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        let (track, carts) = AoC2018_13::parse(&input)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, String::from(err)))?;
        let sol = AoC2018_13 { track, carts };
        assert_eq!(sol.part_two(), "6,4");
        Ok(())
    }

    #[test]
    fn aoc2018_13_correctness() -> io::Result<()> {
        let sol = AoC2018_13::new()?;
        assert_eq!(sol.part_one(), "58,93");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
