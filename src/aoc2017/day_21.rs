use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

type Value = char;
type Matrix = Vec<Vec<Value>>;

fn matrix_from_str(s: &str) -> Matrix {
    s.split('/')
        .map(|x| x.chars().collect::<Vec<Value>>())
        .collect::<Matrix>()
}

fn matrix_to_string(matrix: &Matrix) -> String {
    matrix
        .iter()
        .map(|arr| arr.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("/")
}

fn flip_horizontal(matrix: &Matrix) -> Matrix {
    let mut matrix = matrix.clone();
    matrix.iter_mut().for_each(|row| row.reverse());
    matrix
}

fn flip_vertical(matrix: &Matrix) -> Matrix {
    matrix.clone().into_iter().rev().collect::<Matrix>()
}

fn transpose(matrix: &Matrix) -> Matrix {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut result = vec![vec!['\0'; n]; m];
    for i in 0..n {
        for j in 0..m {
            result[j][i] = matrix[i][j];
        }
    }
    result
}

fn lighting_pixels(matrix: &Matrix) -> usize {
    matrix
        .iter()
        .map(|x| {
            x.iter()
                .map(|v| if *v == '#' { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>()
}

enum Operation {
    Nothing,
    Transpose,
    FlipVertical,
    FlipHorizontal,
}

pub struct AoC2017_21 {
    rules: HashMap<String, String>,
}

impl AoC2017_21 {
    pub fn new() -> io::Result<Self> {
        let mut rules = HashMap::new();
        read_file_as_lines("input/aoc2017_21")?
            .iter()
            .map(|s| s.split_once(" => ").expect("Invalid pattern format"))
            .for_each(|(inp, out)| {
                rules.insert(inp.to_string(), out.to_string());
            });
        Ok(Self { rules })
    }

    fn enhance_step(&self, matrix: &mut Matrix) {
        let len = matrix.len();
        let step = if len % 2 == 0 {
            2
        } else if len % 3 == 0 {
            3
        } else {
            panic!("Unexpected step size")
        };

        let mut output = {
            let dim = len / step * (step + 1);
            vec![vec!['\0'; dim]; dim]
        };
        for r in (0..len).step_by(step) {
            for c in (0..len).step_by(step) {
                let mut inner = vec![vec!['\0'; step]; step];
                for i in 0..step {
                    for j in 0..step {
                        inner[i][j] = matrix[i + r][j + c];
                    }
                }
                let pattern = self
                    .find_pattern(&mut inner)
                    .expect("Matching pattern not found");
                let enhanced = &mut matrix_from_str(&pattern);
                let dim = 1 + step;
                let rx = (r / step) * dim;
                let cx = (c / step) * dim;
                for i in 0..dim {
                    for j in 0..dim {
                        output[rx + i][cx + j] = enhanced[i][j];
                    }
                }
            }
        }
        *matrix = output;
    }

    fn find_pattern(&self, matrix: &mut Matrix) -> Option<String> {
        let ops = [
            Operation::Nothing,
            Operation::FlipHorizontal,
            Operation::FlipVertical,
            Operation::FlipHorizontal,
            Operation::Transpose,
            Operation::FlipHorizontal,
            Operation::FlipVertical,
            Operation::FlipHorizontal,
        ];
        for op in &ops {
            match op {
                Operation::Nothing => {}
                Operation::Transpose => *matrix = transpose(matrix),
                Operation::FlipHorizontal => *matrix = flip_horizontal(matrix),
                Operation::FlipVertical => *matrix = flip_vertical(matrix),
            }
            let key = matrix_to_string(&matrix);
            if let Some(pattern) = self.rules.get(&key) {
                return Some(pattern.to_string());
            }
        }
        None
    }

    fn count_pixels(&self, steps: usize) -> usize {
        let mut matrix = matrix_from_str(".#./..#/###");
        (0..steps).for_each(|_| self.enhance_step(&mut matrix));
        lighting_pixels(&matrix)
    }
}

impl Solution for AoC2017_21 {
    fn part_one(&self) -> String {
        self.count_pixels(5).to_string()
    }

    fn part_two(&self) -> String {
        self.count_pixels(18).to_string()
    }

    fn description(&self) -> String {
        "AoC 2017/Day 21: Fractal Art".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_21_input_load_test() -> io::Result<()> {
        let sol = AoC2017_21::new()?;
        assert!(!sol.rules.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_21_correctness() -> io::Result<()> {
        let sol = AoC2017_21::new()?;
        assert_eq!(sol.part_one(), "208");
        assert_eq!(sol.part_two(), "2480380");
        Ok(())
    }
}
