use crate::solution::Solution;

use std::{
    fs::read_to_string,
    io::{self, Write},
};

use super::intcode_computer::*;

pub struct AoC2019_25 {
    program: Memory,
}

impl AoC2019_25 {
    pub fn new() -> io::Result<Self> {
        let data = read_to_string("input/aoc2019_25")?;
        Ok(Self {
            program: parse_program(&data),
        })
    }
}

impl Solution for AoC2019_25 {
    fn part_one(&self) -> String {
        let mut iter = [
            "west",
            "take mug",
            "north",
            "take easter egg",
            "south",
            "east",
            "south",
            "east",
            "north",
            "take candy cane",
            "south",
            "west",
            "north",
            "east",
            "take coin",
            "north",
            "east",
            "take manifold",
            "west",
            "north",
            "take hypercube",
            "south",
            "south",
            "south",
            "east",
            "take pointer",
            "west",
            "west",
            "take astrolabe",
            "north",
            "east",
            "north",
            "east",
            "drop pointer",
            "drop manifold",
            "drop easter egg",
            "drop candy cane",
            "east",
        ]
        .into_iter();
        let mut computer = IntcodeComputer::with_memory(&self.program);
        loop {
            let status = computer.run();
            match status {
                ExecutionStatus::Halted => break,
                ExecutionStatus::WaitForInput => {
                    let output = computer.sink_outputs_as_string();
                    if let Some(command) = iter.next() {
                        computer.push_input_str(command);
                    } else {
                        println!("{}", output);
                        // Wait for user input
                        print!("> ");
                        io::stdout().flush().unwrap();
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).unwrap();
                        computer.push_input_str(input.trim());
                    }
                    computer.push_input(10);
                }
                ExecutionStatus::WrongInstruction { .. } => panic!("{:?}", status),
            }
        }
        computer.sink_outputs_as_string()
    }

    fn description(&self) -> String {
        "Day 25: Cryostasis".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_25_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.program.is_empty());
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_25> {
        AoC2019_25::new()
    }
}
