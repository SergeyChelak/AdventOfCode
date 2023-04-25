use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2016_20 {
    ranges: Vec<(usize, usize)>,
}

impl AoC2016_20 {
    pub fn new() -> io::Result<Self> {
        let mut ranges = read_file_as_lines("input/aoc2016_20")?
            .iter()
            .map(|s| {
                let (from, to) = s
                    .split_once('-')
                    .expect("Ranges should be separated with '-' delimiter");
                let from = from
                    .parse::<usize>()
                    .expect("Low range value should be integer");
                let to = to
                    .parse::<usize>()
                    .expect("High range value should be integer");
                (from, to)
            })
            .collect::<Vec<(usize, usize)>>();
        Ok(Self {
            ranges: Self::merge_ranges(&mut ranges),
        })
    }

    fn merge_ranges(ranges: &mut [(usize, usize)]) -> Vec<(usize, usize)> {
        ranges.sort_by(|(a, _), (b, _)| a.cmp(b));
        let mut merged = Vec::new();
        let (mut from, mut to) = (ranges[0].0, ranges[0].1);
        for (a, b) in ranges.iter() {
            if *a <= to + 1 {
                to = to.max(*b);
            } else {
                merged.push((from, to));
                from = *a;
                to = *b;
            }
        }
        merged.push((from, to));
        merged
    }
}

impl Solution for AoC2016_20 {
    fn part_one(&self) -> String {
        match self.ranges.len() {
            0 => 0,
            _ => {
                let (l, h) = self.ranges[0];
                if l == 0 {
                    h + 1
                } else {
                    l - 1
                }
            }
        }
        .to_string()
    }

    fn part_two(&self) -> String {
        (4294967296 - self.ranges.iter().map(|(l, h)| *h - *l + 1).sum::<usize>()).to_string()
    }

    fn description(&self) -> String {
        "AoC 2016/Day 20: Firewall Rules".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_20_input_load_test() -> io::Result<()> {
        let sol = AoC2016_20::new()?;
        assert!(!sol.ranges.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2016_20_correctness() -> io::Result<()> {
        let sol = AoC2016_20::new()?;
        assert_eq!(sol.part_one(), "31053880");
        assert_eq!(sol.part_two(), "117");
        Ok(())
    }
}
