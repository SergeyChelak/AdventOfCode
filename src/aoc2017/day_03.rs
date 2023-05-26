use crate::solution::Solution;

use std::io;

pub struct AoC2017_03 {
    input: u32,
}

impl AoC2017_03 {
    pub fn new() -> io::Result<Self> {
        Ok(Self { input: 277678 })
    }
}

impl Solution for AoC2017_03 {
    fn part_one(&self) -> String {
        distance(self.input).to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 3: Spiral Memory".to_string()
    }
}

fn distance(num: u32) -> u32 {
    if num == 1 {
        return 0;
    }
    let mut bound = 3;
    while bound * bound < num {
        bound += 2;
    }
    let (mut x, mut y) = (bound - 1, bound - 2);
    let from = bound - 2;
    let mut pos = 1 + from * from;
    let mut dir = 0;
    while pos != num {
        match dir {
            0 => y -= 1,
            1 => x -= 1,
            2 => y += 1,
            3 => x += 1,
            _ => panic!("Invalid direction"),
        }
        if x == bound - 1 && y == 0 {
            dir = 1;
        }
        if x == 0 && y == 0 {
            dir = 2;
        }
        if x == 0 && y == bound - 1 {
            dir = 3;
        }
        pos += 1;
    }
    let r = bound / 2;
    let c = r;
    let abs = |a: u32, b: u32| a.max(b) - a.min(b);
    abs(r, x) + abs(c, y)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_03_correctness() -> io::Result<()> {
        let sol = AoC2017_03::new()?;
        assert_eq!(sol.part_one(), "475");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2017_03_distance() {
        assert_eq!(distance(1), 0);
        assert_eq!(distance(12), 3);
        assert_eq!(distance(23), 2);
        assert_eq!(distance(1024), 31);
    }
}
