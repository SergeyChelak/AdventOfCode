use std::fs::File;
use std::io::{self, BufRead};

use crate::solution::*;

pub struct Pack(i32, i32, i32); // l - w - h

impl Pack {
    fn from_string(line: &String) -> Self {
        let list: Vec<i32> = line.split("x")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
        Self(list[0], list[1], list[2])
    }

    fn value(&self) -> i32 {
        let sq1 = self.0 * self.1;
        let sq2 = self.1 * self.2;
        let sq3 = self.0 * self.2;
        let extra = i32::min(i32::min(sq1, sq2), sq3);
        2 * (sq1 + sq2 + sq3) + extra
    }
}

pub struct AoC2015_02 {
    items: Vec<Pack>,
}

impl AoC2015_02 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            items: Self::load_input()?
        })
    }

    fn load_input() -> io::Result<Vec<Pack>> {
        let mut list = Vec::<Pack>::new();
        let file = File::open("input/aoc2015_02")?;
        let lines = io::BufReader::new(file).lines();
        for line in lines {
            let box_dims = Pack::from_string(&line?);
            list.push(box_dims);
        }
        Ok(list)
    }
}

impl Solution for AoC2015_02 {
    fn description(&self) -> String {
        "AoC 2015/Day 2".to_string()
    }

    fn part_one(&self) -> String {
        self.items.iter()
        .map(|pack| pack.value())
        .fold(0, |acc, v| acc + v)
        .to_string()
    }

    // fn part_two(&self) -> String {
        
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc2015_02_box_test() {
        let pack = Pack(2, 3, 4);
        assert_eq!(pack.value(), 58);

        let pack = Pack(1, 1, 10);
        assert_eq!(pack.value(), 43);
    }
}