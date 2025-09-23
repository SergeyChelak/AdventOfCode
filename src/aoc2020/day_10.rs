use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

type Int = usize;

pub struct AoC2020_10 {
    input: Vec<Int>,
}

impl AoC2020_10 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2020_10")?;
        Ok(Self::parse(&lines))
    }

    fn parse<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = parse(lines).expect("Invalid input value");
        Self { input }
    }
}

impl Solution for AoC2020_10 {
    fn part_one(&self) -> String {
        let mut arr = self.input.clone();
        assert!(!arr.contains(&0));
        arr.push(0);
        arr.sort();
        let (mut d1, mut d3) = (0, 1);
        for w in arr.windows(2) {
            match w[1] - w[0] {
                1 => d1 += 1,
                3 => d3 += 1,
                _ => continue,
            }
        }
        (d1 * d3).to_string()
    }

    fn part_two(&self) -> String {
        let mut arr = self.input.clone();
        assert!(!arr.contains(&0));
        arr.sort();
        let device = arr.last().unwrap() + 3;
        arr.push(device);

        let mut map = HashMap::new();
        map.insert(0usize, 1usize);

        for val in arr.iter() {
            let mut count = 0;
            for step in 1..=3 {
                if *val < step {
                    continue;
                }
                count += map.get(&(*val - step)).cloned().unwrap_or_default();
            }
            map.insert(*val, count);
        }

        map.get(&device)
            .map(|x| x.to_string())
            .unwrap_or(not_found())
    }

    fn description(&self) -> String {
        "Day 10: Adapter Array".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_10_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2020_10_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "2574");
        Ok(())
    }

    #[test]
    fn aoc2020_10_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "2644613988352");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_10> {
        AoC2020_10::new()
    }

    #[test]
    fn aoc2020_10_case2() {
        let inp = ["16", "10", "15", "5", "1", "11", "7", "19", "6", "12", "4"];
        let sol = AoC2020_10::parse(&inp);
        assert_eq!(sol.part_two(), "8");
    }

    #[test]
    fn aoc2020_10_case2_1() {
        let inp = [
            "28", "33", "18", "42", "31", "14", "46", "20", "48", "47", "24", "23", "49", "45",
            "19", "38", "39", "11", "1", "32", "25", "35", "8", "17", "7", "9", "4", "2", "34",
            "10", "3",
        ];
        let sol = AoC2020_10::parse(&inp);
        assert_eq!(sol.part_two(), "19208");
    }
}
