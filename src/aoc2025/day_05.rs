use crate::{solution::Solution, utils::PlainInterval};

use std::io;

type Int = usize;
type RangeInt = PlainInterval<Int>;

impl RangeInt {
    fn contains(&self, value: &Int) -> bool {
        self.begin <= *value && *value <= self.end
    }
}

pub struct AoC2025_05 {
    ranges: Vec<RangeInt>,
    ingredients: Vec<Int>,
}

impl AoC2025_05 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2025_05")?;
        Ok(Self::parse(&data))
    }

    fn parse(data: &str) -> Self {
        let (ranges, ingredients) = data.split_once("\n\n").expect("Invalid input format");
        let ranges = ranges
            .split('\n')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(Self::parse_range)
            .collect::<Vec<_>>();

        let ingredients = ingredients
            .split('\n')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<Int>().expect("Ingredient must be integer"))
            .collect::<Vec<_>>();
        Self {
            ranges,
            ingredients,
        }
    }

    fn parse_range(s: &str) -> RangeInt {
        let (start, end) = s.split_once('-').expect("Invalid range format");
        RangeInt::new(
            start.parse::<Int>().expect("Range start must be integer"),
            end.parse::<Int>().expect("Range end must be integer"),
        )
    }
}

impl Solution for AoC2025_05 {
    fn part_one(&self) -> String {
        self.ingredients
            .iter()
            .filter(|elem| self.ranges.iter().any(|rng| rng.contains(elem)))
            .count()
            .to_string()
    }

    fn part_two(&self) -> String {
        let mut ranges = self.ranges.clone();
        loop {
            let mut intersection: Option<Intersection> = None;
            'search: for (f, first) in ranges.iter().enumerate() {
                for (s, second) in ranges.iter().enumerate().skip(f + 1) {
                    let Some(merged) = first.union(second) else {
                        continue;
                    };
                    intersection = Some(Intersection {
                        first: f,
                        second: s,
                        merged,
                    });
                    break 'search;
                }
            }
            let Some(intersection) = intersection else {
                break;
            };

            swap_with_last_and_remove(&mut ranges, intersection.second);
            swap_with_last_and_remove(&mut ranges, intersection.first);
            ranges.push(intersection.merged);
        }

        ranges
            .iter()
            .map(|r| r.end - r.begin + 1)
            .sum::<Int>()
            .to_string()
    }

    fn description(&self) -> String {
        "Day 5: Cafeteria".to_string()
    }
}

struct Intersection {
    first: usize,
    second: usize,
    merged: RangeInt,
}

fn swap_with_last_and_remove<T>(arr: &mut Vec<T>, index: usize) {
    if arr.is_empty() {
        return;
    }
    let last_idx = arr.len() - 1;
    arr.swap(index, last_idx);
    arr.pop();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2025_05_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.ingredients.is_empty());
        assert!(!sol.ranges.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2025_05_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "707");
        Ok(())
    }

    #[test]
    fn aoc2025_05_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "361615643045059");
        Ok(())
    }

    #[test]
    fn aoc2025_05_case_1() -> io::Result<()> {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "3");
        Ok(())
    }

    #[test]
    fn aoc2025_05_case_2() -> io::Result<()> {
        let sol = make_test_solution();
        assert_eq!(sol.part_two(), "14");
        Ok(())
    }

    fn make_test_solution() -> AoC2025_05 {
        let input = "3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
";
        AoC2025_05::parse(input)
    }

    fn make_solution() -> io::Result<AoC2025_05> {
        AoC2025_05::new()
    }
}
