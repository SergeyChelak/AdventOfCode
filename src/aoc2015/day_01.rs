use std::fs::File;
use std::io::{self, Read};

use crate::solution::*;

pub struct AoC2015_01 {
    input: Vec<char>,
}

impl AoC2015_01 {
    pub fn new() -> io::Result<Self> {
        let input = Self::load_input()?;
        Ok(Self { input })
    }

    fn load_input() -> io::Result<Vec<char>> {
        let mut file = File::open("input/aoc2015_01")?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer.iter().map(|val| *val as char).collect())
    }
}

impl Solution for AoC2015_01 {
    fn part_one(&self) -> String {
        self.input
            .iter()
            .fold(0isize, |acc, val| acc + if *val == '(' { 1 } else { -1 })
            .to_string()
    }

    fn part_two(&self) -> String {
        let mut level = 0isize;
        let mut index: Option<usize> = None;
        for i in 0..self.input.len() {
            level += if self.input[i] == '(' { 1 } else { -1 };
            if level == -1 {
                index = Some(1 + i);
                break;
            }
        }
        if let Some(index) = index {
            index.to_string()
        } else {
            "Not found".to_string()
        }
    }

    fn description(&self) -> String {
        "AoC 2015/Day 1".to_string()
    }
}
