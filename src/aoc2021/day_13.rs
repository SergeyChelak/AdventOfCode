use crate::{
    solution::Solution,
    utils::{not_found, Point2d},
};

use std::{collections::HashSet, io};

type Int = usize;
type Dot = Point2d<Int>;

enum Fold {
    Up(Int),
    Left(Int),
}

impl From<&str> for Fold {
    fn from(value: &str) -> Self {
        let (axe, num) = value
            .strip_prefix("fold along")
            .expect("Invalid fold format")
            .trim()
            .split_once('=')
            .expect("Invalid fold format");
        let num = num.parse::<Int>().expect("Invalid value format");
        match axe {
            "x" => Fold::Left(num),
            "y" => Fold::Up(num),
            _ => unreachable!(),
        }
    }
}

pub struct AoC2021_13 {
    dots: Vec<Dot>,
    folds: Vec<Fold>,
}

impl AoC2021_13 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2021_13")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        let (dots, folds) = data.split_once("\n\n").expect("Invalid input format");

        let dots = dots
            .split('\n')
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|s| Dot::parse_csv(s).expect("Invalid dot format"))
            .collect::<Vec<_>>();

        let folds = folds
            .split('\n')
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(Fold::from)
            .collect::<Vec<_>>();

        Self { dots, folds }
    }
}

impl Solution for AoC2021_13 {
    fn part_one(&self) -> String {
        let mut dots = make_set(&self.dots);
        let Some(first) = self.folds.first() else {
            return not_found();
        };
        fold(&mut dots, first);
        dots.len().to_string()
    }

    fn part_two(&self) -> String {
        let mut dots = make_set(&self.dots);
        self.folds.iter().for_each(|f| fold(&mut dots, f));

        let (Some(max_x), Some(max_y)) =
            (find_max(&dots, |dot| dot.x), find_max(&dots, |dot| dot.y))
        else {
            return not_found();
        };

        for row in 0..=max_y {
            for col in 0..=max_x {
                let dot = Dot::new(col, row);
                let ch = if dots.contains(&dot) { '❚' } else { ' ' };
                print!("{ch}");
            }
            println!()
        }

        "".to_string()
    }

    fn description(&self) -> String {
        "Day 13: Transparent Origami".to_string()
    }
}

fn fold(dots: &mut HashSet<Dot>, fold: &Fold) {
    match fold {
        Fold::Left(col) => fold_ex(
            dots,
            |dot| dot.x > *col,
            |dot| Dot::new(2 * col - dot.x, dot.y),
        ),
        Fold::Up(row) => fold_ex(
            dots,
            |dot| dot.y > *row,
            |dot| Dot::new(dot.x, 2 * row - dot.y),
        ),
    }
}

fn fold_ex(
    dots: &mut HashSet<Dot>,
    filter: impl Fn(&Dot) -> bool,
    transform: impl Fn(&Dot) -> Dot,
) {
    let new_dots = dots
        .iter()
        .filter(|dot| filter(dot))
        .map(transform)
        .collect::<Vec<_>>();
    dots.extend(new_dots.iter());
    dots.retain(|dot| !filter(dot));
}

fn find_max(dots: &HashSet<Dot>, transform: impl Fn(&Dot) -> Int) -> Option<usize> {
    dots.iter().map(transform).max()
}

fn make_set(dots: &[Dot]) -> HashSet<Dot> {
    dots.iter().copied().collect::<HashSet<_>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_13_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.dots.is_empty());
        assert!(!sol.folds.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2021_13_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "666");
        Ok(())
    }

    #[test]
    fn aoc2021_13_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2021_13_case_1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "17");
    }

    fn make_solution() -> io::Result<AoC2021_13> {
        AoC2021_13::new()
    }

    fn make_test_solution() -> AoC2021_13 {
        let data = "6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5
";
        AoC2021_13::parse_data(data)
    }
}
