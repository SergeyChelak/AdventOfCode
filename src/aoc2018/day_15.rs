use crate::solution::Solution;
use crate::utils::*;

use std::io;

#[derive(Clone, Copy, Debug)]
enum Elem {
    Elf(i32),
    Goblin(i32),
    Wall,
    Empty,
}

// Each unit, either Goblin or Elf, has 3 attack power and starts with 200 hit points.
const INITIAL_HIT_POINTS: i32 = 200;
const ATTACK_POWER: i32 = 3;

pub struct AoC2018_15 {
    maze: Vec<Vec<Elem>>,
}

impl AoC2018_15 {
    pub fn new() -> io::Result<Self> {
        let input = read_file_as_lines("input/aoc2018_15")?;
        let maze = Self::parse_maze(&input);
        Ok(Self { maze })
    }

    fn parse_maze(input: &[String]) -> Vec<Vec<Elem>> {
        let mut maze = Vec::new();
        input.iter().for_each(|line| {
            let row = line
                .chars()
                .map(|ch| match ch {
                    '#' => Elem::Wall,
                    '.' => Elem::Empty,
                    'E' => Elem::Elf(INITIAL_HIT_POINTS),
                    'G' => Elem::Goblin(INITIAL_HIT_POINTS),
                    _ => panic!("Unexpected char '{}'", ch),
                })
                .collect::<Vec<Elem>>();
            maze.push(row);
        });
        maze
    }
}

impl Solution for AoC2018_15 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 15: Beverage Bandits".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_15_input_load_test() -> io::Result<()> {
        let sol = AoC2018_15::new()?;
        assert!(!sol.maze.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_15_correctness() -> io::Result<()> {
        let sol = AoC2018_15::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
