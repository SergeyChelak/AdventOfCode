use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = isize;
type Position = Point2d<Int>;

struct Ship {
    dir: Direction,
    pos: Position,
}

impl Ship {
    fn new() -> Self {
        Self {
            dir: Direction::Right,
            pos: Position::zero(),
        }
    }

    fn movement(&mut self, instr: &str) {
        let (cmd, value) = instr.split_at(1);
        let value = value.parse::<Int>().expect("Invalid instruction format");
        match cmd {
            "N" => self.pos.y -= value,
            "S" => self.pos.y += value,
            "E" => self.pos.x += value,
            "W" => self.pos.x -= value,
            "L" => self.turn_direction(Direction::turn_left, value),
            "R" => self.turn_direction(Direction::turn_right, value),
            "F" => self.move_position(value),
            _ => unreachable!(),
        }
    }

    fn turn_direction(&mut self, turn: impl Fn(&Direction) -> Direction, value: isize) {
        assert!(value % 90 == 0);
        for _ in 0..(value / 90) {
            self.dir = turn(&self.dir);
        }
    }

    fn move_position(&mut self, value: Int) {
        match self.dir {
            Direction::Up => self.pos.y -= value,
            Direction::Down => self.pos.y += value,
            Direction::Left => self.pos.x -= value,
            Direction::Right => self.pos.x += value,
        }
    }

    fn dist(&self) -> Int {
        self.pos.x.abs() + self.pos.y
    }
}

pub struct AoC2020_12 {
    input: Vec<String>,
}

impl AoC2020_12 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2020_12")?;
        // Ok(Self::parse(&lines))
        Ok(Self { input: lines })
    }

    // fn parse<T: AsRef<str>>(lines: &[T]) -> Self {
    //     todo!()
    // }
}

impl Solution for AoC2020_12 {
    fn part_one(&self) -> String {
        let mut ship = Ship::new();
        self.input.iter().for_each(|instr| ship.movement(instr));

        ship.dist().to_string()
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
