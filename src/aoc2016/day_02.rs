use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2016_02 {
    lines: Vec<String>,
}

impl AoC2016_02 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2016_02")?;
        Ok(Self { lines })
    }
}

impl Solution for AoC2016_02 {
    fn part_one(&self) -> String {
        let mut pos = (1, 1);
        let mut output = String::new();
        for line in &self.lines {
            let (r, c) = line.chars().fold(pos, |(r, c), ch| match ch {
                'L' => (r, 0.max(c - 1)),
                'R' => (r, 2.min(c + 1)),
                'U' => (0.max(r - 1), c),
                'D' => (2.min(r + 1), c),
                _ => panic!("Unexpected char {ch}"),
            });
            pos = (r, c);
            let val = r * 3 + c + 1;
            output.push_str(&val.to_string());
        }
        output
    }

    fn part_two(&self) -> String {
        let x = '\0';
        let keypad = [
            vec![x, x, '1', x, x],
            vec![x, '2', '3', '4', x],
            vec!['5', '6', '7', '8', '9'],
            vec![x, 'A', 'B', 'C', x],
            vec![x, x, 'D', x, x],
        ];
        let range = 0..keypad.len() as i32;
        let mut output = String::new();
        let mut pos = (2, 0);
        for line in &self.lines {
            for ch in line.chars() {
                let (mut r, mut c) = pos;
                match ch {
                    'L' => c -= 1,
                    'R' => c += 1,
                    'U' => r -= 1,
                    'D' => r += 1,
                    _ => panic!("Unexpected char {ch}"),
                };
                if range.contains(&r) && range.contains(&c) && keypad[r as usize][c as usize] != x {
                    pos = (r, c);
                }
            }
            output.push(keypad[pos.0 as usize][pos.1 as usize]);
        }
        output
    }

    fn description(&self) -> String {
        "AoC 2016/Day 2: Bathroom Security".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_02_input_load_test() -> io::Result<()> {
        let sol = AoC2016_02::new()?;
        assert!(!sol.lines.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2016_02_correctness() -> io::Result<()> {
        let sol = AoC2016_02::new()?;
        assert_eq!(sol.part_one(), "53255");
        assert_eq!(sol.part_two(), "7423A");
        Ok(())
    }

    #[test]
    fn aoc2016_02_demo_case1() {
        let sol = example_input();
        assert_eq!(sol.part_one(), "1985");
    }

    #[test]
    fn aoc2016_02_demo_case2() {
        let sol = example_input();
        assert_eq!(sol.part_two(), "5DB3");
    }

    fn example_input() -> AoC2016_02 {
        AoC2016_02 {
            lines: vec![
                String::from("ULL"),
                String::from("RRDDD"),
                String::from("LURDL"),
                String::from("UUUUD"),
            ],
        }
    }
}
