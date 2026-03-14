use crate::{
    solution::Solution,
    utils::{lcm, not_found, Direction, Point2d},
};

use std::{
    collections::{HashSet, VecDeque},
    io,
};

type Int = i32;
type Point = Point2d<Int>;
type BlizzardState = Vec<(Point2d<Int>, Direction)>;

pub struct AoC2022_24 {
    input: BlizzardState,
    rows: Int,
    cols: Int,
}

impl AoC2022_24 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2022_24")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        Self::parse_lines(&data.lines().collect::<Vec<_>>())
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let rows = lines.len() as Int;
        let cols = lines[0].as_ref().chars().count() as Int;

        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .enumerate()
            .flat_map(|(r, s)| {
                s.chars()
                    .enumerate()
                    .filter_map(|(c, ch)| {
                        let dir = parse_direction(ch)?;
                        Some((Point::new(c as Int, r as Int), dir))
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<BlizzardState>();
        Self { input, rows, cols }
    }
}

impl Solution for AoC2022_24 {
    fn part_one(&self) -> String {
        let from = Point::new(1, 0);
        let to = Point::new(self.cols - 2, self.rows - 1);
        calc_steps(&self.input, self.rows, self.cols, from, to)
            .map(|x| x.to_string())
            .unwrap_or(not_found())
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 24: Blizzard Basin".to_string()
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct StepState {
    count: usize,
    state_id: usize,
    position: Point,
}

struct BlizzardCache {
    state: BlizzardState,
    points: HashSet<Point>,
}

impl BlizzardCache {
    fn with(state: BlizzardState) -> Self {
        let points = state.iter().map(|(p, _)| *p).collect::<HashSet<_>>();
        Self { state, points }
    }
}

fn calc_steps(
    initial: &BlizzardState,
    rows: Int,
    cols: Int,
    from: Point,
    target: Point,
) -> Option<usize> {
    let mut state_list = vec![BlizzardCache::with(initial.clone())];

    let cycle_len = lcm(rows as usize - 2, cols as usize - 2);

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_front(StepState {
        count: 0,
        state_id: 0,
        position: from,
    });

    while let Some(step_state) = queue.pop_back() {
        let next_state_id = (step_state.state_id + 1) % cycle_len;
        if state_list.len() <= next_state_id {
            let cur_state = &state_list[step_state.state_id];
            let s = calc_next_state(&cur_state.state, rows, cols);
            state_list.push(BlizzardCache::with(s));
        }
        let occupied = &state_list[next_state_id].points;

        let next_count = step_state.count + 1;
        for point in Direction::all()
            .iter()
            .map(|dir| step_state.position.moved_by(dir))
            .chain([step_state.position])
        {
            if point == target {
                return Some(next_count);
            }

            if point != from
                && (point.x <= 0 || point.y <= 0 || point.x >= cols - 1 || point.y >= rows - 1)
            {
                continue;
            }

            if occupied.contains(&point) {
                continue;
            }

            if seen.insert((point, next_state_id)) {
                let next = StepState {
                    count: next_count,
                    state_id: next_state_id,
                    position: point,
                };
                queue.push_front(next);
            }
        }
    }

    None
}

fn calc_next_state(state: &BlizzardState, rows: Int, cols: Int) -> BlizzardState {
    let mut next_state = BlizzardState::new();
    for (point, dir) in state.iter() {
        let mut next_point = point.moved_by(dir);
        // wrap
        if next_point.x == 0 {
            next_point.x = cols - 2;
        }
        if next_point.x == cols - 1 {
            next_point.x = 1;
        }
        if next_point.y == 0 {
            next_point.y = rows - 2;
        }
        if next_point.y == rows - 1 {
            next_point.y = 1;
        }
        assert!(next_point.x > 0 && next_point.x < cols - 1);
        assert!(next_point.y > 0 && next_point.y < rows - 1);
        next_state.push((next_point, *dir));
    }
    next_state
}

fn parse_direction(ch: char) -> Option<Direction> {
    match ch {
        '^' => Some(Direction::Up),
        'v' => Some(Direction::Down),
        '<' => Some(Direction::Left),
        '>' => Some(Direction::Right),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_24_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        assert!(sol.rows > 0);
        assert!(sol.cols > 0);
        Ok(())
    }

    #[test]
    fn aoc2022_24_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "290");
        Ok(())
    }

    #[test]
    fn aoc2022_24_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2022_24_case_1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "18")
    }

    fn make_solution() -> io::Result<AoC2022_24> {
        AoC2022_24::new()
    }

    #[rustfmt::skip]
    fn make_test_solution() -> AoC2022_24 {
        AoC2022_24::parse_lines(&[
            "#.######",
            "#>>.<^<#",
            "#.<..<<#",
            "#>v.><>#",
            "#<^v^^>#",
            "######.#",
        ])
    }
}
