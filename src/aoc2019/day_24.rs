use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
use std::io;

const TILE_EMPTY: char = '.';
const TILE_BUG: char = '#';

pub struct AoC2019_24 {
    input: Grid,
}

impl AoC2019_24 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2019_24")?;
        // #[rustfmt::skip]
        // let lines = [
        //     "....#",
        //     "#..#.",
        //     "#..##",
        //     "..#..",
        //     "#....",
        // ];
        Ok(Self::with_lines(&lines))
    }

    fn with_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut input = Grid::empty();
        lines
            .iter()
            .map(|line| line.as_ref())
            .enumerate()
            .for_each(|(row, s)| {
                for (col, ch) in s.chars().enumerate() {
                    input[row][col] = ch;
                }
            });
        Self { input }
    }
}

impl Solution for AoC2019_24 {
    fn part_one(&self) -> String {
        let mut seen = HashSet::new();
        let mut next = self.input;
        let value = loop {
            let s = next.to_string();
            if seen.contains(&s) {
                break s;
            }
            seen.insert(s);
            next = generate_next_grid(&next);
        };

        let rating = value
            .chars()
            .enumerate()
            .filter(|(_, ch)| *ch == TILE_BUG)
            .map(|(pos, _)| 1 << pos)
            .sum::<usize>();

        rating.to_string()
    }

    fn part_two(&self) -> String {
        let mut grid_area = GridArea::with_grid(self.input);
        let duration = 200;
        for minute in 1..=duration {
            let mut tmp = GridArea::new();
            for i in -minute..=minute {
                let next = grid_area.next(i);
                tmp.insert(i, next);
            }
            grid_area = tmp;
        }

        grid_area.total_bugs().to_string()
    }

    fn description(&self) -> String {
        "Day 24: Planet of Discord".to_string()
    }
}

trait GridOps {
    fn empty() -> Grid;
    fn inner_count(&self, side: Direction) -> usize;
    fn outer_count(&self, side: Direction) -> usize;
    fn to_string(&self) -> String;

    fn adjacent_count(
        &self,
        x: isize,
        y: isize,
        direction: &Direction,
        prev: &Grid,
        next: &Grid,
    ) -> usize;
}

const GRID_SIZE: usize = 5;
type Grid = [[char; GRID_SIZE]; GRID_SIZE];

impl GridOps for Grid {
    fn empty() -> Grid {
        [[TILE_EMPTY; GRID_SIZE]; GRID_SIZE]
    }

    fn adjacent_count(
        &self,
        x: isize,
        y: isize,
        direction: &Direction,
        prev: &Grid,
        next: &Grid,
    ) -> usize {
        let point = Point2d::new(x, y).moved_by(direction);
        let dim = GRID_SIZE as isize;
        match (*direction, point.x, point.y) {
            // check for prev layer
            (_, _, -1) | (_, -1, _) => prev.inner_count(*direction),
            (_, _, y) if y == dim => prev.inner_count(*direction),
            (_, x, _) if x == dim => prev.inner_count(*direction),
            // check for next layer
            (_, 2, 2) => next.outer_count(*direction),
            // inside of self
            (_, col, row) => {
                if self[row as usize][col as usize] == TILE_BUG {
                    1
                } else {
                    0
                }
            }
        }
    }

    fn inner_count(&self, side: Direction) -> usize {
        let ch = match side {
            Direction::Up => self[1][2],
            Direction::Down => self[3][2],
            Direction::Left => self[2][1],
            Direction::Right => self[2][3],
        };
        if ch == TILE_BUG {
            1
        } else {
            0
        }
    }

    fn outer_count(&self, side: Direction) -> usize {
        let row_sum =
            |row: usize| -> usize { self[row].iter().filter(|ch| **ch == TILE_BUG).count() };

        let col_sum = |col: usize| -> usize {
            self.iter()
                .map(|row| row[col])
                .filter(|ch| *ch == TILE_BUG)
                .count()
        };

        match side {
            Direction::Up => row_sum(GRID_SIZE - 1),
            Direction::Down => row_sum(0),
            Direction::Left => col_sum(GRID_SIZE - 1),
            Direction::Right => col_sum(0),
        }
    }

    fn to_string(&self) -> String {
        self.iter().flatten().collect::<String>()
    }
}

type LevelIndex = isize;

struct GridArea {
    levels: HashMap<LevelIndex, Grid>,
}

impl GridArea {
    fn new() -> Self {
        Self {
            levels: HashMap::new(),
        }
    }

    fn with_grid(grid: Grid) -> Self {
        let mut area = Self::new();
        area.insert(0, grid);
        area
    }

    fn next(&self, level_idx: LevelIndex) -> Grid {
        let empty = Grid::empty();
        let prev_level = self.levels.get(&(level_idx - 1)).unwrap_or(&empty);
        let next_level = self.levels.get(&(level_idx + 1)).unwrap_or(&empty);
        let level = self.levels.get(&level_idx).unwrap_or(&empty);

        let mut grid = Grid::empty();
        for (y, row) in level.iter().enumerate() {
            for (x, ch) in row.iter().enumerate() {
                if x == 2 && y == 2 {
                    continue;
                }
                let bugs = Direction::all()
                    .iter()
                    .map(|dir| {
                        level.adjacent_count(x as isize, y as isize, dir, prev_level, next_level)
                    })
                    .sum::<usize>();
                grid[y][x] = match *ch {
                    TILE_BUG if bugs != 1 => TILE_EMPTY,
                    TILE_EMPTY if bugs == 1 || bugs == 2 => TILE_BUG,
                    _ => *ch,
                };
            }
        }
        grid
    }

    fn insert(&mut self, index: LevelIndex, grid: Grid) {
        assert!(!self.levels.contains_key(&index));
        self.levels.insert(index, grid);
    }

    fn total_bugs(&self) -> usize {
        self.levels
            .values()
            .flatten()
            .flatten()
            .filter(|ch| **ch == TILE_BUG)
            .count()
    }
}

type Position = Point2d<usize>;

fn generate_next_grid(input: &Grid) -> Grid {
    let mut result = Grid::empty();
    let rows = input.len();
    for (y, row) in input.iter().enumerate() {
        let cols = row.len();
        for (x, ch) in row.iter().enumerate() {
            let pos = Position::new(x, y);
            let bugs = Direction::all()
                .iter()
                .filter_map(|dir| pos.safe_moved_by(dir))
                .filter(|p| p.x < cols && p.y < rows)
                .filter(|p| (input[p.y].as_ref())[p.x] == TILE_BUG)
                .count();
            result[y][x] = match *ch {
                TILE_BUG if bugs != 1 => TILE_EMPTY,
                TILE_EMPTY if bugs == 1 || bugs == 2 => TILE_BUG,
                _ => *ch,
            };
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_24_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_24_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "28778811");
        Ok(())
    }

    #[test]
    fn aoc2019_24_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "2097");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_24> {
        AoC2019_24::new()
    }
}
