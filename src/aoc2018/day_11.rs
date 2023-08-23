use crate::solution::Solution;

use std::io;

type Int = isize;

pub struct AoC2018_11 {
    serial_number: Int,
}

impl AoC2018_11 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            serial_number: 9221,
        })
    }
}

impl Solution for AoC2018_11 {
    fn part_one(&self) -> String {
        let grid = make_grid(self.serial_number);
        let (mut x, mut y) = (0, 0);
        let mut max = Int::MIN;
        for (i, row) in grid.iter().enumerate().take(GRID_SIZE - 3) {
            for (j, _) in row.iter().enumerate().take(GRID_SIZE - 3) {
                let sum = grid[i][j]
                    + grid[i][j + 1]
                    + grid[i][j + 2]
                    + grid[i + 1][j]
                    + grid[i + 1][j + 1]
                    + grid[i + 1][j + 2]
                    + grid[i + 2][j]
                    + grid[i + 2][j + 1]
                    + grid[i + 2][j + 2];

                if max < sum {
                    max = sum;
                    x = i;
                    y = j;
                }
            }
        }
        format!("{},{}", x + 1, y + 1)
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 11: Chronal Charge".to_string()
    }
}

const GRID_SIZE: usize = 300;

fn make_grid(serial_number: Int) -> Vec<Vec<Int>> {
    let mut grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let x = (i + 1) as Int;
            let y = (j + 1) as Int;
            let rack_id = x + 10;
            let mut power = (rack_id * y + serial_number) * rack_id;
            let mut modulus = 0;
            for _ in 0..3 {
                modulus = power % 10;
                power /= 10;
            }
            grid[i][j] = modulus - 5;
        }
    }
    grid
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_11_correctness() -> io::Result<()> {
        let sol = AoC2018_11::new()?;
        assert_eq!(sol.part_one(), "20,77");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2018_11_make_grid() {
        let grid = make_grid(8);
        assert_eq!(grid[2][4], 4);

        let grid = make_grid(57);
        assert_eq!(grid[121][78], -5);

        // Fuel cell at 217,196, grid serial number 39: power level  0.
        let grid = make_grid(39);
        assert_eq!(grid[216][195], 0);

        // Fuel cell at 101,153, grid serial number 71: power level  4.
        let grid = make_grid(71);
        assert_eq!(grid[100][152], 4);
    }
}
