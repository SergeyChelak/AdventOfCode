use crate::solution::Solution;
use crate::utils::*;

use std::io;

type UInt = usize;
type Coordinate = Point2d<UInt>;
type CaveRegions = Vec<Vec<UInt>>;

pub struct AoC2018_22 {
    depth: usize,
    target: Coordinate,
}

impl AoC2018_22 {
    pub fn new() -> io::Result<Self> {
        Ok(Self::with_parameters(9171, 7, 721))
    }

    fn with_parameters(depth: usize, x: UInt, y: UInt) -> Self {
        Self {
            depth,
            target: Coordinate::new(x, y),
        }
    }

    fn geologic_map(&self) -> CaveRegions {
        let (cols, rows) = (self.target.x + 1, self.target.y + 1);
        let (x, y) = (self.target.x, self.target.y);
        let mut regions = CaveRegions::with_capacity(rows);
        for row in 0..rows {
            let mut arr = Vec::with_capacity(cols);
            for col in 0..cols {
                if (col, row) == (0, 0) || (col, row) == (x, y) {
                    arr.push(0);
                    continue;
                }
                // If the region's Y coordinate is 0, the geologic index is its X coordinate times 16807.
                if row == 0 {
                    arr.push(col * 16807);
                    continue;
                }
                // If the region's X coordinate is 0, the geologic index is its Y coordinate times 48271.
                if col == 0 {
                    arr.push(row * 48271);
                    continue;
                }
                let gi_1 = self.erosion_level(arr[col - 1]);
                let gi_2 = self.erosion_level(regions.last().unwrap()[col]);
                arr.push(gi_1 * gi_2);
            }
            regions.push(arr);
        }
        regions
    }

    fn erosion_level(&self, val: UInt) -> UInt {
        (val + self.depth) % 20183
    }

    fn risk(&self, regions: &CaveRegions) -> UInt {
        let mut result = 0;
        for row in regions.iter() {
            for val in row {
                let level = self.erosion_level(*val);
                result += level % 3;
            }
        }
        result
    }
}

impl Solution for AoC2018_22 {
    fn part_one(&self) -> String {
        let map = self.geologic_map();
        self.risk(&map).to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 22: Mode Maze".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_22_example1() {
        let sol = AoC2018_22::with_parameters(510, 10, 10);
        assert_eq!(sol.part_one(), "114")
    }

    #[test]
    fn aoc2018_22_correctness() -> io::Result<()> {
        let sol = AoC2018_22::new()?;
        assert_eq!(sol.part_one(), "5786");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
