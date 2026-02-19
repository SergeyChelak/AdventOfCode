use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = i32;

#[derive(Debug, Clone)]
enum Op {
    Add(Int),
    Nop,
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        if value == "noop" {
            return Self::Nop;
        }
        let Some(num) = value.strip_prefix("addx ") else {
            panic!("Invalid instruction {value}");
        };
        let num = num.parse::<Int>().expect("Addx argument must be integer");
        Self::Add(num)
    }
}

pub struct AoC2022_10 {
    input: Vec<Op>,
}

impl AoC2022_10 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2022_10")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(Op::from)
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2022_10 {
    fn part_one(&self) -> String {
        signal_strength(&self.input).iter().sum::<Int>().to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 10: Cathode-Ray Tube".to_string()
    }
}

struct OpData {
    operation: Op,
    cycles: Int,
}

impl OpData {
    fn new(operation: Op, cycles: Int) -> Self {
        Self { operation, cycles }
    }
}

fn signal_strength(ops: &[Op]) -> Vec<Int> {
    let mut cycle_counter = 0;
    let mut output = Vec::new();
    let mut x: Int = 1;

    let mut data: Option<OpData> = None;

    let mut iter = ops.iter();
    'runloop: loop {
        if data.is_none() {
            let Some(op) = iter.next() else {
                break 'runloop;
            };
            let cycles = match op {
                Op::Nop => 1,
                Op::Add(_) => 2,
            };
            data = Some(OpData::new(op.clone(), cycles));
        }

        let Some(op_data) = &mut data else {
            break 'runloop;
        };

        cycle_counter += 1;
        let strength = x * cycle_counter;

        if [20, 60, 100, 140, 180, 220].contains(&cycle_counter) {
            output.push(strength);
        }

        if op_data.cycles == 1 {
            match op_data.operation {
                Op::Add(val) => x += val,
                _ => {
                    // no op
                }
            }
        }
        op_data.cycles -= 1;
        if op_data.cycles == 0 {
            data = None;
        }
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_10_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2022_10_case_1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "13140")
    }

    #[test]
    fn aoc2022_10_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "14060");
        Ok(())
    }

    #[test]
    fn aoc2022_10_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2022_10> {
        AoC2022_10::new()
    }

    fn make_test_solution() -> AoC2022_10 {
        let input = [
            "addx 15", "addx -11", "addx 6", "addx -3", "addx 5", "addx -1", "addx -8", "addx 13",
            "addx 4", "noop", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1", "addx 5",
            "addx -1", "addx 5", "addx -1", "addx -35", "addx 1", "addx 24", "addx -19", "addx 1",
            "addx 16", "addx -11", "noop", "noop", "addx 21", "addx -15", "noop", "noop",
            "addx -3", "addx 9", "addx 1", "addx -3", "addx 8", "addx 1", "addx 5", "noop", "noop",
            "noop", "noop", "noop", "addx -36", "noop", "addx 1", "addx 7", "noop", "noop", "noop",
            "addx 2", "addx 6", "noop", "noop", "noop", "noop", "noop", "addx 1", "noop", "noop",
            "addx 7", "addx 1", "noop", "addx -13", "addx 13", "addx 7", "noop", "addx 1",
            "addx -33", "noop", "noop", "noop", "addx 2", "noop", "noop", "noop", "addx 8", "noop",
            "addx -1", "addx 2", "addx 1", "noop", "addx 17", "addx -9", "addx 1", "addx 1",
            "addx -3", "addx 11", "noop", "noop", "addx 1", "noop", "addx 1", "noop", "noop",
            "addx -13", "addx -19", "addx 1", "addx 3", "addx 26", "addx -30", "addx 12",
            "addx -1", "addx 3", "addx 1", "noop", "noop", "noop", "addx -9", "addx 18", "addx 1",
            "addx 2", "noop", "noop", "addx 9", "noop", "noop", "noop", "addx -1", "addx 2",
            "addx -37", "addx 1", "addx 3", "noop", "addx 15", "addx -21", "addx 22", "addx -6",
            "addx 1", "noop", "addx 2", "addx 1", "noop", "addx -10", "noop", "noop", "addx 20",
            "addx 1", "addx 2", "addx 2", "addx -6", "addx -11", "noop", "noop", "noop",
        ];
        AoC2022_10::parse_lines(&input)
    }
}
