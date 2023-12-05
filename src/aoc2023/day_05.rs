use crate::solution::Solution;
use crate::utils::*;

use std::io;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Component {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl TryFrom<&str> for Component {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "seed" => Ok(Component::Seed),
            "soil" => Ok(Component::Soil),
            "fertilizer" => Ok(Component::Fertilizer),
            "water" => Ok(Component::Water),
            "light" => Ok(Component::Light),
            "temperature" => Ok(Component::Temperature),
            "humidity" => Ok(Component::Humidity),
            "location" => Ok(Component::Location),
            _ => Err(()),
        }
    }
}

struct MapItem {
    src_type: Component,
    src_start: Int,
    dest_type: Component,
    dest_start: Int,
    length: Int,
}

type Int = u64;

pub struct AoC2023_05 {
    seeds: Vec<Int>,
    mapping: Vec<MapItem>,
}

impl AoC2023_05 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_05")?;
        let (seeds, mapping) = Self::parse_input(&lines);
        Ok(Self { seeds, mapping })
    }

    fn parse_input(lines: &[String]) -> (Vec<Int>, Vec<MapItem>) {
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

        let mut mapping: Vec<MapItem> = Vec::new();
        let mut src = Component::Seed;
        let mut dest = Component::Seed;
        for s in lines.iter().skip(1) {
            if s.is_empty() {
                continue;
            }
            if s.contains("-to-") {
                let (map_type, _) = s.split_once(' ').expect("Map header is invalid");
                let (src_str, dest_str) = map_type.split_once("-to-").expect("Invalid map type");
                src = Component::try_from(src_str).unwrap();
                dest = Component::try_from(dest_str).unwrap();
                continue;
            }
            let nums = parse_nums(s);
            assert_eq!(3, nums.len(), "Invalid map parameters count");
            let item = MapItem {
                src_type: src,
                dest_type: dest,
                dest_start: nums[0],
                src_start: nums[1],
                length: nums[2],
            };
            mapping.push(item);
        }
        (seeds, mapping)
    }

    fn convert(&self, src_type: Component, dest_type: Component, value: Int) -> Int {
        for item in &self.mapping {
            if item.src_type != src_type || item.dest_type != dest_type {
                continue;
            }
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
        let flow = [
            Component::Seed,
            Component::Soil,
            Component::Fertilizer,
            Component::Water,
            Component::Light,
            Component::Temperature,
            Component::Humidity,
            Component::Location,
        ];
        let mut values = self.seeds.clone();
        for i in 0..flow.len() - 1 {
            let src = flow[i];
            let dest = flow[i + 1];
            values
                .iter_mut()
                .for_each(|x| *x = self.convert(src, dest, *x));
        }
        values.iter().min().expect("Incorrect state").to_string()
    }

    // fn part_two(&self) -> String {
    // }

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
    fn aoc2023_05_correctness() -> io::Result<()> {
        let sol = AoC2023_05::new()?;
        assert_eq!(sol.part_one(), "57075758");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
