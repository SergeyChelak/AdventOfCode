use crate::solution::Solution;
use crate::utils::*;

use std::io;

const DIM: usize = 4;
type Int = i32;
type Coordinate = [Int; DIM];

pub struct AoC2018_25 {
    points: Vec<Coordinate>,
}

impl AoC2018_25 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2018_25")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let points = lines.iter().map(|s| parse_coordinate(s.as_str())).collect();
        Self { points }
    }
}

impl Solution for AoC2018_25 {
    fn part_one(&self) -> String {
        constellations(&self.points).to_string()
    }

    fn description(&self) -> String {
        "AoC 2018/Day 25: Four-Dimensional Adventure".to_string()
    }
}

fn constellations(points: &[Coordinate]) -> usize {
    let mut count = 0;
    let mut seen = vec![false; points.len()];
    for (i, _) in points.iter().enumerate() {
        if seen[i] {
            continue;
        }
        let mut len = 0;
        dfs(points, &mut seen, i, &mut len);
        count += 1;
    }
    count
}

fn dfs(points: &[Coordinate], seen: &mut Vec<bool>, pos: usize, len: &mut usize) {
    *len += 1;
    let from = &points[pos];
    seen[pos] = true;
    for (i, to) in points.iter().enumerate() {
        if seen[i] {
            continue;
        }
        if distance(from, to) <= 3 {
            dfs(points, seen, i, len)
        }
    }
}

fn distance(a: &Coordinate, b: &Coordinate) -> Int {
    a.iter().zip(b.iter()).map(|(a, b)| (a - b).abs()).sum()
}

fn parse_coordinate(s: &str) -> Coordinate {
    let coords = s
        .split(',')
        .map(|x| x.parse::<Int>().expect("Non integer coordinate value"))
        .collect::<Vec<Int>>();
    coords.try_into().expect("Invalid coordinate size")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_25_input_load_test() -> io::Result<()> {
        let sol = AoC2018_25::new()?;
        assert_eq!(sol.points.len(), 1498);
        Ok(())
    }

    #[test]
    fn aoc2018_25_ex1() {
        let inp = " 0,0,0,0
            3,0,0,0
            0,3,0,0
            0,0,3,0
            0,0,0,3
            0,0,0,6
            9,0,0,0
            12,0,0,0";
        let puzzle = from_str(inp);
        assert_eq!(puzzle.part_one(), "2")
    }

    #[test]
    fn aoc2018_25_ex2() {
        let inp = "-1,2,2,0
            0,0,2,-2
            0,0,0,-2
            -1,2,0,0
            -2,-2,-2,2
            3,0,2,-1
            -1,3,2,2
            -1,0,-1,0
            0,2,1,-2
            3,0,0,0";
        let puzzle = from_str(inp);
        assert_eq!(puzzle.part_one(), "4")
    }

    #[test]
    fn aoc2018_25_ex3() {
        let inp = "1,-1,0,1
            2,0,-1,0
            3,2,-1,0
            0,0,3,1
            0,0,-1,-1
            2,3,-2,0
            -2,2,0,0
            2,-2,0,-1
            1,-1,0,-1
            3,2,0,2";
        let puzzle = from_str(inp);
        assert_eq!(puzzle.part_one(), "3")
    }

    #[test]
    fn aoc2018_25_ex4() {
        let inp = "1,-1,-1,-2
            -2,-2,0,1
            0,2,1,3
            -2,3,-2,1
            0,2,3,-2
            -1,-1,1,-2
            0,-2,-1,0
            -2,2,3,-1
            1,2,2,0
            -1,-2,0,-2";
        let puzzle = from_str(inp);
        assert_eq!(puzzle.part_one(), "8")
    }

    fn from_str(s: &str) -> AoC2018_25 {
        let lines = s
            .split('\n')
            .map(|x| x.trim())
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        AoC2018_25::with_lines(&lines)
    }

    #[test]
    fn aoc2018_25_correctness() -> io::Result<()> {
        let sol = AoC2018_25::new()?;
        assert_eq!(sol.part_one(), "305");
        Ok(())
    }
}
