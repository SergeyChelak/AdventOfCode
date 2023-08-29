use crate::solution::Solution;
use crate::utils::coordinate::*;
use std::io;

type Int = isize;
type Position = Point2d<usize>;

pub struct AoC2018_11 {
    serial_number: Int,
}

impl AoC2018_11 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            serial_number: 9221,
        })
    }

    fn max_square(&self, grid: &Grid, dim: usize) -> (Position, Int) {
        let (mut x, mut y) = (0, 0);
        let mut max = Int::MIN;
        for (i, row) in grid.iter().enumerate().take(GRID_SIZE - dim) {
            for (j, _) in row.iter().enumerate().take(GRID_SIZE - dim) {
                let sum = sum(&grid, i, j, dim);
                if max < sum {
                    max = sum;
                    x = i;
                    y = j;
                }
            }
        }
        let position = Position { x, y };
        (position, max)
    }
}

impl Solution for AoC2018_11 {
    fn part_one(&self) -> String {
        let grid = make_grid(self.serial_number);
        let (pos, _) = self.max_square(&grid, 3);
        format!("{},{}", pos.x + 1, pos.y + 1)
    }

    fn part_two(&self) -> String {
        let grid = make_grid(self.serial_number);
        let mut position = Position { x: 0, y: 0 };
        let mut size = 0;
        let mut max = Int::MIN;
        for dim in 0..GRID_SIZE {
            let (pos, power) = self.max_square(&grid, dim);
            if max < power {
                max = power;
                position = pos;
                size = dim;
            }
        }
        format!("{},{},{}", position.x + 1, position.y + 1, size)
    }

    fn description(&self) -> String {
        "AoC 2018/Day 11: Chronal Charge".to_string()
    }
}

const GRID_SIZE: usize = 300;
type Grid = Vec<Vec<Int>>; //[[Int; GRID_SIZE]; GRID_SIZE];

fn make_grid(serial_number: Int) -> Grid {
    let mut grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];
    for (i, row) in grid.iter_mut().enumerate() {
        for (j, val) in row.iter_mut().enumerate() {
            let x = (i + 1) as Int;
            let y = (j + 1) as Int;
            let rack_id = x + 10;
            let mut power = (rack_id * y + serial_number) * rack_id;
            let mut modulus = 0;
            for _ in 0..3 {
                modulus = power % 10;
                power /= 10;
            }
            *val = modulus - 5;
        }
    }
    grid
}

fn sum(grid: &Grid, i: usize, j: usize, size: usize) -> Int {
    let mut sum = 0;
    for r in 0..size {
        for c in 0..size {
            sum += grid[i + r][j + c];
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_11_correctness() -> io::Result<()> {
        let sol = AoC2018_11::new()?;
        assert_eq!(sol.part_one(), "20,77");
        assert_eq!(sol.part_two(), "143,57,10");
        Ok(())
    }

    #[test]
    fn aoc2018_11_make_grid() {
        let grid = make_grid(8);
        assert_eq!(grid[2][4], 4);

        let grid = make_grid(57);
        assert_eq!(grid[121][78], -5);

        let grid = make_grid(39);
        assert_eq!(grid[216][195], 0);

        let grid = make_grid(71);
        assert_eq!(grid[100][152], 4);
    }
}
