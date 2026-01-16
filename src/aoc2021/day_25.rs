use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2021_25 {
    input: Vec2<char>,
}

impl AoC2021_25 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2021_25")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|s| s.as_ref())
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2021_25 {
    fn part_one(&self) -> String {
        let mut buffer_1 = self.input.clone();
        let mut buffer_2 = self.input.clone();

        let mut input = &mut buffer_1;
        let mut output = &mut buffer_2;

        for step in 1.. {
            let east_moved = make_movement(EAST, &input, &mut output);
            let south_moved = make_movement(SOUTH, &output, &mut input);
            if !east_moved && !south_moved {
                return step.to_string();
            }
            // mem::swap(&mut input, &mut output);
        }
        not_found()
    }

    fn description(&self) -> String {
        "Day 25: Sea Cucumber".to_string()
    }
}

const EAST: char = '>';
const SOUTH: char = 'v';
const EMPTY: char = '.';

fn make_movement(template: char, input: &Vec2<char>, output: &mut Vec2<char>) -> bool {
    let (d_row, d_col) = match template {
        EAST => (0, 1),
        SOUTH => (1, 0),
        _ => unreachable!(),
    };

    let mut moved = Vec::new();

    let rows = input.len();
    for (row, array) in input.iter().enumerate() {
        let cols = array.len();
        for (col, ch) in array.iter().enumerate() {
            output[row][col] = *ch;
            if *ch != template {
                continue;
            }
            let (n_row, n_col) = ((row + d_row) % rows, (col + d_col) % cols);
            if input[n_row][n_col] == EMPTY {
                moved.push((row, col, n_row, n_col));
            }
        }
    }
    if moved.is_empty() {
        return false;
    }

    for (row, col, n_row, n_col) in moved {
        output[row][col] = EMPTY;
        output[n_row][n_col] = template;
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_25_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2021_25_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "321");
        Ok(())
    }

    #[test]
    fn aoc2021_25_case_1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "58");
    }

    fn make_solution() -> io::Result<AoC2021_25> {
        AoC2021_25::new()
    }

    fn make_test_solution() -> AoC2021_25 {
        let lines = [
            "v...>>.vv>",
            ".vv>>.vv..",
            ">>.>v>...v",
            ">>v>>.>.v.",
            "v>v.vv.v..",
            ">.>>..v...",
            ".vv..>.>v.",
            "v.v..>>v.v",
            "....v..v.>",
        ];
        AoC2021_25::parse_lines(&lines)
    }
}
