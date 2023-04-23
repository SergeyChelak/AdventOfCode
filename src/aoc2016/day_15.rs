use crate::solution::Solution;
use crate::utils::*;

use std::io;

struct Disk {
    number: usize,
    phase: usize,       // initial position
    positions: usize
}

impl Disk {
    fn from_str(s: &str) -> Self {
        let tokens = s.split(' ').collect::<Vec<&str>>();
        let number = tokens[1][1..].parse::<usize>()
            .expect("Disk number non integer or in wrong position");
        let positions = tokens[3].parse::<usize>()
            .expect("Disk positions amount is non integer or in wrong position");
        let phase = tokens[tokens.len() - 1];
        let phase = phase[0..phase.len()-1].parse::<usize>()
            .expect("Initial disk position is non integer or in wrong position");
        Self {
            number,
            phase,
            positions
        }
    }

    fn position(&self, time: usize) -> usize {
        (self.phase + self.number + time) % self.positions
    }
}

pub struct AoC2016_15 {
    input: Vec<Disk>,
}

impl AoC2016_15 {
    pub fn new() -> io::Result<Self> {
        let input = read_file_as_lines("input/aoc2016_15")?
            .iter()
            .map(|s| Disk::from_str(s))
            .collect();
        Ok(Self {
            input
        })
    }
}

impl Solution for AoC2016_15 {
    fn part_one(&self) -> String {
        let mut time = 0usize;
        loop {
            let can_drop = self.input
                .iter()
                .map(|disk| disk.position(time))
                .sum::<usize>() == 0;
            if can_drop {
                break;
            }
            time += 1;
        }
        time.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2016/Day 15: Timing is Everything".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_15_input_load_test() -> io::Result<()> {
        let sol = AoC2016_15::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2016_15_correctness() -> io::Result<()> {
        let sol = AoC2016_15::new()?;
        assert_eq!(sol.part_one(), "16824");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}