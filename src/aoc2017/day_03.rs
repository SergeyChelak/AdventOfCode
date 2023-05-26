use crate::solution::Solution;

use std::io;

pub struct AoC2017_03 {
    input: u32,
}

impl AoC2017_03 {
    pub fn new() -> io::Result<Self> {
        Ok(Self { input: 277678 })
    }
}

impl Solution for AoC2017_03 {
    fn part_one(&self) -> String {
        distance(self.input).to_string()
    }

    fn part_two(&self) -> String {
        spiral_sum(self.input).to_string()
    }

    fn description(&self) -> String {
        "AoC 2017/Day 3: Spiral Memory".to_string()
    }
}

fn distance(num: u32) -> u32 {
    if num == 1 {
        return 0;
    }
    let dim = dim(num);
    let (mut x, mut y) = (dim - 1, dim - 2);
    let from = dim - 2;
    let mut pos = 1 + from * from;
    let mut dir = 0;
    while pos != num {
        match dir {
            0 => y -= 1,
            1 => x -= 1,
            2 => y += 1,
            3 => x += 1,
            _ => panic!("Invalid direction"),
        }
        if x == dim - 1 && y == 0 {
            dir = 1;
        }
        if x == 0 && y == 0 {
            dir = 2;
        }
        if x == 0 && y == dim - 1 {
            dir = 3;
        }
        pos += 1;
    }
    let r = dim / 2;
    let c = r;
    let abs = |a: u32, b: u32| a.max(b) - a.min(b);
    abs(r, x) + abs(c, y)
}

fn spiral_sum(num: u32) -> u32 {
    if num == 1 {
        return 0;
    }
    let dim = dim(num) as usize;
    let mut matrix = vec![vec![0u32; dim]; dim];
    let mut x = dim / 2;
    let mut y = x;
    matrix[x][y] = 1;
    let (mut leading, mut top, mut trailing, mut bottom) = (x - 1, y - 1, x + 1, y + 1);
    x += 1;
    let mut dir = 0;
    loop {
        let val = adj_sum(&matrix, x, y);
        if val > num {
            break val;
        }
        matrix[x][y] = val;
        match dir {
            0 => y -= 1,
            1 => x -= 1,
            2 => y += 1,
            3 => x += 1,
            _ => panic!("Invalid direction"),
        }
        if x == trailing && y == top {
            dir = 1;
        }
        if x == leading && y == top {
            dir = 2;
        }
        if x == leading && y == bottom {
            dir = 3;
        }
        if x == bottom + 1 && y == bottom {
            dir = 0;
            leading -= 1;
            top -= 1;
            trailing += 1;
            bottom += 1;
        }
    }
}

fn adj_sum(matrix: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let mut sum = 0;
    let dim = matrix.len();
    let x_inc = x < dim - 1;
    let x_dec = x > 0;
    let y_inc = y < dim - 1;
    let y_dec = y > 0;
    if x_inc {
        sum += matrix[x + 1][y];
    }
    if y_inc {
        sum += matrix[x][y + 1];
    }
    if x_dec {
        sum += matrix[x - 1][y];
    }
    if y_dec {
        sum += matrix[x][y - 1];
    }
    if x_inc && y_inc {
        sum += matrix[x + 1][y + 1];
    }
    if x_inc && y_dec {
        sum += matrix[x + 1][y - 1];
    }
    if x_dec && y_inc {
        sum += matrix[x - 1][y + 1];
    }
    if x_dec && y_dec {
        sum += matrix[x - 1][y - 1];
    }
    sum
}

fn dim(num: u32) -> u32 {
    if num == 1 {
        return 1;
    }
    let mut result = 3;
    while result * result < num {
        result += 2;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_03_correctness() -> io::Result<()> {
        let sol = AoC2017_03::new()?;
        assert_eq!(sol.part_one(), "475");
        assert_eq!(sol.part_two(), "279138");
        Ok(())
    }

    #[test]
    fn aoc2017_03_distance() {
        assert_eq!(distance(1), 0);
        assert_eq!(distance(12), 3);
        assert_eq!(distance(23), 2);
        assert_eq!(distance(1024), 31);
    }
}
