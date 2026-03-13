use crate::{
    solution::Solution,
    utils::{bounds, not_found, Direction, Point2d},
};

use std::{
    collections::{HashMap, HashSet},
    io,
};

type Int = i32;
type Point = Point2d<Int>;

pub struct AoC2022_23 {
    input: HashSet<Point>,
}

impl AoC2022_23 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2022_23")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        Self::parse_lines(&data.lines().collect::<Vec<_>>())
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .enumerate()
            .flat_map(|(row, s)| {
                s.trim()
                    .chars()
                    .enumerate()
                    .filter(|(_, ch)| *ch == '#')
                    .map(|(col, _)| Point::new(col as Int, row as Int))
                    .collect::<HashSet<_>>()
            })
            .collect::<HashSet<_>>();
        Self { input }
    }
}

impl Solution for AoC2022_23 {
    fn part_one(&self) -> String {
        let mut positions = self.input.clone();
        for step in 0..10 {
            let len_before = positions.len();
            let Some(p) = simulate_step(&positions, step) else {
                break;
            };
            positions = p;
            assert_eq!(len_before, positions.len(), "failed at step {step}");
        }
        calc_free_space(&positions)
            .map(|x| x.to_string())
            .unwrap_or(not_found())
    }

    fn part_two(&self) -> String {
        let mut positions = self.input.clone();
        for step in 0.. {
            let len_before = positions.len();
            let Some(p) = simulate_step(&positions, step) else {
                return (step + 1).to_string();
            };
            positions = p;
            assert_eq!(len_before, positions.len(), "failed at step {step}");
        }
        not_found()
    }

    fn description(&self) -> String {
        "Day 23: Unstable Diffusion".to_string()
    }
}

fn simulate_step(positions: &HashSet<Point>, step: usize) -> Option<HashSet<Point>> {
    let rules = [
        // If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
        (
            [
                vec![Direction::Up],
                vec![Direction::Up, Direction::Right],
                vec![Direction::Up, Direction::Left],
            ],
            Direction::Up,
        ),
        // If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
        (
            [
                vec![Direction::Down],
                vec![Direction::Down, Direction::Right],
                vec![Direction::Down, Direction::Left],
            ],
            Direction::Down,
        ),
        // If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
        (
            [
                vec![Direction::Left],
                vec![Direction::Left, Direction::Up],
                vec![Direction::Left, Direction::Down],
            ],
            Direction::Left,
        ),
        // If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step.
        (
            [
                vec![Direction::Right],
                vec![Direction::Right, Direction::Up],
                vec![Direction::Right, Direction::Down],
            ],
            Direction::Right,
        ),
    ];

    let mut attempt = HashMap::<Point, Point>::new();
    let mut point_count = HashMap::<Point, usize>::new();

    let mut result = HashSet::new();

    for p in positions.iter() {
        let all_empty = |directions: &[Vec<Direction>]| -> bool {
            directions
                .iter()
                .map(|directions| p.moved_with_dirs(directions))
                .all(|point| !positions.contains(&point))
        };
        // check if all empty
        let mut destination: Option<Point> = None;
        if !all_empty(&Direction::circular_directions()) {
            for rule_id in 0..rules.len() {
                let index = (rule_id + step) % rules.len();
                let (adj, dir) = &rules[index];
                if all_empty(adj) {
                    destination = Some(p.moved_by(dir));
                    break;
                }
            }
        }
        if let Some(d) = destination {
            *point_count.entry(d).or_insert(0) += 1;
            attempt.insert(*p, d);
        } else {
            result.insert(*p);
        }
    }

    if attempt.is_empty() {
        return None;
    }

    for (from, to) in attempt.into_iter() {
        if *point_count.get(&to).expect("missing mapping") == 1 {
            result.insert(to);
        } else {
            result.insert(from);
        }
    }

    Some(result)
}

fn calc_free_space(input: &HashSet<Point>) -> Option<usize> {
    let bounds = bounds(&input.iter().cloned().collect::<Vec<_>>())?;
    let square =
        (1 + bounds.high.x.abs_diff(bounds.low.x)) * (1 + bounds.high.y.abs_diff(bounds.low.y));
    Some(square as usize - input.len())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_23_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2022_23_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "4116");
        Ok(())
    }

    #[test]
    fn aoc2022_23_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "984");
        Ok(())
    }

    #[test]
    fn aoc2022_23_case_1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "110");
    }

    #[test]
    fn aoc2022_23_case_2() {
        let sol = make_test_solution();
        assert_eq!(sol.part_two(), "20");
    }

    fn make_solution() -> io::Result<AoC2022_23> {
        AoC2022_23::new()
    }

    #[rustfmt::skip]
    fn make_test_solution() -> AoC2022_23 {
        AoC2022_23::parse_lines(&[
            "..............",
            "..............",
            ".......#......",
            ".....###.#....",
            "...#...#.#....",
            "....#...##....",
            "...#.###......",
            "...##.#.##....",
            "....#..#......",
            "..............",
            "..............",
            "..............",
        ])
    }
}
