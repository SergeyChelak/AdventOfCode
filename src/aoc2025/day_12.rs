use crate::solution::Solution;

use std::io;

struct Region {
    width: usize,
    height: usize,
    amount: Vec<usize>,
}

impl Region {
    fn is_enough_space(&self, squares: &[usize]) -> bool {
        let shapes_square = self
            .amount
            .iter()
            .zip(squares.iter())
            .map(|(amount, square)| *amount * *square)
            .sum::<usize>();
        self.square() >= shapes_square
    }

    fn square(&self) -> usize {
        self.width * self.height
    }
}

impl From<&str> for Region {
    fn from(value: &str) -> Self {
        let (size, amount) = value.split_once(':').expect("Invalid region format");

        let (width, height) = size.split_once('x').expect("Invalid region size format");

        let amount = amount
            .split_whitespace()
            .map(|x| {
                x.parse::<usize>()
                    .expect("Amount must contain integer values only")
            })
            .collect::<Vec<_>>();

        Self {
            width: width
                .parse::<usize>()
                .expect("Region width must be integer"),
            height: height
                .parse::<usize>()
                .expect("Region height must be integer"),
            amount,
        }
    }
}

pub struct AoC2025_12 {
    regions: Vec<Region>,
    squares: Vec<usize>,
}

impl AoC2025_12 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2025_12")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        let blocks = data.split("\n\n").collect::<Vec<_>>();
        let mut iter = blocks.iter();
        let region_data = iter.next_back().expect("Empty input");
        let regions = parse_regions(region_data);
        let squares = iter.map(|s| shape_square(s)).collect::<Vec<_>>();
        Self { regions, squares }
    }
}

impl Solution for AoC2025_12 {
    fn part_one(&self) -> String {
        self.regions
            .iter()
            .filter(|r| r.is_enough_space(&self.squares))
            .count()
            .to_string()
    }

    fn description(&self) -> String {
        "Day 12: Christmas Tree Farm".to_string()
    }
}

fn parse_regions(data: &str) -> Vec<Region> {
    data.trim()
        .split('\n')
        .map(Region::from)
        .collect::<Vec<_>>()
}

fn shape_square(data: &str) -> usize {
    data.split('\n')
        .skip(1)
        .map(|s| s.chars().filter(|ch| *ch == '#').count())
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2025_12_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.regions.is_empty());
        assert!(!sol.squares.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2025_12_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "408");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2025_12> {
        AoC2025_12::new()
    }
}
