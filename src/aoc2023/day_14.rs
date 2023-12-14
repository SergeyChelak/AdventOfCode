use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

pub struct AoC2023_14 {
    input: Vec<Vec<char>>,
}

impl AoC2023_14 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_14")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let input = lines
            .iter()
            .map(|row| row.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2023_14 {
    fn part_one(&self) -> String {
        let mut platform = self.input.clone();
        slide(Direction::North, &mut platform);
        total_load(&platform).to_string()
    }

    fn part_two(&self) -> String {
        let cycles = 1000000000_usize;
        let mut platform = self.input.clone();
        let mut load = Vec::new();
        let mut map = HashMap::new();
        for i in 0..cycles {
            // north, then west, then south, then east
            slide(Direction::North, &mut platform);
            slide(Direction::West, &mut platform);
            slide(Direction::South, &mut platform);
            slide(Direction::East, &mut platform);
            if let Some(from) = map.get(&platform) {
                let in_loop: &[usize] = &load[*from..];
                return in_loop[(cycles - i - 1) % in_loop.len()].to_string();
            }
            load.push(total_load(&platform));
            map.insert(platform.clone(), i);
        }

        "Age later...".to_string()
    }

    fn description(&self) -> String {
        "AoC 2023/Day 14: Parabolic Reflector Dish".to_string()
    }
}

fn slide(direction: Direction, platform: &mut Vec<Vec<char>>) {
    let rows = platform.len();
    for row in 0..rows {
        let cols = platform[row].len();
        for col in 0..cols {
            let mut r = row;
            let mut c = col;
            // remap current row, cols
            if direction == Direction::South {
                r = rows - row - 1
            };
            if direction == Direction::East {
                c = cols - col - 1
            }
            if platform[r][c] != 'O' {
                continue;
            }

            loop {
                let prev_r = r;
                let prev_c = c;
                match direction {
                    Direction::North if r > 0 => {
                        r -= 1;
                    }
                    Direction::South if r < rows - 1 => {
                        r += 1;
                    }
                    Direction::West if c > 0 => {
                        c -= 1;
                    }
                    Direction::East if c < cols - 1 => {
                        c += 1;
                    }
                    _ => {}
                }
                if platform[r][c] != '.' {
                    break;
                }
                platform[prev_r][prev_c] = '.';
                platform[r][c] = 'O';
            }
        }
    }
}

fn _dump(input: &[Vec<char>]) {
    for row in input {
        for ch in row {
            print!("{ch}");
        }
        println!();
    }
}

#[allow(clippy::needless_range_loop)]
fn total_load(input: &[Vec<char>]) -> usize {
    let mut total = 0;
    for col in 0..input[0].len() {
        let len = input.len();
        for row in 0..len {
            if input[row][col] == 'O' {
                total += len - row;
            }
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_14_input_load_test() -> io::Result<()> {
        let sol = AoC2023_14::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_14_ex1() {
        assert_eq!(puzzle().part_one(), "136");
    }

    #[test]
    fn aoc2023_14_ex2() {
        assert_eq!(puzzle().part_two(), "64");
    }

    fn puzzle() -> AoC2023_14 {
        let input = [
            "O....#....",
            "O.OO#....#",
            ".....##...",
            "OO.#O....O",
            ".O.....O#.",
            "O.#..O.#.#",
            "..O..#O..O",
            ".......O..",
            "#....###..",
            "#OO..#....",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        AoC2023_14::with_lines(&input)
    }

    #[test]
    fn aoc2023_14_correctness() -> io::Result<()> {
        let sol = AoC2023_14::new()?;
        assert_eq!(sol.part_one(), "109385");
        assert_eq!(sol.part_two(), "93102");
        Ok(())
    }
}
