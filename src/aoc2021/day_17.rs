use crate::{solution::Solution, utils::PlainInterval};

use std::io;

type Int = isize;
type Interval = PlainInterval<Int>;

impl From<&str> for Interval {
    fn from(value: &str) -> Self {
        let (_, rng) = value.split_once('=').expect("Invalid interval format");
        let (l, r) = rng.split_once("..").expect("Invalid range format");

        Self::new(
            l.parse::<Int>().expect("Integer only in range start"),
            r.parse::<Int>().expect("Integer only in range end"),
        )
    }
}

pub struct AoC2021_17 {
    x_range: Interval,
    y_range: Interval,
}

impl AoC2021_17 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2021_17")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        // target area: x=156..202, y=-110..-69
        let data = data.strip_prefix("target area: ").expect("Invalid input");
        let (x_rng, y_rng) = data.trim().split_once(", ").expect("Invalid delimiter");
        Self {
            x_range: Interval::from(x_rng),
            y_range: Interval::from(y_rng),
        }
    }
}

impl Solution for AoC2021_17 {
    fn part_one(&self) -> String {
        let y = self.y_range.begin;
        (y * (y + 1) / 2).to_string()
    }

    fn part_two(&self) -> String {
        let mut count = 0;
        for vy in self.y_range.begin..=1 - self.y_range.begin {
            for vx in 0..=self.x_range.end {
                let (mut x, mut y) = (0, 0);

                for t in 0.. {
                    y += vy - t;
                    if y < self.y_range.begin {
                        break;
                    }
                    if vx > t {
                        x += vx - t;
                    }
                    if self.x_range.close_contain(x) && self.y_range.close_contain(y) {
                        count += 1;
                        break;
                    }
                }
            }
        }

        count.to_string()
    }

    fn description(&self) -> String {
        "Day 17: Trick Shot".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_17_input_load_test() -> io::Result<()> {
        let _ = make_solution()?;
        Ok(())
    }

    #[test]
    fn aoc2021_17_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "5995");
        Ok(())
    }

    #[test]
    fn aoc2021_17_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "3202");
        Ok(())
    }

    #[test]
    fn aoc2021_17_case2() {
        let sol = AoC2021_17::parse_data("target area: x=20..30, y=-10..-5");
        assert_eq!(sol.part_two(), "112");
    }

    fn make_solution() -> io::Result<AoC2021_17> {
        AoC2021_17::new()
    }
}
