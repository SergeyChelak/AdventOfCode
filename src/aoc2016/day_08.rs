use crate::solution::Solution;
use crate::utils::*;

use std::io;

enum Operation {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

impl Operation {
    fn from_str(s: &str) -> Self {
        let (cmd, rest) = s.split_once(' ').expect("Incorrect input format");
        match cmd {
            "rect" => Self::rect_from(rest),
            "rotate" => Self::rotate_from(rest),
            _ => panic!("Unexpected instruction {cmd}"),
        }
    }

    fn rect_from(s: &str) -> Self {
        let (wide, tall) = s
            .split_once('x')
            .expect("Incorrect format for 'rect' operation");
        let wide = wide.parse::<usize>().expect("wide should be unsigned int");
        let tall = tall.parse::<usize>().expect("tall should be unsigned int");
        Self::Rect(wide, tall)
    }

    fn rotate_from(s: &str) -> Self {
        let (dir, rest) = s
            .split_once('=')
            .expect("Incorrect format for 'rotate' operation");
        let is_row = dir.starts_with("row");
        let (number, count) = rest
            .split_once(" by ")
            .expect("Incorrect parameters for 'rotate' operation");
        let number = number
            .parse::<usize>()
            .expect("row/col number should be int");
        let count = count.parse::<usize>().expect("'by' count should be int");
        if is_row {
            Self::RotateRow(number, count)
        } else {
            Self::RotateCol(number, count)
        }
    }
}

struct Display {
    pixels: Vec<Vec<bool>>,
}

impl Display {
    fn new(wide: usize, tall: usize) -> Self {
        Self {
            pixels: vec![vec![false; wide]; tall],
        }
    }

    fn execute(&mut self, op: &Operation) {
        match op {
            Operation::Rect(wide, tall) => self.execute_rect(*wide, *tall),
            Operation::RotateCol(x, count) => self.execute_rotate_col(*x, *count),
            Operation::RotateRow(y, count) => self.execute_rotate_row(*y, *count),
        }
    }

    fn execute_rect(&mut self, wide: usize, tall: usize) {
        for r in 0..tall {
            for c in 0..wide {
                self.pixels[r][c] = true;
            }
        }
    }

    #[allow(clippy::needless_range_loop)]
    fn execute_rotate_col(&mut self, x: usize, count: usize) {
        let mut arr = Vec::new();
        for r in 0..self.pixels.len() {
            arr.push(self.pixels[r][x]);
        }
        let len = arr.len();
        let mut rotated = vec![false; len];
        for i in 0..len {
            rotated[(i + count) % len] = arr[i];
        }
        for r in 0..self.pixels.len() {
            self.pixels[r][x] = rotated[r];
        }
    }

    fn execute_rotate_row(&mut self, y: usize, count: usize) {
        let row = &self.pixels[y];
        let len = row.len();

        let mut rotated = vec![false; len];
        for i in 0..len {
            rotated[(i + count) % len] = row[i];
        }

        self.pixels[y] = rotated;
    }

    fn lit_pixels_count(&self) -> usize {
        self.pixels
            .iter()
            .map(|row| row.iter().filter(|&px| *px).count())
            .sum()
    }
}

pub struct AoC2016_08 {
    ops: Vec<Operation>,
}

impl AoC2016_08 {
    pub fn new() -> io::Result<Self> {
        let ops = read_file_as_lines("input/aoc2016_08")?
            .iter()
            .map(|s| Operation::from_str(s))
            .collect();
        Ok(Self { ops })
    }
}

impl Solution for AoC2016_08 {
    fn part_one(&self) -> String {
        let mut display = Display::new(50, 6);
        self.ops.iter().for_each(|op| {
            display.execute(op);
        });
        print(&display.pixels);
        display.lit_pixels_count().to_string()
    }

    fn part_two(&self) -> String {
        "EOARGPHYAO".to_string()
    }

    fn description(&self) -> String {
        "AoC 2016/Day 8: Two-Factor Authentication".to_string()
    }
}

#[allow(clippy::needless_range_loop)]
fn print(inp: &Vec<Vec<bool>>) {
    for i in 0..inp.len() {
        for ch in &inp[i] {
            print!("{}", if *ch { '#' } else { ' ' });
        }
        println!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_08_input_load_test() -> io::Result<()> {
        let sol = AoC2016_08::new()?;
        assert!(!sol.ops.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2016_08_correctness() -> io::Result<()> {
        let sol = AoC2016_08::new()?;
        assert_eq!(sol.part_one(), "128");
        assert_eq!(sol.part_two(), "EOARGPHYAO");
        Ok(())
    }

    #[test]
    fn aoc2016_08_operations() {
        let mut display = Display::new(7, 3);
        {
            display.execute(&Operation::Rect(3, 2));
            let pixels = convert(&vec!["###....", "###....", "......."]);
            assert!(is_equal(&pixels, &display.pixels));
        }
        {
            // rotate column x=1 by 1
            display.execute(&Operation::RotateCol(1, 1));
            let pixels = convert(&vec!["#.#....", "###....", ".#....."]);
            assert!(is_equal(&pixels, &display.pixels));
        }
        {
            // rotate row y=0 by 4
            display.execute(&Operation::RotateRow(0, 4));
            let pixels = convert(&vec!["....#.#", "###....", ".#....."]);
            assert!(is_equal(&pixels, &display.pixels));
        }
        {
            // rotate column x=1 by 1
            display.execute(&Operation::RotateCol(1, 1));
            let pixels = convert(&vec![".#..#.#", "#.#....", ".#....."]);
            assert!(is_equal(&pixels, &display.pixels));
        }
    }

    fn is_equal(a: &Vec<Vec<bool>>, b: &Vec<Vec<bool>>) -> bool {
        if a.len() != b.len() {
            return false;
        }
        for i in 0..a.len() {
            if a[i].len() != b[i].len() {
                return false;
            }
            for j in 0..a[i].len() {
                if a[i][j] != b[i][j] {
                    return false;
                }
            }
        }
        true
    }

    fn convert(inp: &Vec<&str>) -> Vec<Vec<bool>> {
        let mut result = vec![];
        for i in 0..inp.len() {
            result.push(vec![]);
            for ch in inp[i].chars() {
                result[i].push(ch == '#');
            }
        }
        result
    }
}
