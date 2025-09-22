use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2020_05 {
    input: Vec<String>,
}

impl AoC2020_05 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2020_05")?;
        Ok(Self { input: lines })
    }
}

impl Solution for AoC2020_05 {
    fn part_one(&self) -> String {
        self.input
            .iter()
            .map(|s| Position::try_from(s.as_str()).unwrap())
            .map(|p| p.seat())
            .max()
            .map(|x| x.to_string())
            .unwrap_or(not_found())
    }

    fn part_two(&self) -> String {
        let max = 1usize << 10;
        let mut seats = vec![false; max];
        self.input
            .iter()
            .map(|s| Position::try_from(s.as_str()).unwrap())
            .map(|p| p.seat())
            .for_each(|idx| seats[idx] = true);
        for i in 1..max - 1 {
            if seats[i] {
                continue;
            }
            if seats[i - 1] && seats[i + 1] {
                return i.to_string();
            }
        }
        not_found()
    }

    fn description(&self) -> String {
        "Day 5: Binary Boarding".to_string()
    }
}

type Position = Point2d<usize>;

impl Position {
    fn seat(&self) -> usize {
        self.x + self.y * 8
    }
}

impl TryFrom<&str> for Position {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 10 {
            return Err("Invalid format");
        }
        let convert = |s: &str| -> Result<usize, Self::Error> {
            let mut result = 0;
            for ch in s.chars() {
                result <<= 1;
                match ch {
                    'F' | 'L' => continue,
                    'B' | 'R' => result |= 1,
                    _ => return Err("Unexpected character"),
                }
            }
            Ok(result)
        };
        let row = &value[..7];
        let col = &value[7..];
        Ok(Self {
            x: convert(col)?,
            y: convert(row)?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_05_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2020_05_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "913");
        Ok(())
    }

    #[test]
    fn aoc2020_05_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "717");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_05> {
        AoC2020_05::new()
    }

    #[test]
    fn aoc2020_05_parse() {
        [
            ("FBFBBFFRLR", Position::new(5, 44), 357),
            ("BFFFBBFRRR", Position::new(7, 70), 567),
            ("FFFBBBFRRR", Position::new(7, 14), 119),
            ("BBFFBBFRLL", Position::new(4, 102), 820),
        ]
        .into_iter()
        .for_each(|(s, val, seat)| {
            let p: Position = s.try_into().unwrap();
            assert_eq!(p, val);
            assert_eq!(seat, p.seat());
        });
    }
}
