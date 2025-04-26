use crate::{solution::Solution, utils::Point2d};

use std::{collections::HashMap, fs::read_to_string, io};

use super::intcode_computer::*;

const TILE_EMPTY: Int = 0;
const TILE_WALL: Int = 1;
const TILE_BLOCK: Int = 2;
const TILE_PADDLE: Int = 3;
const TILE_BALL: Int = 4;

const IS_ANIMATION_ENABLED: bool = false;

type Pixel = Point2d<Int>;

pub struct AoC2019_13 {
    input: Memory,
}

impl AoC2019_13 {
    pub fn new() -> io::Result<Self> {
        let line = read_to_string("input/aoc2019_13")?;
        Ok(Self::with_str(&line))
    }

    fn with_str(input: &str) -> Self {
        Self {
            input: parse_program(input),
        }
    }
}

impl Solution for AoC2019_13 {
    fn part_one(&self) -> String {
        let mut computer = IntcodeComputer::with_size(2 * self.input.len());
        computer.load_program(&self.input);
        let status = computer.run();
        assert!(matches!(status, ExecutionStatus::Halted));
        let display = build_display(&computer);
        blocks_amount(&display).to_string()
    }

    fn part_two(&self) -> String {
        let mut memory = self.input.clone();
        // set 2 to play for free
        memory[0] = 2;
        let mut computer = IntcodeComputer::with_size(2 * memory.len());
        computer.load_program(&memory);
        loop {
            let status = computer.run();
            assert!(!matches!(status, ExecutionStatus::WrongInstruction { .. }));
            let display = build_display(&computer);

            if IS_ANIMATION_ENABLED {
                let image = display_image(&display);
                println!("{}\n", image);
            }

            let blocks = blocks_amount(&display);
            if blocks == 0 {
                break *display
                    .get(&Pixel::new(-1, 0))
                    .expect("Score value must be stored");
            }
            if matches!(status, ExecutionStatus::WaitForInput) {
                let (ball, paddle) = sprites_position(&display);
                let command = match (ball.x, paddle.x) {
                    (b, p) if b > p => 1,
                    (b, p) if b < p => -1,
                    _ => 0,
                };
                computer.push_input(command);
            }
        }
        .to_string()
    }

    fn description(&self) -> String {
        "Day 13: Care Package".to_string()
    }
}

type PixelDisplay = HashMap<Pixel, Int>;

fn build_display(computer: &IntcodeComputer) -> PixelDisplay {
    let mut display = HashMap::new();
    for chunk in computer.outputs().chunks(3) {
        let point = Pixel::new(chunk[0], chunk[1]);
        let tile = chunk[2];
        display.insert(point, tile);
    }
    display
}

fn display_image(display: &PixelDisplay) -> String {
    let max_x = display.keys().map(|p| p.x).max().unwrap_or_default();
    let max_y = display.keys().map(|p| p.y).max().unwrap_or_default();
    let mut result = String::new();
    for y in 0..=max_y {
        for x in 0..=max_x {
            let pixel = Pixel::new(x, y);
            let value = display.get(&pixel).unwrap_or(&TILE_EMPTY);
            let ch = match *value {
                TILE_EMPTY => ' ',
                TILE_WALL => '#',
                TILE_BLOCK => 'X',
                TILE_PADDLE => '=',
                TILE_BALL => 'O',
                _ => panic!("Unexpected display values {}", value),
            };
            result.push(ch);
        }
        result.push('\n');
    }
    result
}

fn blocks_amount(display: &PixelDisplay) -> usize {
    display.iter().filter(|(_, v)| **v == TILE_BLOCK).count()
}

fn sprites_position(display: &PixelDisplay) -> (Pixel, Pixel) {
    let mut ball = Pixel::zero();
    let mut paddle = Pixel::zero();
    display
        .iter()
        .filter(|(_, v)| **v == TILE_PADDLE || **v == TILE_BALL)
        .for_each(|(p, v)| {
            if *v == TILE_BALL {
                ball = *p;
            }
            if *v == TILE_PADDLE {
                paddle = *p;
            }
        });
    (ball, paddle)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_13_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_13_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "432");
        Ok(())
    }

    #[test]
    fn aoc2019_13_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "22225");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_13> {
        AoC2019_13::new()
    }
}
