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

struct SimpleNavigation {
    dir: Direction,
    pos: Position,
}

impl SimpleNavigation {
    fn new() -> Self {
        Self {
            dir: Direction::Right,
            pos: Position::zero(),
        }
    }

    fn movement(&mut self, instr: &Instruction) {
        let value = instr.param;
        match instr.command {
            Command::North => self.pos.y -= value,
            Command::South => self.pos.y += value,
            Command::East => self.pos.x += value,
            Command::West => self.pos.x -= value,
            Command::Left => self.turn_direction(Direction::turn_left, value),
            Command::Right => self.turn_direction(Direction::turn_right, value),
            Command::Forward => self.move_in_direction(value),
        }
    }

    fn turn_direction(&mut self, turn: impl Fn(&Direction) -> Direction, value: isize) {
        assert!(value % 90 == 0);
        for _ in 0..(value / 90) {
            self.dir = turn(&self.dir);
        }
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

impl Position {
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
        //        wp (10, 1)  p (0, 0)
        // f10        10, 1    100, 10
        // n3         10, 4    100, 10
        // f7         10, 4    170, 38
        // r90        4, 10    170, 38
        // f11        4, 10    214, 72
    }
}

impl Solution for AoC2020_12 {
    fn part_one(&self) -> String {
        let mut ship = SimpleNavigation::new();
        self.input.iter().for_each(|instr| ship.movement(instr));

        ship.pos.manhattan_distance().to_string()
    }

    // fn part_two(&self) -> String {
    // }

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
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_12> {
        AoC2020_12::new()
    }
}
