use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = usize;

struct Rect {
    width: Int,
    height: Int,
}

impl Rect {
    fn from_str(s: &str) -> Self {
        let (width, height) = s.split_once('x').expect("Can't parse rect dimensions");
        let width = width.parse::<Int>().expect("Can't parse rect dim: Width");
        let height = height.parse::<Int>().expect("Can't parse rect dim: Height");
        Self { width, height }
    }
}

struct Origin {
    left: Int,
    top: Int,
}

impl Origin {
    fn from_str(s: &str) -> Self {
        let len = s.len();
        let s = &s[..len - 1];
        let (left, top) = s.split_once(',').expect("Can't parse origin");
        let left = left.parse::<Int>().expect("Can't parse origin: Left");
        let top = top.parse::<Int>().expect("Can't parse origin: Top");
        Self { left, top }
    }
}

struct Claim {
    rect: Rect,
    origin: Origin,
}

impl Claim {
    fn from_str(s: &str) -> Self {
        let tokens = s.split(' ').collect::<Vec<&str>>();
        let origin = Origin::from_str(tokens[2]);
        let rect = Rect::from_str(tokens[3]);
        Self { origin, rect }
    }
}

const SQUARE_SIZE: usize = 1000;
type Square = [[usize; SQUARE_SIZE]; SQUARE_SIZE];

pub struct AoC2018_03 {
    input: Vec<Claim>,
}

impl AoC2018_03 {
    pub fn new() -> io::Result<Self> {
        let input = read_file_as_lines("input/aoc2018_03")?
            .iter()
            .map(|s| Claim::from_str(s))
            .collect();
        Ok(Self { input })
    }

    #[allow(clippy::needless_range_loop)]
    fn fill_square(&self) -> Square {
        let mut square = [[0usize; SQUARE_SIZE]; SQUARE_SIZE];
        self.input.iter().for_each(|claim| {
            let top = claim.origin.top;
            let left = claim.origin.left;
            for i in top..top + claim.rect.height {
                for j in left..left + claim.rect.width {
                    square[i][j] += 1;
                }
            }
        });
        square
    }
}

impl Solution for AoC2018_03 {
    fn part_one(&self) -> String {
        self.fill_square()
            .iter()
            .map(|row| row.iter().filter(|&x| *x > 1).count())
            .sum::<usize>()
            .to_string()
    }

    #[allow(clippy::needless_range_loop)]
    fn part_two(&self) -> String {
        let square = self.fill_square();
        self.input
            .iter()
            .enumerate()
            .find(|(_, claim)| {
                let top = claim.origin.top;
                let left = claim.origin.left;
                for i in top..top + claim.rect.height {
                    for j in left..left + claim.rect.width {
                        if square[i][j] != 1 {
                            return false;
                        }
                    }
                }
                true
            })
            .map(|(i, _)| i + 1)
            .expect("Non-overlap claim doesn't found")
            .to_string()
    }

    fn description(&self) -> String {
        "AoC 2018/Day 3: No Matter How You Slice It".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_03_input_load_test() -> io::Result<()> {
        let sol = AoC2018_03::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_03_correctness() -> io::Result<()> {
        let sol = AoC2018_03::new()?;
        assert_eq!(sol.part_one(), "117505");
        assert_eq!(sol.part_two(), "1254");
        Ok(())
    }
}
