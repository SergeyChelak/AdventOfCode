use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Vec2<T> = Vec<Vec<T>>;

pub struct AoC2024_04 {
    input: Vec2<char>,
}

impl AoC2024_04 {
    pub fn new() -> io::Result<Self> {
        let strings = read_file_as_lines("input/aoc2024_04")?;
        Ok(Self::with_strings(&strings))
    }

    fn with_strings(arr: &[String]) -> Self {
        let input = arr
            .iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec2<char>>();
        Self { input }
    }
}

impl Solution for AoC2024_04 {
    fn part_one(&self) -> String {
        let mut total = 0;

        let directions = [
            (CounterState::Inc, CounterState::None),
            (CounterState::Dec, CounterState::None),
            (CounterState::None, CounterState::Inc),
            (CounterState::None, CounterState::Dec),
            (CounterState::Inc, CounterState::Inc),
            (CounterState::Inc, CounterState::Dec),
            (CounterState::Dec, CounterState::Inc),
            (CounterState::Dec, CounterState::Dec),
        ];

        for (row, arr) in self.input.iter().enumerate() {
            for (col, val) in arr.iter().enumerate() {
                if *val != 'X' {
                    continue;
                }
                total += directions
                    .iter()
                    .filter_map(|d| collect_string(&self.input, row, d.0, col, d.1))
                    // .inspect(|val| println!("{val}"))
                    .filter(|s| s == "XMAS")
                    .count();
            }
        }
        total.to_string()
    }

    fn part_two(&self) -> String {
        let mut total = 0;
        for (row, arr) in self.input.iter().enumerate() {
            if row == 0 || row == self.input.len() - 1 {
                continue;
            }
            for (col, val) in arr.iter().enumerate() {
                if col == 0 || col == arr.len() - 1 {
                    continue;
                }
                if *val != 'A' {
                    continue;
                }
                if is_x_pattern(&self.input, row, col) {
                    total += 1;
                }
            }
        }
        total.to_string()
    }

    fn description(&self) -> String {
        "2024/Day 4: Ceres Search".to_string()
    }
}

#[derive(Debug, Clone, Copy)]
enum CounterState {
    None,
    Inc,
    Dec,
}

fn collect_string(
    arr: &Vec2<char>,
    mut row: usize,
    row_state: CounterState,
    mut col: usize,
    col_state: CounterState,
) -> Option<String> {
    let mut output = Vec::new();

    let modify = |val: usize, max_val: usize, modifier: CounterState| -> Option<usize> {
        match modifier {
            CounterState::None => Some(val),
            CounterState::Dec if val > 0 => Some(val - 1),
            CounterState::Inc if val < max_val - 1 => Some(val + 1),
            _ => None,
        }
    };
    output.push(arr[row][col]);
    for _ in 0..3 {
        row = modify(row, arr.len(), row_state)?;
        col = modify(col, arr[row].len(), col_state)?;
        output.push(arr[row][col]);
    }
    Some(output.iter().collect::<String>())
}

fn is_x_pattern(arr: &Vec2<char>, row: usize, col: usize) -> bool {
    let up_left = arr[row - 1][col - 1];
    let up_right = arr[row - 1][col + 1];
    let down_left = arr[row + 1][col - 1];
    let down_right = arr[row + 1][col + 1];
    let check = |a: char, b: char| -> bool { a == 'M' && b == 'S' || b == 'M' && a == 'S' };
    check(up_left, down_right) && check(up_right, down_left)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_04_input_load_test() -> io::Result<()> {
        let sol = AoC2024_04::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2024_04_case_1() {
        let sol = puzzle();
        assert_eq!(sol.part_one(), "18")
    }

    #[test]
    fn aoc2024_04_case_2() {
        let sol = puzzle();
        assert_eq!(sol.part_two(), "9")
    }

    fn puzzle() -> AoC2024_04 {
        let input = [
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        AoC2024_04::with_strings(&input)
    }

    #[test]
    fn aoc2024_04_correctness() -> io::Result<()> {
        let sol = AoC2024_04::new()?;
        assert_eq!(sol.part_one(), "2534");
        assert_eq!(sol.part_two(), "1866");
        Ok(())
    }
}
