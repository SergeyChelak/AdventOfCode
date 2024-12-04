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

    // fn part_two(&self) -> String {
    // }

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
        let sol = AoC2024_04::with_strings(&input);
        assert_eq!(sol.part_one(), "18")
    }

    #[test]
    fn aoc2024_04_correctness() -> io::Result<()> {
        let sol = AoC2024_04::new()?;
        col = c;
        assert_eq!(sol.part_one(), "2534");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
