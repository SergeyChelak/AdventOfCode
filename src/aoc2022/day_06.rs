use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

pub struct AoC2022_06 {
    input: Vec<char>,
}

impl AoC2022_06 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2022_06")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        let input = data.chars().collect();
        Self { input }
    }

    fn post_marker_index(&self, window_size: usize) -> String {
        // not too much efficient but quick in the implementation
        for (index, window) in self.input.windows(window_size).enumerate() {
            let set = window.iter().collect::<HashSet<_>>();
            if set.len() == window_size {
                return (window_size + index).to_string();
            }
        }
        not_found()
    }
}

impl Solution for AoC2022_06 {
    fn part_one(&self) -> String {
        self.post_marker_index(4)
    }

    fn part_two(&self) -> String {
        self.post_marker_index(14)
    }

    fn description(&self) -> String {
        "Day 6: Tuning Trouble".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_06_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2022_06_case_1() {
        let cases = [
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];
        for (inp, out) in cases {
            let sol = AoC2022_06::parse_data(inp);
            assert_eq!(sol.part_one(), out.to_string());
        }
    }

    #[test]
    fn aoc2022_06_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "1896");
        Ok(())
    }

    #[test]
    fn aoc2022_06_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "3452");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2022_06> {
        AoC2022_06::new()
    }
}
