use crate::solution::Solution;

use std::io;

pub struct AoC2016_19 {
    elves: usize,
}

impl AoC2016_19 {
    pub fn new() -> io::Result<Self> {
        Ok(Self { elves: 3012210 })
    }
}

impl Solution for AoC2016_19 {
    fn part_one(&self) -> String {
        // https://en.wikipedia.org/wiki/Josephus_problem
        let prev = self.elves - (self.elves.next_power_of_two() >> 1);
        (2 * prev + 1).to_string()
    }

    fn part_two(&self) -> String {
        // this solved by https://www.youtube.com/watch?v=5Zqs6e5DwWQ
        let mut i = 1;
        while 3 * i < self.elves {
            i *= 3;
        }
        i = if self.elves < 2 * i {
            self.elves - i
        } else {
            2 * self.elves - 3 * i
        };
        i.to_string()
    }

    fn description(&self) -> String {
        "AoC 2016/Day 19: An Elephant Named Joseph".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_19_correctness() -> io::Result<()> {
        let sol = AoC2016_19::new()?;
        assert_eq!(sol.part_one(), "1830117");
        assert_eq!(sol.part_two(), "1417887");
        Ok(())
    }

    #[test]
    fn aoc2016_19_example1() {
        let res = AoC2016_19 { elves: 5 }.part_one();
        assert_eq!(res, "3");
    }

    #[test]
    fn aoc2016_19_example2() {
        let res = AoC2016_19 { elves: 5 }.part_one();
        assert_eq!(res, "2");
    }
}
