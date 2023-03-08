use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Grid = Vec<Vec<i32>>;

fn switch_lights(input: &Grid) -> Grid {
    let rows = input.len();
    let mut result = Grid::with_capacity(rows);
    input.iter()
        .map(|arr| arr.len())
        .for_each(|s| {
            result.push(vec![0; s]);
        });
    
    for i in 0..rows {
        let cols = input[i].len();
        for j in 0..cols {
            let offs = vec![
                (-1, -1), // 1
                ( 0, -1), // 2
                ( 1, -1), // 3
                (-1,  0), // 4
                ( 1,  0), // 5
                (-1,  1), // 6
                ( 0,  1), // 7
                ( 1,  1)  // 8
            ];
            let on_count = offs.iter()                
                .map(|(dx, dy)| (i as i32 + *dx, j as i32 + *dy))
                .filter(|(r, c)| {
                    (0..rows as i32).contains(r) && (0..cols as i32).contains(c)
                })
                .map(|(x, y)| input[x as usize][y as usize])
                .sum::<i32>();
            result[i][j] = if input[i][j] == 1 {
                if matches!(on_count, 2 | 3) { 1 } else { 0 }
            } else {
                if on_count == 3 { 1 } else { 0 }
            }
        }
    }
    result
}

fn lights_count(input: &Grid) -> i32 {
    input.iter()
        .map(|arr| arr.iter().sum::<i32>())
        .sum()
}

fn turn_on_corners(lights: &mut Grid) {
    let n = lights.len() - 1;
    lights[0][0] = 1;
    lights[0][n] = 1;
    lights[n][0] = 1;
    lights[n][n] = 1;
}

pub struct AoC2015_18 {
    grid: Grid
}

impl AoC2015_18 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2015_18")?;
        Ok(Self {
            grid: Self::parse_lines(&lines)
        })
    }

    fn parse_lines(lines: &[String]) -> Grid {
        lines.iter()
            .map(|s| 
                s.chars()
                  .map(|ch| if ch == '.' { 0 } else { 1 })
                  .collect::<Vec<i32>>())
            .collect::<Grid>()
    }
}

impl Solution for AoC2015_18 {
    fn part_one(&self) -> String {
        let mut lights = self.grid.clone();
        for _ in 0..100 {
            lights = switch_lights(&lights);
        }
        lights_count(&lights)
            .to_string()
    }

    fn part_two(&self) -> String {
        let mut lights = self.grid.clone();
        turn_on_corners(&mut lights);
        for _ in 0..100 {
            lights = switch_lights(&lights);
            turn_on_corners(&mut lights);
        }
        lights_count(&lights)
            .to_string()
    }

    fn description(&self) -> String {
        "AoC 2015/Day 18: Like a GIF For Your Yard".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_18_input_load_test() -> io::Result<()> {
        let sol = AoC2015_18::new()?;
        assert_eq!(sol.grid.len(), 100);
        Ok(())
    }

    #[test]
    fn aoc2015_18_correctness() -> io::Result<()> {
        let sol = AoC2015_18::new()?;
        assert_eq!(sol.part_one(), "1061");
        assert_eq!(sol.part_two(), "1006");
        Ok(())
    }

    #[test]
    fn aoc2015_18_case6x6() {
        let mut grid = lines_to_grid(&vec![
            ".#.#.#",
            "...##.",
            "#....#",
            "..#...",
            "#.#..#",
            "####.."
        ]);

        for _ in 0..4 {
            grid = switch_lights(&grid);
        }

        let expected_grid = lines_to_grid(&vec![
            "......",
            "......",
            "..##..",
            "..##..",
            "......",
            "......"
        ]);

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                assert_eq!(grid[i][j], expected_grid[i][j]);
            }
        }
    }

    fn lines_to_grid(input: &[&str]) -> Grid {
        let lines = input.iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        AoC2015_18::parse_lines(&lines)
    }
}