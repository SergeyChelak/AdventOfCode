use crate::solution::Solution;
use crate::utils::*;

use std::io;
use std::str::FromStr;
use std::num::ParseIntError;

struct Reindeer {
    //name: String,
    speed: i32,
    fly_time: i32,
    rest_time: i32
}

impl Reindeer {
    fn distance(&self, time: i32) -> i32 {
        let interval = self.fly_time + self.rest_time;
        let count = time / interval;
        let mut dist = self.speed * count * self.fly_time;
        dist += (time % interval).min(self.fly_time) * self.speed;
        dist
    }
}

impl FromStr for Reindeer {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s.split(" ").collect::<Vec<&str>>();
        // let name = items[0].to_string();
        let speed = items[3].parse::<i32>()?;
        let fly_time = items[6].parse::<i32>()?;
        let rest_time = items[items.len() - 2].parse::<i32>()?;
        Ok(
            Self {
                //name,
                speed,
                fly_time,
                rest_time,
            }
        )
    }
}

pub struct AoC2015_14 {
    reindeers: Vec<Reindeer>,
}

impl AoC2015_14 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2015_14")?;
        let mut reindeers: Vec<Reindeer> = Vec::with_capacity(lines.len());
        for line in lines {
            let item = line.parse::<Reindeer>().ok()
                .expect("Failed to parse reindeer data");
            reindeers.push(item)
        }
        Ok(Self {
            reindeers
        })
    }
}

impl Solution for AoC2015_14 {
    fn part_one(&self) -> String {
        let time = 2503;
        self.reindeers.iter()
            .map(|r| r.distance(time))
            .max()
            .expect("Not found")
            .to_string()
    }

    fn part_two(&self) -> String {
        todo!()
    }

    fn description(&self) -> String {
        "AoC 2015/Day 14: Reindeer Olympics".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_14_input_load_test() -> io::Result<()> {
        let sol = AoC2015_14::new()?;
        assert!(sol.reindeers.len() > 0);
        Ok(())
    }

    #[test]
    fn aoc2015_14_correctness() -> io::Result<()> {
        let sol = AoC2015_14::new()?;
        assert_eq!(sol.part_one(), "2655");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2015_14_distance_case1() {
        let reindeer = Reindeer {
            speed: 8,
            fly_time: 8,
            rest_time: 53
        };
        assert_eq!(reindeer.distance(1), 8);
        assert_eq!(reindeer.distance(2), 16);
        assert_eq!(reindeer.distance(8), 64);
        assert_eq!(reindeer.distance(10), 64);
        assert_eq!(reindeer.distance(53), 64);
        assert_eq!(reindeer.distance(62), 72);
    }

    #[test]
    fn aoc2015_14_distance_case2() {
        let reindeer = Reindeer {
            speed: 14,
            fly_time: 10,
            rest_time: 127
        };
        assert_eq!(reindeer.distance(1), 14);
        assert_eq!(reindeer.distance(11), 140);
        assert_eq!(reindeer.distance(12), 140);
        assert_eq!(reindeer.distance(1000), 1120);
    }

    #[test]
    fn aoc2015_14_distance_case3() {
        let reindeer = Reindeer {
            speed: 16,
            fly_time: 11,
            rest_time: 162
        };
        assert_eq!(reindeer.distance(1), 16);
        assert_eq!(reindeer.distance(10), 160);
        assert_eq!(reindeer.distance(12), 176);
        assert_eq!(reindeer.distance(1000), 1056);
    }
}