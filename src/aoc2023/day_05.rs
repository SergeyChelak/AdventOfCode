use crate::solution::Solution;
use crate::utils::*;

use std::io;

#[derive(Clone)]
struct MapItem {
    src_start: Int,
    dest_start: Int,
    length: Int,
}

type Int = u64;

pub struct AoC2023_05 {
    seeds: Vec<Int>,
    mapping: Vec<Vec<MapItem>>,
}

impl AoC2023_05 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_05")?;
        let (seeds, mapping) = Self::parse_input(&lines);
        Ok(Self { seeds, mapping })
    }

    fn parse_input(lines: &[String]) -> (Vec<Int>, Vec<Vec<MapItem>>) {
        let parse_nums = |s: &str| -> Vec<Int> {
            s.split_whitespace()
                .map(|x| x.parse::<Int>().expect("Number is expected"))
                .collect::<Vec<_>>()
        };

        let seeds = {
            let (_, nums) = lines[0]
                .split_once(": ")
                .expect("Seeds should be at 1st line");

            parse_nums(nums)
        };

        let mut map: Vec<Vec<MapItem>> = Vec::new();
        let mut mapping: Vec<MapItem> = Vec::new();
        for s in lines.iter().skip(1) {
            if s.is_empty() {
                if !mapping.is_empty() {
                    map.push(mapping.clone());
                }
                continue;
            }
            if s.contains("-to-") {
                mapping = Vec::new();
                continue;
            }
            let nums = parse_nums(s);
            assert_eq!(3, nums.len(), "Invalid map parameters count");
            let item = MapItem {
                dest_start: nums[0],
                src_start: nums[1],
                length: nums[2],
            };
            mapping.push(item);
        }
        map.push(mapping);
        (seeds, map)
    }

    fn convert(&self, map_id: usize, value: Int) -> Int {
        for item in &self.mapping[map_id] {
            if (item.src_start..=item.src_start + item.length).contains(&value) {
                let delta = value - item.src_start;
                return delta + item.dest_start;
            }
        }
        value
    }
}

impl Solution for AoC2023_05 {
    fn part_one(&self) -> String {
        let mut values = self.seeds.clone();
        for i in 0..self.mapping.len() {
            values.iter_mut().for_each(|x| *x = self.convert(i, *x));
        }
        values.iter().min().expect("Incorrect state").to_string()
    }

    fn part_two(&self) -> String {
        let mut result = Int::MAX;
        for chunk in self.seeds.chunks(2) {
            for i in 0..chunk[1] {
                let mut val = chunk[0] + i;
                for i in 0..self.mapping.len() {
                    val = self.convert(i, val);
                }
                result = result.min(val);
            }
        }
        result.to_string()
    }

    fn description(&self) -> String {
        "AoC 2023/Day 5: If You Give A Seed A Fertilizer".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_05_input_load_test() -> io::Result<()> {
        let sol = AoC2023_05::new()?;
        assert!(!sol.seeds.is_empty());
        assert!(!sol.mapping.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_05_ex2() {
        let inp = [
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
        let (seeds, mapping) = AoC2023_05::parse_input(&inp);
        let sol = AoC2023_05 { seeds, mapping };
        assert_eq!("46", sol.part_two());
    }

    #[test]
    fn aoc2023_05_correctness() -> io::Result<()> {
        let sol = AoC2023_05::new()?;
        assert_eq!(sol.part_one(), "57075758");
        assert_eq!(sol.part_two(), "31161857");
        Ok(())
    }
}
