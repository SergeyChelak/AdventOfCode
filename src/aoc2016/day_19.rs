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
        play_white_elephant_adjacent(self.elves).to_string()
    }

    fn part_two(&self) -> String {
        play_white_elephant_cross(self.elves).to_string()
    }

    fn description(&self) -> String {
        "AoC 2016/Day 19: An Elephant Named Joseph".to_string()
    }
}

fn play_white_elephant_adjacent(participants: usize) -> usize {
    let mut table = vec![1; participants];
    let mut ptr = 0usize;
    loop {
        let mut next = (ptr + 1) % participants;
        while table[next] == 0 {
            next = (next + 1) % participants;
        }
        if next == ptr {
            break next + 1;
        }
        table[ptr] += table[next];
        table[next] = 0;
        ptr = next;
        while table[ptr] == 0 {
            ptr = (ptr + 1) % participants;
        }
    }
}

fn play_white_elephant_cross(participants: usize) -> usize {
    let mut table = vec![1; participants];
    let mut ptr = 0usize;
    loop {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_19_correctness() -> io::Result<()> {
        let sol = AoC2016_19::new()?;
        assert_eq!(sol.part_one(), "1830117");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2016_19_example1() {
        assert_eq!(play_white_elephant_adjacent(5), 3);
    }

    #[test]
    fn aoc2016_19_example2() {
        assert_eq!(play_white_elephant_cross(5), 2);
    }
}
