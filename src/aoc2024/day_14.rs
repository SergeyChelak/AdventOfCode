use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashSet;
use std::io;

type Int = isize;
type Point = Point2d<Int>;

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let (p, v) = value
            .split_once(' ')
            .expect("Spacer not found between <p, v>");
        let parse_point = |s: &str| -> Point {
            let (_, data) = s.split_once('=').expect("Unexpected format");
            let (x, y) = data.split_once(',').expect("Incorrect coordinates format");
            let x = x.parse::<Int>().expect("x coordinate isn't integer");
            let y = y.parse::<Int>().expect("y coordinate isn't integer");
            Point::new(x, y)
        };
        Self {
            position: parse_point(p),
            velocity: parse_point(v),
        }
    }
}

pub struct AoC2024_14 {
    input: Vec<Robot>,
    rows: Int,
    cols: Int,
}

impl AoC2024_14 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2024_14")?;
        Ok(Self::with_lines(&lines, 103, 101))
    }

    fn with_lines<T: AsRef<str>>(lines: &[T], rows: Int, cols: Int) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(Robot::from)
            .collect::<Vec<_>>();
        Self { input, rows, cols }
    }
}

impl Solution for AoC2024_14 {
    fn part_one(&self) -> String {
        let mut robots = self.input.clone();
        for _ in 0..100 {
            simulate(&mut robots, self.rows, self.cols);
        }
        safety_factor(&robots, self.rows, self.cols).to_string()
    }

    fn part_two(&self) -> String {
        let mut robots = self.input.clone();
        let mut seconds = 0;
        let len = robots.len();
        loop {
            seconds += 1;
            simulate(&mut robots, self.rows, self.cols);
            let set = robots.iter().map(|r| r.position).collect::<HashSet<_>>();
            if set.len() == len {
                break;
            }
        }
        seconds.to_string()
    }

    fn description(&self) -> String {
        "2024/Day 14: Restroom Redoubt".to_string()
    }
}

fn simulate(robots: &mut [Robot], rows: Int, cols: Int) {
    assert!(rows > 0 && cols > 0);
    robots.iter_mut().for_each(|r| {
        let x = (r.position.x + cols + r.velocity.x) % cols;
        let y = (r.position.y + rows + r.velocity.y) % rows;
        r.position = Point::new(x, y);
    });
}

fn safety_factor(robots: &[Robot], rows: Int, cols: Int) -> usize {
    let quadrants = {
        let q1 = (0..cols / 2, 0..rows / 2);
        let q4 = (0..cols / 2, rows / 2 + 1..rows);
        let q3 = (cols / 2 + 1..cols, 0..rows / 2);
        let q2 = (cols / 2 + 1..cols, rows / 2 + 1..rows);
        [q1, q2, q3, q4]
    };
    let mut count = [0usize; 4];
    for robot in robots {
        let p = robot.position;
        for (i, q) in quadrants.iter().enumerate() {
            if q.0.contains(&p.x) && q.1.contains(&p.y) {
                count[i] += 1;
            }
        }
    }
    count[0] * count[1] * count[2] * count[3]
}

// fn dump(robots: &[Robot], rows: Int, cols: Int) {
//     let set = robots.iter().map(|r| r.position).collect::<HashSet<_>>();
//     for row in 0..rows {
//         for col in 0..cols {
//             let point = Point::new(col, row);
//             if set.contains(&point) {
//                 print!("*")
//             } else {
//                 print!(" ")
//             }
//         }
//         println!()
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_14_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2024_14_parse_test() {
        let robot = Robot::from("p=0,4 v=3,-3");
        assert_eq!(0, robot.position.x);
        assert_eq!(4, robot.position.y);
        assert_eq!(3, robot.velocity.x);
        assert_eq!(-3, robot.velocity.y);
    }

    #[test]
    fn aoc2024_14_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "215987200");
        Ok(())
    }

    #[test]
    fn aoc2024_14_case_1() {
        let puzzle = make_test_solution();
        assert_eq!("12", puzzle.part_one());
    }

    #[test]
    fn aoc2024_14_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "8050");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2024_14> {
        AoC2024_14::new()
    }

    fn make_test_solution() -> AoC2024_14 {
        let lines = [
            "p=0,4 v=3,-3",
            "p=6,3 v=-1,-3",
            "p=10,3 v=-1,2",
            "p=2,0 v=2,-1",
            "p=0,0 v=1,3",
            "p=3,0 v=-2,-2",
            "p=7,6 v=-1,-3",
            "p=3,0 v=-1,-2",
            "p=9,3 v=2,3",
            "p=7,3 v=-1,2",
            "p=2,4 v=2,-3",
            "p=9,5 v=-3,-3",
        ];
        AoC2024_14::with_lines(&lines, 7, 11)
    }
}
