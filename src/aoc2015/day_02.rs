use std::io;

use crate::file_utils::read_file_as_lines;
use crate::solution::*;

struct Pack(i32, i32, i32); // l - w - h

impl Pack {
    fn from_string(line: &str) -> Self {
        let list: Vec<i32> = line.split('x')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
        Self(list[0], list[1], list[2])
    }

    fn wrap_size(&self) -> i32 {
        let sq1 = self.0 * self.1;
        let sq2 = self.1 * self.2;
        let sq3 = self.0 * self.2;
        let extra = sq1.min(sq2).min(sq3);
        2 * (sq1 + sq2 + sq3) + extra
    }

    fn ribbon_len(&self) -> i32 {
        let p1 = self.0 + self.1;
        let p2 = self.1 + self.2;
        let p3 = self.0 + self.2;
        2 * (p1.min(p2).min(p3))  + self.0 * self.1 * self.2
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
        let input = read_file_as_lines("input/aoc2015_02")?
                            .iter()
                            .map(|line| Pack::from_string(line))
                            .collect();
        Ok(input)
    }
}

impl Solution for AoC2015_02 {
    fn description(&self) -> String {
        "AoC 2015/Day 2".to_string()
    }

    fn part_one(&self) -> String {
        self.items.iter()
        .map(|pack| pack.wrap_size())
        .sum::<i32>()
        .to_string()
    }

    fn part_two(&self) -> String {
        self.items.iter()
        .map(|pack| pack.ribbon_len())
        .sum::<i32>()
        .to_string()        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc2015_02_wrap_test() {
        let pack = Pack(2, 3, 4);
        assert_eq!(pack.wrap_size(), 58);

        let pack = Pack(1, 1, 10);
        assert_eq!(pack.wrap_size(), 43);
    }

    #[test]
    fn aoc2015_02_ribbon_test() {
        let pack = Pack(2, 3, 4);
        assert_eq!(pack.ribbon_len(), 34);

        let pack = Pack(1, 1, 10);
        assert_eq!(pack.ribbon_len(), 14);
    }

    #[test]
    fn aoc2015_02_correctness() -> io::Result<()> {
        let sol = AoC2015_02::new()?;
        assert_eq!(sol.part_one(), "1598415".to_string());
        assert_eq!(sol.part_two(), "3812909".to_string());
        Ok(())
    }
}