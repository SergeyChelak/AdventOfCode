use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet, VecDeque};
use std::io;

type Int = isize;
type Point = Point2d<Int>;

type Map = HashMap<Point, char>;

pub struct AoC2022_12 {
    map: Map,
}

impl AoC2022_12 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2022_12")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let map = lines
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
        Self { map }
    }
}

impl Solution for AoC2022_12 {
    fn part_one(&self) -> String {
        let Some(start) = self
            .map
            .iter()
            .find(|(_, v)| **v == 'S')
            .map(|(k, _)| k)
            .cloned()
        else {
            return not_found();
        };
        bsf(&self.map, start, 'E')
            .map(|x| x.to_string())
            .unwrap_or(not_found())
    }

    fn part_two(&self) -> String {
        // I know this could be more efficient using caching or by pruning paths longer than the current minimum.
        // However, a 1047ms runtime is acceptable for a value that is only calculated once.
        // Brute force is sufficient here.
        let mut min_steps = usize::MAX;
        for (k, _) in self.map.iter().filter(|(_, v)| **v == 'a' || **v == 'S') {
            let Some(steps) = bsf(&self.map, *k, 'E') else {
                continue;
            };
            min_steps = min_steps.min(steps);
        }
        min_steps.to_string()
    }

    fn description(&self) -> String {
        "Day 12: Hill Climbing Algorithm".to_string()
    }
}

fn bsf(map: &Map, start: Point, target: char) -> Option<usize> {
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
        if seen.contains(&elem.point) {
            continue;
        }
        seen.insert(elem.point);

        let Some(value) = map
            .get(&elem.point)
            .map(|ch| if *ch == 'S' { &'a' } else { ch })
        else {
            unreachable!()
        };
        if *value == target {
            return Some(elem.length);
        }

        for dir in Direction::all() {
            let next_point = elem.point.moved_by(&dir);
            if seen.contains(&next_point) {
                continue;
            }
            let Some(next_value) = map
                .get(&next_point)
                .map(|ch| if *ch == 'E' { &'z' } else { ch })
            else {
                continue;
            };

            if (0..=*value as u8 + 1).contains(&(*next_value as u8)) {
                //if (*value as u8).abs_diff(*next_value as u8) <= 1 {
                dequeue.push_front(Elem {
                    point: next_point,
                    length: 1 + elem.length,
                });
            }
        }
    }
    None
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
        assert_eq!(sol.part_two(), "");
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
