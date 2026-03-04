use crate::solution::Solution;

use std::io::{self};

pub struct AoC2022_17 {
    input: Vec<char>,
}

impl AoC2022_17 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2022_17")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        let input = data
            .trim()
            .chars()
            .inspect(|ch| assert!(*ch == '<' || *ch == '>'))
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2022_17 {
    fn part_one(&self) -> String {
        let shapes = make_shapes();
        simulate(&shapes, &self.input, 2022).to_string()
    }

    fn part_two(&self) -> String {
        let shapes = make_shapes();
        simulate(&shapes, &self.input, 1000000000000).to_string()
    }

    fn description(&self) -> String {
        "Day 17: Pyroclastic Flow".to_string()
    }
}

const SHAPE_HEIGHT: usize = 4;
const APPEARANCE_OFFSET: usize = 3;

type Shape = [u8; SHAPE_HEIGHT];

fn simulate(shapes: &[Shape], jet_pattern: &[char], rocks: usize) -> usize {
    let capacity = APPEARANCE_OFFSET + SHAPE_HEIGHT * rocks;
    let mut chamber = vec![0; capacity];

    let mut shape_idx = 0;
    let mut jet_idx = 0;

    let mut top = 0;
    for _ in 0..rocks {
        // spawn the rock
        let mut height = top + APPEARANCE_OFFSET;
        let mut shape = shapes[shape_idx];
        shape_idx = (shape_idx + 1) % shapes.len();

        let mut can_move_down = true;
        while can_move_down {
            // push by jet
            process_jet_movement(
                jet_pattern[jet_idx],
                &mut shape,
                &chamber[height..height + SHAPE_HEIGHT],
            );
            jet_idx = (jet_idx + 1) % jet_pattern.len();

            // move down
            if height > 0 {
                let tmp_h = height - 1;
                can_move_down = is_applicable(&shape, &chamber[tmp_h..tmp_h + SHAPE_HEIGHT])
            } else {
                can_move_down = false;
            }

            if can_move_down {
                height -= 1;
            }
        }
        // store the rock
        chamber
            .iter_mut()
            .skip(height)
            .zip(shape.iter())
            .for_each(|(c, s)| *c |= *s);

        while chamber[top] > 0 {
            top += 1;
        }
    }

    top
}

fn process_jet_movement(movement: char, shape: &mut Shape, chamber: &[u8]) {
    let mut next = *shape;
    match movement {
        '<' if check_shl_bounds(shape) => next.iter_mut().for_each(|x| *x <<= 1),
        '>' if check_shr_bounds(shape) => next.iter_mut().for_each(|x| *x >>= 1),
        _ => {
            return;
        }
    };
    if is_applicable(&next, chamber) {
        *shape = next;
    }
}

fn is_applicable(shape: &Shape, chamber: &[u8]) -> bool {
    assert_eq!(shape.len(), chamber.len());
    shape
        .iter()
        .zip(chamber.iter())
        .all(|(s, c)| *s | *c == *s ^ *c)
}

fn check_shl_bounds(shape: &[u8]) -> bool {
    shape.iter().all(|x| x & (1 << 6) == 0)
}

fn check_shr_bounds(shape: &[u8]) -> bool {
    shape.iter().all(|x| x & 1 == 0)
}

#[rustfmt::skip]
fn make_shapes() -> [Shape; 5]{
    [
        [
        //    1234567
            0b0011110,
            0b0000000,
            0b0000000,
            0b0000000,
        ],
        [
            0b0001000,
            0b0011100,
            0b0001000,
            0b0000000,
        ],
        [
            0b0011100,
            0b0000100,
            0b0000100,
            0b0000000,
        ],
        [
            0b0010000,
            0b0010000,
            0b0010000,
            0b0010000,
        ],
        [
            0b0011000,
            0b0011000,
            0b0000000,
            0b0000000,
        ]
    ]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_17_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2022_17_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "3197");
        Ok(())
    }

    #[test]
    fn aoc2022_17_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2022_17_case_1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "3068")
    }

    fn make_solution() -> io::Result<AoC2022_17> {
        AoC2022_17::new()
    }

    fn make_test_solution() -> AoC2022_17 {
        AoC2022_17::parse_data(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>")
    }
}
