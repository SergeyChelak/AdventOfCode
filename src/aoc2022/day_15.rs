use regex::Regex;

use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = isize;
type Point = Point2d<Int>;

struct Element {
    sensor: Point,
    beacon: Point,
}

impl Element {
    fn is_beacon_not_expected(&self, point: &Point) -> bool {
        if *point == self.beacon {
            return false;
        }
        self.sensor.manhattan(point) <= self.distance()
    }

    fn distance(&self) -> usize {
        self.sensor.manhattan(&self.beacon)
    }
}

impl Point {
    fn manhattan(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

pub struct AoC2022_15 {
    input: Vec<Element>,
}

impl AoC2022_15 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2022_15")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        Self::parse_lines(&data.lines().collect::<Vec<_>>())
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let Ok(regex) = Regex::new(r"-?\d+") else {
            panic!("Invalid regexp")
        };
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .filter(|x| !x.is_empty())
            .map(|s| {
                regex
                    .find_iter(s)
                    .map(|x| {
                        x.as_str()
                            .parse::<Int>()
                            .expect("Failed to parse coordinate value")
                    })
                    .collect::<Vec<_>>()
            })
            .inspect(|arr| assert_eq!(4, arr.len()))
            .map(|arr| Element {
                sensor: Point::new(arr[0], arr[1]),
                beacon: Point2d::new(arr[2], arr[3]),
            })
            .collect::<Vec<_>>();
        Self { input }
    }

    fn simulate(&self, y: Int) -> usize {
        let max_distance = self
            .input
            .iter()
            .map(|x| x.distance())
            .max()
            .expect("Input is empty?") as Int;

        let flat_list = self
            .input
            .iter()
            .flat_map(|x| [x.sensor, x.beacon])
            .collect::<Vec<_>>();
        let bounds = bounds(&flat_list).expect("failed to calculate bounds");
        // println!("{:?} - {:?}", bounds.low, bounds.h);
        let mut total = 0;
        let from = bounds.low.x - max_distance;
        let to = bounds.high.x + max_distance;
        for x in from..to {
            let p = Point::new(x, y);
            if self.input.iter().any(|x| x.is_beacon_not_expected(&p)) {
                total += 1;
            }
        }
        total
    }
}

impl Solution for AoC2022_15 {
    fn part_one(&self) -> String {
        self.simulate(2000000).to_string()
    }

    fn part_two(&self) -> String {
        let in_range = |p: &Point| -> bool {
            let limit = 4_000_000;
            (0..=limit).contains(&p.x) && (0..=limit).contains(&p.y)
        };
        for element in self.input.iter() {
            let sensor = element.sensor;
            let r = element.distance() as Int;
            for i in 0..=r + 1 {
                let moves = {
                    let dx = i;
                    let dy = (r + 1) - i;
                    [(dx, dy), (dx, -dy), (-dx, dy), (-dx, -dy)]
                };
                for (dx, dy) in moves {
                    let p = sensor.add(&Point::new(dx, dy));
                    if !in_range(&p) {
                        continue;
                    }
                    if !self.input.iter().any(|x| x.is_beacon_not_expected(&p)) {
                        return (4_000_000 * p.x + p.y).to_string();
                    }
                }
            }
        }
        not_found()
    }

    fn description(&self) -> String {
        "Day 15: Beacon Exclusion Zone".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_15_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2022_15_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "13267474686239");
        Ok(())
    }

    #[test]
    fn aoc2022_15_parse() {
        let lines = ["Sensor at x=2, y=18: closest beacon is at x=-2, y=15"];
        let sol = AoC2022_15::parse_lines(&lines);
        let item = sol.input.first().unwrap();
        assert_eq!(item.sensor, Point::new(2, 18));
        assert_eq!(item.beacon, Point::new(-2, 15));
    }

    #[test]
    fn aoc2022_15_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "13267474686239");
        Ok(())
    }

    #[test]
    fn aoc2022_15_case_1() {
        let sol = make_test_solution();
        assert_eq!(sol.simulate(10), 26)
    }

    fn make_solution() -> io::Result<AoC2022_15> {
        AoC2022_15::new()
    }

    fn make_test_solution() -> AoC2022_15 {
        let lines = [
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
            "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
            "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
            "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
            "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
            "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
            "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
            "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
            "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
            "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
            "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
            "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
            "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
            "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        ];
        AoC2022_15::parse_lines(&lines)
    }
}
