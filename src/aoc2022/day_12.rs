use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet, VecDeque};
use std::io;

type Int = isize;
type Point = Point2d<Int>;

type Map = HashMap<Point, char>;

pub struct AoC2022_12 {
    map: Map,
    start: Point,
    end: Point,
}

impl AoC2022_12 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2022_12")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut map = lines
            .iter()
            .map(|s| s.as_ref())
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, ch)| (Point::new(col as Int, row as Int), ch))
                    .collect::<Vec<_>>()
            })
            .collect::<Map>();

        let find_point = |map: &Map, ch: char| -> Option<Point> {
            map.iter().find(|(_, v)| **v == ch).map(|(k, _)| k).cloned()
        };

        let start = find_point(&map, 'S').expect("Starting point not found");
        map.insert(start, 'a');

        let end = find_point(&map, 'E').expect("End point not found");
        map.insert(end, 'z');

        Self { map, start, end }
    }
}

impl Solution for AoC2022_12 {
    fn part_one(&self) -> String {
        bfs(&self.map, self.start, can_move_forward, |_map, p| {
            p == self.end
        })
        .map(|x| x.to_string())
        .unwrap_or(not_found())
    }

    fn part_two(&self) -> String {
        bfs(&self.map, self.end, can_move_backward, |_map, p| {
            self.map.get(&p) == Some(&'a')
        })
        .map(|x| x.to_string())
        .unwrap_or(not_found())
    }

    fn description(&self) -> String {
        "Day 12: Hill Climbing Algorithm".to_string()
    }
}

fn bfs(
    map: &Map,
    start: Point,
    can_move: impl Fn(char, char) -> bool,
    is_target: impl Fn(&Map, Point) -> bool,
) -> Option<usize> {
    struct Elem {
        point: Point,
        length: usize,
    }

    let mut dequeue = VecDeque::new();
    dequeue.push_front(Elem {
        point: start,
        length: 0,
    });

    let mut seen = HashSet::<Point>::new();

    while let Some(elem) = dequeue.pop_back() {
        if !seen.insert(elem.point) {
            continue;
        }

        let Some(value) = map.get(&elem.point) else {
            unreachable!()
        };

        if is_target(map, elem.point) {
            return Some(elem.length);
        }

        for dir in Direction::all() {
            let next_point = elem.point.moved_by(&dir);
            if seen.contains(&next_point) {
                continue;
            }
            let Some(next_value) = map.get(&next_point) else {
                continue;
            };

            if can_move(*value, *next_value) {
                dequeue.push_front(Elem {
                    point: next_point,
                    length: 1 + elem.length,
                });
            }
        }
    }
    None
}

fn can_move_forward(current: char, next: char) -> bool {
    (0..=current as u8 + 1).contains(&(next as u8))
}

fn can_move_backward(current: char, next: char) -> bool {
    can_move_forward(next, current)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_12_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.map.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2022_12_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "339");
        Ok(())
    }

    #[test]
    fn aoc2022_12_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "332");
        Ok(())
    }

    #[test]
    fn aoc2022_12_case_1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "31");
    }

    fn make_solution() -> io::Result<AoC2022_12> {
        AoC2022_12::new()
    }

    fn make_test_solution() -> AoC2022_12 {
        let input = ["Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi"];
        AoC2022_12::parse_lines(&input)
    }
}
