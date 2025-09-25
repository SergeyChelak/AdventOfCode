use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = isize;
type Position = Point2d<Int>;

enum Command {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

struct Instruction {
    command: Command,
    param: Int,
}

impl Instruction {
    fn new(command: Command, param: Int) -> Self {
        Self { command, param }
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let (cmd, param) = value.split_at(1);
        let param = param
            .parse::<Int>()
            .expect("Invalid instruction format: {value}");
        match cmd {
            "N" => Self::new(Command::North, param),
            "S" => Self::new(Command::South, param),
            "E" => Self::new(Command::East, param),
            "W" => Self::new(Command::West, param),
            "L" => Self::new(Command::Left, param),
            "R" => Self::new(Command::Right, param),
            "F" => Self::new(Command::Forward, param),
            _ => unreachable!(),
        }
    }
}

trait Navigation {
    fn movement(&mut self, instr: &Instruction);

    fn position(&self) -> Position;
}

struct SimpleNavigation {
    dir: Direction,
    pos: Position,
}

impl Navigation for SimpleNavigation {
    fn movement(&mut self, instr: &Instruction) {
        let value = instr.param;
        match instr.command {
            Command::North => self.pos.y -= value,
            Command::South => self.pos.y += value,
            Command::East => self.pos.x += value,
            Command::West => self.pos.x -= value,
            Command::Left => self.rotate_direction(Direction::turn_left, value),
            Command::Right => self.rotate_direction(Direction::turn_right, value),
            Command::Forward => self.move_in_direction(value),
        }
    }

    fn position(&self) -> Position {
        self.pos
    }
}

impl SimpleNavigation {
    fn new() -> Self {
        Self {
            dir: Direction::Right,
            pos: Position::zero(),
        }
    }

    fn rotate_direction(&mut self, modifier: impl Fn(&Direction) -> Direction, value: Int) {
        self.dir = rotate(self.dir, modifier, value);
    }

    fn move_in_direction(&mut self, value: Int) {
        match self.dir {
            Direction::Up => self.pos.y -= value,
            Direction::Down => self.pos.y += value,
            Direction::Left => self.pos.x -= value,
            Direction::Right => self.pos.x += value,
        }
    }
}

struct WaypointNavigation {
    pos: Position,
    waypoint: Position,
}

impl Navigation for WaypointNavigation {
    fn movement(&mut self, instr: &Instruction) {
        let value = instr.param;
        match instr.command {
            Command::North => self.waypoint.y -= value,
            Command::South => self.waypoint.y += value,
            Command::East => self.waypoint.x += value,
            Command::West => self.waypoint.x -= value,
            Command::Left => self.rotate_waypoint(Position::rotate_left, value),
            Command::Right => self.rotate_waypoint(Position::rotate_right, value),
            Command::Forward => {
                self.pos.x += value * self.waypoint.x;
                self.pos.y += value * self.waypoint.y;
            }
        }
    }

    fn position(&self) -> Position {
        self.pos
    }
}

impl WaypointNavigation {
    fn new() -> Self {
        Self {
            pos: Position::zero(),
            waypoint: Position::new(10, -1),
        }
    }

    fn rotate_waypoint(&mut self, modifier: impl Fn(&Position) -> Position, value: Int) {
        self.waypoint = rotate(self.waypoint, modifier, value);
    }
}

fn rotate<T>(initial: T, modifier: impl Fn(&T) -> T, value: Int) -> T {
    assert!(value % 90 == 0);
    let value = value % 360;
    let mut output = initial;
    for _ in 0..(value / 90) {
        output = modifier(&output);
    }
    output
}
impl Position {
    fn rotate_right(&self) -> Position {
        Position::new(-self.y, self.x)
    }

    fn rotate_left(&self) -> Position {
        Position::new(self.y, -self.x)
    }

    fn manhattan_distance(&self) -> Int {
        self.x.abs() + self.y.abs()
    }
}

pub struct AoC2020_12 {
    input: Vec<Instruction>,
}

impl AoC2020_12 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2020_12")?;
        Ok(Self::parse(&lines))
    }

    fn parse<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(Instruction::from)
            .collect();
        Self { input }
    }

    fn distance(&self, mut navigation: impl Navigation) -> String {
        self.input
            .iter()
            .for_each(|instr| navigation.movement(instr));
        navigation.position().manhattan_distance().to_string()
    }
}

impl Solution for AoC2020_12 {
    fn part_one(&self) -> String {
        self.distance(SimpleNavigation::new())
    }

    fn part_two(&self) -> String {
        self.distance(WaypointNavigation::new())
    }

    fn description(&self) -> String {
        "Day 12: Rain Risk".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_12_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2020_12_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "508");
        Ok(())
    }

    #[test]
    fn aoc2020_12_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "30761");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_12> {
        AoC2020_12::new()
    }

    #[test]
    fn aoc2020_12_case2() {
        let lines = ["F10", "N3", "F7", "R90", "F11"];
        let sol = AoC2020_12::parse(&lines);
        assert_eq!(sol.part_two(), "286")
    }
}
