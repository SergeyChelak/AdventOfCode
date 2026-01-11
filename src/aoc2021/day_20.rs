use crate::{
    solution::Solution,
    utils::{not_found, Point2d},
};

use std::{collections::HashSet, io};

type Int = isize;
type Point = Point2d<Int>;
type Image = HashSet<Point>;

const LIGHT_PIXEL: char = '#';

pub struct AoC2021_20 {
    enhancement: Vec<u8>,
    image: Image,
}

impl AoC2021_20 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2021_20")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        let (enhancement, pixels) = data.split_once("\n\n").expect("Invalid input format");
        let enhancement = enhancement
            .trim()
            .chars()
            .map(|ch| (ch == LIGHT_PIXEL) as u8)
            .collect::<Vec<_>>();

        let mut image = Image::new();
        for (row, line) in pixels.split('\n').enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch != LIGHT_PIXEL {
                    continue;
                }
                let point = Point::new(col as isize, row as isize);
                image.insert(point);
            }
        }
        Self { enhancement, image }
    }

    fn enhance(&self, times: usize) -> Option<usize> {
        if times.is_multiple_of(2) {
            return None;
        }
        let infinity_values = if self.enhancement[0] == 1 {
            [
                self.enhancement[self.enhancement.len() - 1] as usize,
                self.enhancement[0] as usize,
            ]
        } else {
            [0, 0]
        };

        let mut inf_idx = 0usize;

        let mut image: Option<Image> = Some(self.image.clone());
        for _ in 0..times {
            let prev = image?;
            let inf = infinity_values[inf_idx];
            image = enhance_image(&prev, &self.enhancement, inf);
            inf_idx = (inf_idx + 1) % 2;
        }

        Some(image?.len())
    }
}

impl Solution for AoC2021_20 {
    fn part_one(&self) -> String {
        self.enhance(2)
            .map(|x| x.to_string())
            .unwrap_or(not_found())
    }

    fn part_two(&self) -> String {
        self.enhance(50)
            .map(|x| x.to_string())
            .unwrap_or(not_found())
    }

    fn description(&self) -> String {
        "Day 20: Trench Map".to_string()
    }
}

fn enhance_image(image: &Image, enhancement: &[u8], infinity: usize) -> Option<Image> {
    let (min_x, max_x) = bounds(image, &|p| p.x)?;
    let (min_y, max_y) = bounds(image, &|p| p.y)?;

    let mut result = Image::new();
    const OFFSET: Int = 1;
    for y in min_y - OFFSET..=max_y + OFFSET {
        for x in min_x - OFFSET..=max_x + OFFSET {
            let mut total = 0;
            for r in y - 1..=y + 1 {
                for c in x - 1..=x + 1 {
                    let val = if (min_x..=max_x).contains(&c) && (min_y..=max_y).contains(&r) {
                        let p = Point::new(c, r);
                        if image.contains(&p) {
                            1
                        } else {
                            0
                        }
                    } else {
                        infinity
                    };
                    total = (total << 1) | val;
                }
            }

            if enhancement[total] == 1 {
                let point = Point::new(x, y);
                result.insert(point);
            }
        }
    }

    Some(result)
}

fn bounds(image: &Image, criteria: &impl Fn(&Point) -> Int) -> Option<(Int, Int)> {
    let (Some(lower), Some(upper)) = (
        image.iter().map(criteria).min(),
        image.iter().map(criteria).max(),
    ) else {
        return None;
    };
    Some((lower, upper))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_20_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.image.is_empty());
        assert!(!sol.enhancement.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2021_20_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "5057");
        Ok(())
    }

    #[test]
    fn aoc2021_20_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2021_20_case1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "35");
    }

    #[test]
    fn aoc2021_20_case2() {
        let sol = make_test_solution();
        assert_eq!(sol.part_two(), "3351");
    }

    fn make_solution() -> io::Result<AoC2021_20> {
        AoC2021_20::new()
    }

    fn make_test_solution() -> AoC2021_20 {
        let data = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

        #..#.
        #....
        ##..#
        ..#..
        ..###
";
        AoC2021_20::parse_data(data)
    }
}
