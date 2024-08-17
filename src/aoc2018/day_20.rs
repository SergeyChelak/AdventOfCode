use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

type Expression = Vec<char>;
type Coordinate = Point2d<isize>;

pub struct AoC2018_20 {
    input: Expression,
}

impl AoC2018_20 {
    pub fn new() -> io::Result<Self> {
        let input = read_file_as_chars("input/aoc2018_20")?;
        Ok(Self { input })
    }
}

impl Solution for AoC2018_20 {
    fn part_one(&self) -> String {
        let map = build_map(&self.input);
        map.values().max().unwrap().to_string()
    }

    fn part_two(&self) -> String {
        let map = build_map(&self.input);
        map.values().filter(|x| **x >= 1000).count().to_string()
    }

    fn description(&self) -> String {
        "AoC 2018/Day 20: A Regular Map".to_string()
    }
}

fn build_map(regex: &Expression) -> HashMap<Coordinate, usize> {
    let mut current = Coordinate::new(0, 0);
    let mut map = HashMap::<Coordinate, usize>::new();
    map.insert(current, 0);
    let mut stack = Vec::<Coordinate>::new();
    for ch in regex {
        match ch {
            '(' => stack.push(current),
            ')' => current = stack.pop().unwrap(),
            '|' => current = *stack.last().unwrap(),
            _ => {
                if let Some(next) = move_coordinate(current, *ch) {
                    let dist = map.get(&current).unwrap() + 1;
                    let prev_dist = map.entry(next).or_insert(dist);
                    *prev_dist = dist.min(*prev_dist);
                    current = next;
                }
            }
        }
    }
    map
}

fn move_coordinate(mut coordinate: Coordinate, direction: char) -> Option<Coordinate> {
    match direction {
        'N' => {
            coordinate.y -= 1;
        }
        'S' => {
            coordinate.y += 1;
        }
        'W' => {
            coordinate.x -= 1;
        }
        'E' => {
            coordinate.x += 1;
        }
        _ => return None,
    }
    Some(coordinate)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_20_input_load_test() -> io::Result<()> {
        let sol = AoC2018_20::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_20_examples_part1() {
        let cases = [
            ("^WNE$", 3),
            ("^ENWWW(NEEE|SSE(EE|N))$", 10),
            ("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$", 18),
            ("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$", 23),
            (
                "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$",
                31,
            ),
        ];
        for (input, output) in cases {
            let input = input.chars().collect::<Vec<char>>();
            let sol = AoC2018_20 { input };
            assert_eq!(sol.part_one(), output.to_string());
        }
    }

    #[test]
    fn aoc2018_20_correctness() -> io::Result<()> {
        let sol = AoC2018_20::new()?;
        assert_eq!(sol.part_one(), "3568");
        assert_eq!(sol.part_two(), "8475");
        Ok(())
    }
}
