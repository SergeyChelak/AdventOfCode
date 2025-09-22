use crate::solution::Solution;
use crate::utils::not_found;

use std::fs::read_to_string;
use std::io;

type Int = usize;

pub struct AoC2019_02 {
    input: Vec<Int>,
}

impl AoC2019_02 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2019_02")?;
        Ok(Self::with_str(&input))
    }

    fn with_str(input: &str) -> Self {
        let input = input
            .trim()
            .split(',')
            .map(|s| s.parse::<Int>().expect("Invalid input"))
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2019_02 {
    fn part_one(&self) -> String {
        let mut data = self.input.clone();
        data[1] = 12;
        data[2] = 2;
        execute(&mut data);
        data[0].to_string()
    }

    fn part_two(&self) -> String {
        let target = 19690720;
        for noun in 0usize..=99 {
            for verb in 0usize..=99 {
                let mut data = self.input.clone();
                data[1] = noun;
                data[2] = verb;
                execute(&mut data);
                if data[0] == target {
                    let result = 100 * noun + verb;
                    return result.to_string();
                }
            }
        }
        not_found()
    }

    fn description(&self) -> String {
        "Day 2: 1202 Program Alarm".to_string()
    }
}

fn execute(data: &mut [Int]) {
    let size = data.len();
    let mut index = 0usize;
    while index + 4 < size {
        let slice = &data[index..index + 4];
        let opcode = slice[0];
        let first_at = slice[1];
        let first = data[first_at];
        let second_at = slice[2];
        let second = data[second_at];
        let result_at = slice[3];
        data[result_at] = match opcode {
            1 => first + second,
            2 => first * second,
            99 => break,
            x => panic!("Unexpected opcode {x}"),
        };
        index += 4;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_02_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_02_case_1() {
        let puzzle = AoC2019_02::with_str("1,9,10,3,2,3,11,0,99,30,40,50");
        let mut data = puzzle.input.clone();
        execute(&mut data);
        assert_eq!(data[0], 3500)
    }

    #[test]
    fn aoc2019_02_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "4690667");
        Ok(())
    }

    #[test]
    fn aoc2019_02_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "6255");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_02> {
        AoC2019_02::new()
    }
}
