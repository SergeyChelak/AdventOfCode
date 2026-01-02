use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

type Int = isize;
type Position = Point2d<usize>;

#[derive(Clone)]
struct Board {
    data: HashMap<Int, Position>,
    marked: HashMap<Int, bool>,
    filled_rows_count: Vec<usize>,
    filled_cols_count: Vec<usize>,
}

impl From<&str> for Board {
    fn from(value: &str) -> Self {
        let mut data = HashMap::<Int, Position>::new();
        let mut marked = HashMap::<Int, bool>::new();
        let mut rows = 0usize;
        let mut cols = 0usize;
        for (row, line) in value.split('\n').enumerate() {
            rows = rows.max(row);
            for (col, token) in line.split_ascii_whitespace().enumerate() {
                cols = cols.max(col);
                let value = token.parse::<Int>().expect("Board values must be integer");
                let pos = Position::new(col, row);
                data.insert(value, pos);
                marked.insert(value, false);
            }
        }
        Self {
            data,
            marked,
            filled_rows_count: vec![0; rows + 1],
            filled_cols_count: vec![0; cols + 1],
        }
    }
}

impl Board {
    fn put(&mut self, value: Int) {
        let (Some(pos), Some(marked)) = (
            self.data.get(&value).cloned(),
            self.marked.get(&value).copied(),
        ) else {
            return;
        };
        if marked {
            return;
        }
        self.marked.insert(value, true);
        self.filled_rows_count[pos.y] += 1;
        self.filled_cols_count[pos.x] += 1;
    }

    fn is_bingo(&self) -> bool {
        self.filled_rows_count.contains(&5) || self.filled_cols_count.contains(&5)
    }

    fn unmarked_sum(&self) -> Int {
        self.marked
            .iter()
            .filter(|(_, v)| !**v)
            .map(|(k, _)| *k)
            .sum()
    }
}

pub struct AoC2021_04 {
    numbers: Vec<Int>,
    boards: Vec<Board>,
}

impl AoC2021_04 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2021_04")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        let sections = data.split("\n\n").collect::<Vec<_>>();
        let numbers = Self::parse_numbers(sections[0]);
        let boards = sections
            .into_iter()
            .skip(1)
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(Board::from)
            .collect::<Vec<_>>();

        Self { numbers, boards }
    }

    fn parse_numbers(input: &str) -> Vec<Int> {
        input
            .split(',')
            .map(|x| x.parse::<Int>().expect("Invalid numbers format"))
            .collect()
    }
}

impl Solution for AoC2021_04 {
    fn part_one(&self) -> String {
        let mut boards = self.boards.clone();
        for value in self.numbers.iter() {
            for board in boards.iter_mut() {
                board.put(*value);
                if board.is_bingo() {
                    return (board.unmarked_sum() * *value).to_string();
                }
            }
        }
        not_found()
    }

    fn part_two(&self) -> String {
        let mut boards = self.boards.clone();
        let mut result = 0;
        for value in self.numbers.iter() {
            let mut wins = 0;
            for board in boards.iter_mut() {
                board.put(*value);
                let mut tmp = 0;
                if board.is_bingo() {
                    wins += 1;
                    tmp = if wins == 1 {
                        board.unmarked_sum() * *value
                    } else {
                        0
                    };
                }
                if wins == 1 {
                    result = tmp;
                }
            }
            boards.retain(|b| !b.is_bingo());
        }
        result.to_string()
    }

    fn description(&self) -> String {
        "Day 4: Giant Squid".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_04_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.numbers.is_empty());
        assert!(!sol.boards.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2021_04_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "35711");
        Ok(())
    }

    #[test]
    fn aoc2021_04_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "5586");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2021_04> {
        AoC2021_04::new()
    }
}
