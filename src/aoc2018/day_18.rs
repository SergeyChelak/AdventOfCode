use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
enum Acre {
    OpenGround,
    Trees,
    Lumberyard,
}

impl From<char> for Acre {
    fn from(value: char) -> Self {
        match value {
            '.' => Acre::OpenGround,
            '|' => Acre::Trees,
            '#' => Acre::Lumberyard,
            _ => panic!("invalid input"),
        }
    }
}

type Area = Vec<Vec<Acre>>;

pub struct AoC2018_18 {
    input: Area,
}

impl AoC2018_18 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2018_18")?;
        let input = parse_lines(&lines);
        Ok(Self { input })
    }

    fn compute(&self, minutes: usize) -> usize {
        let mut state = self.input.clone();
        (0..minutes).for_each(|_| state = update(&state));
        resource_value(&state)
    }
}

impl Solution for AoC2018_18 {
    fn part_one(&self) -> String {
        self.compute(10).to_string()
    }

    fn part_two(&self) -> String {
        let mut hashes = HashMap::<u64, usize>::new();
        let minutes = 1_000_000_000;
        let mut state = self.input.clone();
        let mut values = Vec::new();
        let mut offset = 0;
        let mut period = 0;
        for step in 0..minutes {
            let hash = hash(&state);
            if let Some(prev) = hashes.get(&hash) {
                println!("found a loop at {}, prev step {}", step, prev);
                period = step - prev;
                offset = *prev;
                break;
            }
            let value = resource_value(&state);
            values.push(value);
            hashes.insert(hash, step);
            state = update(&state);
        }
        let pos = (minutes - offset) % period;
        values[pos + offset].to_string()
    }

    fn description(&self) -> String {
        "AoC 2018/Day 18: Settlers of The North Pole".to_string()
    }
}

fn parse_lines(lines: &[String]) -> Area {
    let mut result = Vec::with_capacity(lines.len());
    lines.iter().for_each(|array| {
        let mut row = Vec::with_capacity(array.len());
        array.chars().for_each(|value| {
            let acre = Acre::from(value);
            row.push(acre);
        });
        result.push(row);
    });
    result
}

fn update(input: &Area) -> Area {
    let mut adj = Vec::<Acre>::with_capacity(8);
    let mut output = input.clone();
    let rows = input.len();
    for (row, arr) in input.iter().enumerate() {
        let cols = arr.len();
        for (col, value) in arr.iter().enumerate() {
            adj.clear();
            let up = row > 0;
            let left = col > 0;
            let down = row < rows - 1;
            let right = col < cols - 1;
            if up {
                adj.push(input[row - 1][col])
            }
            if down {
                adj.push(input[row + 1][col])
            }
            if left {
                adj.push(input[row][col - 1])
            }
            if right {
                adj.push(input[row][col + 1])
            }
            if up && left {
                adj.push(input[row - 1][col - 1])
            }
            if up && right {
                adj.push(input[row - 1][col + 1])
            }
            if down && left {
                adj.push(input[row + 1][col - 1])
            }
            if down && right {
                adj.push(input[row + 1][col + 1])
            }
            let (mut trees, mut lumberyard) = (0, 0);
            adj.iter().for_each(|x| match x {
                Acre::Trees => trees += 1,
                Acre::Lumberyard => lumberyard += 1,
                _ => {}
            });
            output[row][col] = next_acre(*value, trees, lumberyard);
        }
    }
    output
}

fn next_acre(current: Acre, trees: usize, lumberyard: usize) -> Acre {
    match current {
        // An open acre will become filled with trees if three or more adjacent acres contained trees
        Acre::OpenGround if trees > 2 => Acre::Trees,
        // An acre filled with trees will become a lumberyard if three or more adjacent acres were lumberyards
        Acre::Trees if lumberyard > 2 => Acre::Lumberyard,
        Acre::Lumberyard if lumberyard == 0 || trees == 0 => {
            // An acre containing a lumberyard will remain a lumberyard if it was adjacent to at least one other lumberyard and at least one acre containing trees.
            // Otherwise, it becomes open.
            Acre::OpenGround
        }
        _ => current,
    }
}

fn resource_value(input: &Area) -> usize {
    count(input, Acre::Trees) * count(input, Acre::Lumberyard)
}

fn count(input: &Area, value: Acre) -> usize {
    input.iter().flatten().filter(|&x| *x == value).count()
}

fn hash(area: &Area) -> u64 {
    let mut hasher = std::hash::DefaultHasher::new();
    area.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_18_input_load_test() -> io::Result<()> {
        let sol = AoC2018_18::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_18_test_case_1() {
        let lines = initial_state();
        let input = parse_lines(&lines);
        let sol = AoC2018_18 { input };
        assert_eq!(sol.part_one(), "1147")
    }

    #[test]
    fn aoc2018_18_transition_1() {
        let input = initial_state();
        let area = parse_lines(&input);
        let next = update(&area);
        let new_area = parse_lines(&min_1_state());
        assert_eq!(next, new_area)
    }

    #[test]
    fn aoc2018_18_resource_value() {
        let input = final_state();
        let area = parse_lines(&input);
        let val = resource_value(&area);
        assert_eq!(val, 1147);
    }

    #[test]
    fn aoc2018_18_correctness() -> io::Result<()> {
        let sol = AoC2018_18::new()?;
        assert_eq!(sol.part_one(), "480150");
        assert_eq!(sol.part_two(), "233020");
        Ok(())
    }

    fn convert(inp: &[&str]) -> Vec<String> {
        inp.iter().map(|x| x.to_string()).collect()
    }

    fn initial_state() -> Vec<String> {
        convert(&[
            ".#.#...|#.",
            ".....#|##|",
            ".|..|...#.",
            "..|#.....#",
            "#.#|||#|#|",
            "...#.||...",
            ".|....|...",
            "||...#|.#|",
            "|.||||..|.",
            "...#.|..|.",
        ])
    }

    fn min_1_state() -> Vec<String> {
        convert(&[
            ".......##.",
            "......|###",
            ".|..|...#.",
            "..|#||...#",
            "..##||.|#|",
            "...#||||..",
            "||...|||..",
            "|||||.||.|",
            "||||||||||",
            "....||..|.",
        ])
    }

    fn final_state() -> Vec<String> {
        convert(&[
            ".||##.....",
            "||###.....",
            "||##......",
            "|##.....##",
            "|##.....##",
            "|##....##|",
            "||##.####|",
            "||#####|||",
            "||||#|||||",
            "||||||||||",
        ])
    }

    fn dump(input: &Area) {
        input.iter().for_each(|arr| {
            arr.iter().for_each(|x| {
                let ch = match *x {
                    Acre::Lumberyard => "#",
                    Acre::Trees => "|",
                    Acre::OpenGround => ".",
                };
                print!("{ch}");
            });
            println!();
        })
    }
}
