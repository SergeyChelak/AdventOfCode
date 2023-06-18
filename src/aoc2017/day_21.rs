use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

type Value = char;
type Matrix = Vec<Vec<Value>>;

trait MatrixOps {
    fn initial() -> Self;

    fn as_string(&self) -> String;

    fn from_str(s: &str) -> Self;

    fn rotate(&self) -> Self;
}

impl MatrixOps for Matrix {
    fn initial() -> Self {
        Self::from_str(".#./..#/###")
    }

    fn as_string(&self) -> String {
        self.iter()
            .map(|arr| arr.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("/")
    }

    fn from_str(s: &str) -> Self {
        s.split('/')
            .map(|x| x.chars().collect::<Vec<Value>>())
            .collect::<Matrix>()
    }

    fn rotate(&self) -> Self {
        let mut matrix = self.clone().into_iter().rev().collect::<Matrix>();
        for i in 0..matrix.len() {
            for j in i + 1..matrix[i].len() {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
        matrix
    }
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
}

impl Solution for AoC2017_21 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

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
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
