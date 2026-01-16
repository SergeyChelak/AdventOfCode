use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, VecDeque};
use std::io;

type Int = usize;
type Point = Point2d<Int>;

// - 0123456789012
// 0 #############
// 1 #...........#
// 2 ###B#C#B#D###
// 3   #A#D#C#A#

type RawState = [(char, Point); 8];

fn min_energy(initial: RawState) -> usize {
    let mut costs = HashMap::<RawState, usize>::new();
    costs.insert(initial, 0);

    let mut min_cost = usize::MAX;
    let all_positions = all_positions();
    let mut queue = VecDeque::<RawState>::new();
    queue.push_front(initial);
    while let Some(raw) = queue.pop_back() {
        let state = AmphipodState::with_raw(raw);
        let cost = *costs.get(&raw).unwrap();

        if cost >= min_cost {
            continue;
        }

        if state.is_organized() {
            min_cost = cost;
            continue;
        }

        for (name, from) in raw.iter() {
            for to in all_positions.iter() {
                if !state.can_move(*from, *to) {
                    continue;
                }
                let next = state.moved_by(*from, *to);
                let movement_cost =
                    (from.x.abs_diff(to.x) + from.y.abs_diff(to.y)) * step_cost(*name);

                let next_cost = movement_cost + cost;
                if next_cost >= min_cost {
                    continue;
                }
                let stored_cost = *costs.get(&next.raw).unwrap_or(&usize::MAX);
                if next_cost < stored_cost {
                    costs.insert(next.raw, next_cost);
                    queue.push_front(next.raw);
                }
            }
        }
    }
    min_cost
}

#[derive(Clone)]
struct AmphipodState {
    raw: RawState,
    p2i: HashMap<Point, usize>,
}

impl AmphipodState {
    fn with_array(array: Vec<(char, Point)>) -> Self {
        assert_eq!(8, array.len());
        let mut raw = [('_', Point::zero()); 8];
        array
            .iter()
            .zip(raw.iter_mut())
            .for_each(|(src, dest)| *dest = *src);

        Self::with_raw(raw)
    }

    fn with_raw(mut raw: RawState) -> Self {
        // raw.sort_by_key(|x| x.0);
        raw.sort_by(|(ch1, p1), (ch2, p2)| {
            ch1.cmp(ch2).then(p1.y.cmp(&p2.y).then(p1.x.cmp(&p2.x)))
        });

        let p2i = raw
            .iter()
            .enumerate()
            .map(|(i, val)| (val.1, i))
            .collect::<HashMap<Point, usize>>();
        assert_eq!(8, p2i.len());
        Self { raw, p2i }
    }

    fn can_move(&self, from: Point, to: Point) -> bool {
        // don't move to itself
        if from == to {
            return false;
        }
        // don't touch empty point
        let Some(name) = self.point_name(from) else {
            return false;
        };

        // don't move to occupied cell
        if self.p2i.contains_key(&to) {
            return false;
        }

        let mut path = Vec::<Point>::new();

        let is_hallway = from.y == 1;
        if is_hallway {
            // is correct slot
            let expected_x = 3 + 2 * (name as u8 - b'A') as usize;
            let allowed = [Point::new(expected_x, 2), Point::new(expected_x, 3)];
            if !allowed.contains(&to) {
                return false;
            }
            // isn't occupied with wrong amphipod
            if allowed
                .iter()
                .filter_map(|p| self.point_name(*p))
                .any(|p_name| p_name != name)
            {
                return false;
            }

            fill_path(
                &mut path,
                PlainInterval::new(expected_x, expected_x),
                PlainInterval::with_arbitrary(from.y, to.y),
            );
        } else {
            // can't jump
            if to.y > 1 && to.x != from.x {
                return false;
            }

            fill_path(
                &mut path,
                PlainInterval::new(from.x, from.x),
                PlainInterval::with_arbitrary(from.y, to.y),
            );
        }

        fill_path(
            &mut path,
            PlainInterval::with_arbitrary(from.x, to.x),
            PlainInterval::new(1, 1),
        );

        // if no items on the way
        for p in path {
            if p == from {
                continue;
            }
            if self.p2i.contains_key(&p) {
                return false;
            }
        }

        true
    }

    fn point_name(&self, point: Point) -> Option<char> {
        let index = self.p2i.get(&point)?;
        Some(self.raw[*index].0)
    }

    fn moved_by(&self, from: Point, to: Point) -> Self {
        let index = self.p2i.get(&from).expect("point must be set");
        let mut raw = self.raw;
        raw[*index].1 = to;
        Self::with_raw(raw)
    }

    fn is_organized(&self) -> bool {
        // check if there is no items in hallway
        if self.raw.iter().any(|(_, p)| p.y < 2) {
            return false;
        }
        for (i, chunk) in self.raw.chunks(2).enumerate() {
            let expected_x = i * 2 + 3;
            if chunk.iter().any(|(_, p)| p.x != expected_x) {
                return false;
            }
        }
        true
    }
}

fn all_positions() -> Vec<Point> {
    [
        "#############",
        "#..-.-.-.-..#",
        "###.#.#.#.###",
        "  #.#.#.#.#  ",
        "  #########  ",
    ]
    .iter()
    .enumerate()
    .flat_map(|(row, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, ch)| *ch == '.')
            .map(|(col, _)| Point::new(col, row))
            .collect::<Vec<_>>()
    })
    .collect()
}

fn fill_path(array: &mut Vec<Point>, x_rng: PlainInterval<usize>, y_rng: PlainInterval<usize>) {
    for x in x_rng.begin..=x_rng.end {
        for y in y_rng.begin..=y_rng.end {
            array.push(Point::new(x, y));
        }
    }
}

fn step_cost(ch: char) -> usize {
    let index = (ch as u8 - b'A') as usize;
    [1, 10, 100, 1000][index]
}

impl From<&[&str]> for AmphipodState {
    fn from(lines: &[&str]) -> Self {
        let array = lines
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(col, name)| {
                        if !('A'..='D').contains(&name) {
                            return None;
                        }
                        let pos = Point::new(col, row);
                        Some((name, pos))
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self::with_array(array)
    }
}

pub struct AoC2021_23 {
    state: AmphipodState,
}

impl AoC2021_23 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2021_23")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let data = lines.iter().map(|x| x.as_ref()).collect::<Vec<_>>();
        Self {
            state: AmphipodState::from(data.as_ref()),
        }
    }
}

impl Solution for AoC2021_23 {
    fn part_one(&self) -> String {
        min_energy(self.state.raw).to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 23: Amphipod".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_23_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.state.raw.len(), 8);
        Ok(())
    }

    #[test]
    fn aoc2021_23_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "15385");
        Ok(())
    }

    #[test]
    fn aoc2021_23_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2021_23_is_organized() {
        assert!(!make_test_state().is_organized());
        assert!(make_organized_state().is_organized());
        assert!(!make_hallway_state().is_organized());
    }

    #[test]
    fn aoc2021_23_can_move_basic() {
        let state = make_test_state();

        // move to itself
        assert!(
            !state.can_move(Point::new(3, 2), Point::new(3, 2)),
            "shouldn't allow to move to itself"
        );
        // try to move empty point
        assert!(
            !state.can_move(Point::new(2, 1), Point::new(3, 2)),
            "shouldn't allow to move empty point"
        );
        //
        assert!(state.can_move(Point::new(3, 2), Point::new(1, 1)));
        assert!(state.can_move(Point::new(3, 2), Point::new(2, 1)));
        assert!(state.can_move(Point::new(3, 2), Point::new(4, 1)));
        assert!(state.can_move(Point::new(3, 2), Point::new(6, 1)));
        assert!(state.can_move(Point::new(3, 2), Point::new(8, 1)));
        assert!(state.can_move(Point::new(3, 2), Point::new(10, 1)));

        assert!(!state.can_move(Point::new(3, 3), Point::new(1, 1)));
        assert!(!state.can_move(Point::new(3, 3), Point::new(2, 1)));
        assert!(!state.can_move(Point::new(3, 3), Point::new(4, 1)));
        assert!(!state.can_move(Point::new(3, 3), Point::new(6, 1)));
        assert!(!state.can_move(Point::new(3, 3), Point::new(8, 1)));
        assert!(!state.can_move(Point::new(3, 3), Point::new(10, 1)));
    }

    #[test]
    fn aoc2021_23_can_move_hallway() {
        let state = make_hallway_state();
        assert!(state.can_move(Point::new(4, 1), Point::new(3, 2)));
        assert!(!state.can_move(Point::new(4, 1), Point::new(6, 2)));

        assert!(!state.can_move(Point::new(6, 1), Point::new(5, 2)));

        assert!(!state.can_move(Point::new(2, 1), Point::new(9, 2)));
    }

    fn make_solution() -> io::Result<AoC2021_23> {
        AoC2021_23::new()
    }

    fn make_test_solution() -> AoC2021_23 {
        AoC2021_23 {
            state: make_test_state(),
        }
    }

    fn make_test_state() -> AmphipodState {
        let lines = [
            "#############",
            "#...........#",
            "###B#C#B#D###",
            "  #A#D#C#A#  ",
            "  #########  ",
        ];
        AmphipodState::from(lines.as_ref())
    }

    fn make_organized_state() -> AmphipodState {
        let lines = [
            "#############",
            "#...........#",
            "###A#B#C#D###",
            "  #A#B#C#D#  ",
            "  #########  ",
        ];
        AmphipodState::from(lines.as_ref())
    }

    fn make_hallway_state() -> AmphipodState {
        let lines = [
            "#############",
            "#.D.A.B.....#",
            "###.#.#B#.###",
            "  #A#C#C#D#  ",
            "  #########  ",
        ];
        AmphipodState::from(lines.as_ref())
    }
}
