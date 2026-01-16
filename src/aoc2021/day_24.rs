use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = isize;

#[derive(Debug, Clone)]
struct Parameters {
    div_z: Int,
    add_x: Int,
    add_y: Int,
}

#[derive(Debug, Clone, Copy)]
struct Equation {
    index: usize,
    value: Int,
}

type Equations = [Option<Equation>; 14];

fn make_equations(params: &[Parameters]) -> Equations {
    let mut result = [None; 14];
    let mut stack = Vec::new();
    for (i, p) in params.iter().enumerate() {
        if p.div_z == 1 {
            stack.push((i, p.clone()));
            continue;
        }
        if p.div_z == 26 {
            let (j, dep) = stack.pop().unwrap();
            let equation = Equation {
                index: j,
                value: p.add_x + dep.add_y,
            };
            result[i] = Some(equation);
            continue;
        }
        unreachable!()
    }
    result
}

fn find_number(range: &[Int], equations: &Equations) -> Int {
    let mut numbers: [Option<Int>; 14] = [None; 14];

    loop {
        let Some((index, _)) = numbers
            .iter()
            .enumerate()
            .find(|(i, x)| equations[*i].is_none() && x.is_none())
        else {
            break;
        };

        'range: for var in range {
            for (dep_index, eq) in equations.iter().enumerate() {
                let Some(eq) = eq else {
                    continue;
                };
                if eq.index != index {
                    continue;
                }
                let dep = *var + eq.value;
                if !(1..=9).contains(&dep) {
                    continue;
                }
                numbers[index] = Some(*var);
                numbers[dep_index] = Some(dep);
                break 'range;
            }
        }
        assert!(numbers[index].is_some());
    }

    numbers
        .iter()
        .inspect(|x| assert!(x.is_some()))
        .filter_map(|x| *x)
        .fold(0, |acc, x| acc * 10 + x)
}

pub struct AoC2021_24 {
    input: Vec<Parameters>,
}

impl AoC2021_24 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2021_24")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let last_number = |s: &T| -> Int {
            s.as_ref()
                .split_whitespace()
                .last()
                .and_then(|x| x.parse::<Int>().ok())
                .expect("Invalid instruction")
        };

        let input = lines
            .chunks(18)
            .map(|chunk| Parameters {
                div_z: last_number(&chunk[4]),
                add_x: last_number(&chunk[5]),
                add_y: last_number(&chunk[15]),
            })
            .collect::<Vec<_>>();

        Self { input }
    }

    fn find_number(&self, range: &[Int]) -> String {
        let equations = make_equations(&self.input);
        find_number(range, &equations).to_string()
    }
}

impl Solution for AoC2021_24 {
    fn part_one(&self) -> String {
        self.find_number(&(1..=9).rev().collect::<Vec<_>>())
    }

    fn part_two(&self) -> String {
        self.find_number(&(1..=9).collect::<Vec<_>>())
    }

    fn description(&self) -> String {
        "Day 24: Arithmetic Logic Unit".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_24_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2021_24_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "99919765949498");
        Ok(())
    }

    #[test]
    fn aoc2021_24_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "24913111616151");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2021_24> {
        AoC2021_24::new()
    }
}
