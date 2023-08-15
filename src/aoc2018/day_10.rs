use crate::solution::Solution;
use crate::utils::*;

use std::io;

type PointElem = Point2d<i32>;

#[derive(Clone, Copy)]
struct Point {
    position: PointElem,
    speed: PointElem,
}

impl Point {
    fn from_str(s: &str) -> Self {
        let idx = s.find("velocity").expect("velocity parameter is not found");
        let position = Self::parse_parameter_value(&s[..idx]);
        let speed = Self::parse_parameter_value(&s[idx..]);
        Self { position, speed }
    }

    fn parse_parameter_value(s: &str) -> PointElem {
        let mut s = s.trim().split_once('=').expect("'=' not found").1;
        s = remove_first_and_last(s);
        PointElem::parse_csv(s).unwrap()
    }
}

pub struct AoC2018_10 {
    points: Vec<Point>,
}

impl AoC2018_10 {
    pub fn new() -> io::Result<Self> {
        let points = read_file_as_lines("input/aoc2018_10")?
            .iter()
            .map(|x| Point::from_str(x))
            .collect::<Vec<Point>>();
        Ok(Self { points })
    }

    fn wait_for_message(&self, need_print: bool) -> usize {
        let mut data = self.points.clone();
        let mut min_square = usize::MAX;
        let mut time = 0;
        loop {
            let points = data.iter().map(|x| x.position).collect::<Vec<PointElem>>();
            let bounds = bounds(&points).expect("Bounds input is empty");
            let size = bounds.size();
            let square = size.x as usize * size.y as usize;
            if square > min_square {
                time -= 1;
                if need_print {
                    data.iter_mut().for_each(|elem| {
                        elem.position = elem.position.sub(&elem.speed);
                    });
                    let points = data.iter().map(|x| x.position).collect::<Vec<PointElem>>();
                    print_points(&points, &bounds);
                }
                break;
            }
            min_square = min_square.min(square);
            data.iter_mut().for_each(|elem| {
                elem.position = elem.position.add(&elem.speed);
            });
            time += 1;
        }
        time
    }
}

impl Solution for AoC2018_10 {
    fn part_one(&self) -> String {
        self.wait_for_message(true);
        "".to_string()
    }

    fn part_two(&self) -> String {
        self.wait_for_message(false).to_string()
    }

    fn description(&self) -> String {
        "AoC 2018/Day 10: The Stars Align".to_string()
    }
}

fn print_points(points: &[PointElem], bounds: &Bounds<i32>) {
    let size = bounds.size().add(&PointElem { x: 1, y: 1 });
    let cols = size.x as usize;
    let rows = size.y as usize;
    let mut matrix = vec![vec![0; cols]; rows];
    let points = normalize_with_bounds(points, bounds);
    points.iter().for_each(|p| {
        let x = p.x as usize;
        let y = p.y as usize;
        matrix[y][x] = 1;
    });
    for y in matrix.iter() {
        for val in y {
            let ch = if *val == 1 { '#' } else { ' ' };
            print!("{ch}");
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_10_input_load_test() -> io::Result<()> {
        let sol = AoC2018_10::new()?;
        assert!(!sol.points.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_10_correctness() -> io::Result<()> {
        let sol = AoC2018_10::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "10391");
        Ok(())
    }
}
