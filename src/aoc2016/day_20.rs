use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2016_20 {
    ranges: Vec<(usize, usize)>,
}

impl AoC2016_20 {
    pub fn new() -> io::Result<Self> {
        let ranges = read_file_as_lines("input/aoc2016_20")?
            .iter()
            .map(|s| {
                let (from, to) = s.split_once('-')
                    .expect("Ranges should be separated with '-' delimiter");
                let from = from.parse::<usize>()
                    .expect("Low range value should be integer");
                let to = to.parse::<usize>()
                    .expect("High range value should be integer");
                (from, to)
            })
            .collect::<Vec<(usize, usize)>>();
        Ok(Self {
            ranges
        })
    }
}

impl Solution for AoC2016_20 {
    fn part_one(&self) -> String {
        let mut i = 0;
        'outer: while i <= 4_294_967_295 {
            for (a, b) in &self.ranges {
                if *a <= i && i <= *b {
                    i += 1;
                    continue 'outer;
                }                
            }
            break;
        }
        i.to_string()
    }

    // fn part_two(&self) -> String {
    // }

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
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}