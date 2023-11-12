use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
use std::io;

#[derive(Clone, Copy, Debug, PartialEq)]
enum UnitType {
    Elf,
    Goblin,
}

#[derive(Clone, Copy, Debug)]
enum Elem {
    Unit(UnitType, i32),
    Wall,
    Empty,
}

// Each unit, either Goblin or Elf, has 3 attack power and starts with 200 hit points.
const INITIAL_HIT_POINTS: i32 = 200;
const ATTACK_POWER: i32 = 3;

type Maze = Vec<Vec<Elem>>;
type Coordinate = Point2d<usize>;

struct Battlefield {
    maze: Maze,
}

impl Battlefield {
    fn new(maze: &Maze) -> Self {
        Self { maze: maze.clone() }
    }

    fn outcome(&mut self) -> i32 {
        let rounds = self.combat();
        let hp = self.total_hp();
        // don't take into account round in which combat ends
        (rounds - 1)* hp
    }

    /// result is a number of rounds
    fn combat(&mut self) -> i32 {
        let mut rounds = 0;
        loop {
            let mut has_moves = false;
            let units_coords = self.unit_positions();
            for pos in units_coords {
                if self.try_attack(pos) {
                    has_moves = true;
                } else if let Some(next_step) = self.next_step(pos) {
                    let Elem::Empty = self.get(&next_step) else {
                        panic!("Unexpected elem type for step: {:?}", self.get(&next_step))
                    };
                    self.set(&next_step, self.get(&pos));
                    self.set(&pos, Elem::Empty);

                    _ = self.try_attack(next_step);

                    has_moves = true;
                }
            }
            if has_moves {
                rounds += 1;
                continue;
            }
            break rounds;
        }
    }

    fn try_attack(&mut self, pos: Coordinate) -> bool {
        let Some(enemy_pos) = self.attack_position(pos) else {
            return false;
        };
        let Elem::Unit(elem_type, hp) = self.get(&enemy_pos) else {
            panic!(
                "Unexpected elem type for attack: {:?}",
                self.get(&enemy_pos)
            )
        };
        let new_hp = hp - ATTACK_POWER;
        let updated_item = if new_hp > 0 {
            Elem::Unit(elem_type, new_hp)
        } else {
            Elem::Empty
        };
        self.set(&enemy_pos, updated_item);
        true
    }

    fn total_hp(&self) -> i32 {
        self.maze
            .iter()
            .map(|row| {
                row.iter().fold(0, |acc, elem| {
                    acc + match elem {
                        Elem::Unit(_, hp) => 0.max(*hp),
                        _ => 0,
                    }
                })
            })
            .sum()
    }

    fn unit_positions(&self) -> Vec<Coordinate> {
        let mut result = Vec::new();
        for (i, row) in self.maze.iter().enumerate() {
            for (j, elem) in row.iter().enumerate() {
                if let Elem::Unit(_, _) = elem {
                    result.push(Coordinate::new(i, j));
                }
            }
        }
        result
    }

    fn next_step(&self, pos: Coordinate) -> Option<Coordinate> {
        let Elem::Unit(original_type, _) = self.get(&pos) else {
            return None;
        };
        let mut visited: HashSet<Coordinate> = HashSet::new();
        let mut cur = vec![pos];
        let mut enemy: Option<Coordinate> = None;
        let mut path: HashMap<Coordinate, Coordinate> = HashMap::new();
        loop {
            let mut next = vec![];
            for root in cur {
                visited.insert(root);
                self.get_adjacent(root, |elem| match elem {
                    Elem::Wall => false,
                    Elem::Empty => true,
                    Elem::Unit(unit_type, _) => *unit_type != original_type,
                })
                .iter()
                .filter(|p| !visited.contains(p))
                .for_each(|p| {
                    if let Elem::Unit(_, _) = self.get(p) {
                        enemy = enemy.or(Some(*p));
                    }
                    path.entry(*p).or_insert(root);
                    next.push(*p);
                });
            }
            if enemy.is_some() || next.is_empty() {
                break;
            }
            cur = next;
        }
        let mut enemy_pos = enemy?;
        while let Some(p) = path.get(&enemy_pos) {
            if path.get(p).is_some() {
                enemy_pos = *p;
            } else {
                break;
            }
        }
        Some(enemy_pos)
    }

    fn attack_position(&self, pos: Coordinate) -> Option<Coordinate> {
        let Elem::Unit(pos_type, _) = self.get(&pos) else {
            return None;
        };
        let adj = self.get_adjacent(pos, |elem| match elem {
            Elem::Empty | Elem::Wall => false,
            Elem::Unit(unit_type, _) => pos_type != *unit_type,
        });
        let Some(min) = adj
            .iter()
            .map(|p| match self.get(p) {
                Elem::Unit(_, hp) => hp,
                _ => panic!("Unexpected element type for attack"),
            })
            .min()
        else {
            return None;
        };
        adj.into_iter().find(|p| {
            let Elem::Unit(_, hp) = self.get(p) else {
                return false;
            };
            hp == min
        })
    }

    fn get_adjacent<T>(&self, pos: Coordinate, criteria: T) -> Vec<Coordinate>
    where
        T: Fn(&Elem) -> bool,
    {
        let mut result = Vec::new();
        let (x, y) = (pos.x, pos.y);
        // order is matter!
        if x > 0 {
            result.push(Coordinate::new(x - 1, y));
        }
        if y > 0 {
            result.push(Coordinate::new(x, y - 1));
        }
        if y < self.maze[x].len() - 1 {
            result.push(Coordinate::new(x, y + 1));
        }
        if x < self.maze.len() - 1 {
            result.push(Coordinate::new(x + 1, y));
        }
        result
            .iter()
            .filter(|p| criteria(&self.get(p)))
            .copied()
            .collect()
    }

    fn get(&self, pos: &Coordinate) -> Elem {
        self.maze[pos.x][pos.y]
    }

    fn set(&mut self, pos: &Coordinate, value: Elem) {
        self.maze[pos.x][pos.y] = value;
    }
}

pub struct AoC2018_15 {
    maze: Maze,
}

impl AoC2018_15 {
    pub fn new() -> io::Result<Self> {
        let input = read_file_as_lines("input/aoc2018_15")?;
        let maze = Self::parse_maze(&input);
        Ok(Self { maze })
    }

    fn parse_maze(input: &[String]) -> Maze {
        let mut maze = Maze::new();
        input.iter().for_each(|line| {
            let row = line
                .chars()
                .map(|ch| match ch {
                    '#' => Elem::Wall,
                    '.' => Elem::Empty,
                    'E' => Elem::Unit(UnitType::Elf, INITIAL_HIT_POINTS),
                    'G' => Elem::Unit(UnitType::Goblin, INITIAL_HIT_POINTS),
                    _ => panic!("Unexpected char '{}'", ch),
                })
                .collect::<Vec<Elem>>();
            maze.push(row);
        });
        maze
    }
}

impl Solution for AoC2018_15 {
    fn part_one(&self) -> String {
        let mut battlefiled = Battlefield::new(&self.maze);
        battlefiled.outcome().to_string()
    }

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
        assert_eq!(sol.part_one(), "196200");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2018_15_ex1_1() {
        #[rustfmt::skip]
        let input = [
            "#######", 
            "#G..#E#", 
            "#E#E.E#", 
            "#G.##.#", 
            "#...#E#", 
            "#...E.#", 
            "#######",
        ];
        test_part1(&input, "36334");
    }

    #[test]
    fn aoc2018_15_ex1_2() {
        #[rustfmt::skip]
        let input = [
            "#######",
            "#E..EG#",
            "#.#G.E#",
            "#E.##E#",
            "#G..#.#",
            "#..E#.#",
            "#######",
        ];
        test_part1(&input, "39514");
    }

    #[test]
    fn aoc2018_15_ex1_3() {
        #[rustfmt::skip]
        let input = [
            "#######",
            "#E.G#.#",
            "#.#G..#",
            "#G.#.G#",
            "#G..#.#",
            "#...E.#",
            "#######",
        ];
        test_part1(&input, "27755");
    }

    #[test]
    fn aoc2018_15_ex1_5() {
        #[rustfmt::skip]
        let input = [
            "#########",
            "#G......#",
            "#.E.#...#",
            "#..##..G#",
            "#...##..#",
            "#...#...#",
            "#.G...G.#",
            "#.....G.#",
            "#########",
        ];
        test_part1(&input, "18740");
    }

    fn test_part1(input: &[&str], expected: &str) {
        let src = input.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let maze = AoC2018_15::parse_maze(&src);
        let sol = AoC2018_15 { maze };
        assert_eq!(sol.part_one(), expected);
    }
}
