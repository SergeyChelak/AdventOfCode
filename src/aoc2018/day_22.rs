use crate::solution::Solution;
use crate::utils::*;

use std::io;

type UInt = usize;
type Coordinate = Point2d<UInt>;

#[derive(Clone, Copy)]
struct InputData {
    depth: usize,
    target: Coordinate,
}

impl InputData {
    fn new(depth: usize, x: UInt, y: UInt) -> Self {
        Self {
            depth,
            target: Coordinate::new(x, y),
        }
    }
}

type CaveRegions = Vec<Vec<UInt>>;

struct Cave {
    params: InputData,
    regions: CaveRegions,
}

impl Cave {
    fn new(params: InputData) -> Self {
        Self {
            params,
            regions: Default::default(),
        }
    }

    fn erosion_level(&self, val: UInt) -> UInt {
        (val + self.params.depth) % 20183
    }

    fn calculate_geologic_index(&mut self) {
        let (cols, rows) = {
            let coord = self.params.target;
            (coord.x + 1, coord.y + 1)
        };
        let (x, y) = {
            let coord = self.params.target;
            (coord.x, coord.y)
        };
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
        self.regions = regions
    }

    fn risk(&self) -> UInt {
        let mut result = 0;
        for row in self.regions.iter() {
            for val in row {
                let level = self.erosion_level(*val);
                result += level % 3;
            }
        }
        result
    }
}

pub struct AoC2018_22 {
    input: InputData,
}

impl AoC2018_22 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: InputData::new(9171, 7, 721),
        })
    }
}

impl Solution for AoC2018_22 {
    fn part_one(&self) -> String {
        let mut cave = Cave::new(self.input);
        cave.calculate_geologic_index();
        cave.risk().to_string()
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
        let sol = AoC2018_22 {
            input: InputData::new(510, 10, 10),
        };
        assert_eq!(sol.part_one(), "114")
    }

    #[test]
    fn aoc2018_22_correctness() -> io::Result<()> {
        let sol = AoC2018_22::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
