use crate::solution::Solution;
use crate::utils::*;

use std::io;

use super::machine::{Instruction, Machine, MachineInt, Registers, TraceData, REGISTERS_COUNT};

#[derive(Clone, Copy, Debug)]
enum ParserState {
    None,
    Separator(usize),
    Log(TraceData),
    Instruction,
}

struct Parser {
    input: Vec<String>,
    out_1: Vec<TraceData>,
    out_2: Vec<Instruction>,
    state: ParserState,
}

impl Parser {
    fn new(input: Vec<String>) -> Self {
        Self {
            input,
            out_1: Vec::new(),
            out_2: Vec::new(),
            state: ParserState::None,
        }
    }

    #[allow(clippy::field_reassign_with_default)]
    fn parse(&mut self) {
        for s in &self.input {
            let s = s.trim();
            if s.starts_with("Before") {
                let mut log_item = TraceData::default();
                log_item.before = Self::parse_reg_values(s);
                self.state = ParserState::Log(log_item);
            } else if s.starts_with("After") {
                let ParserState::Log(log_item) = &mut self.state else {
                    panic!("Invalid parser state (1)");
                };
                log_item.after = Self::parse_reg_values(s);
            } else if s.is_empty() {
                let next_state = match &self.state {
                    ParserState::Log(log_item) => {
                        self.out_1.push(*log_item);
                        ParserState::Separator(1)
                    }
                    ParserState::Separator(count) => {
                        if *count > 1 {
                            ParserState::Instruction
                        } else {
                            ParserState::Separator(1 + count)
                        }
                    }
                    _ => self.state,
                };
                self.state = next_state;
            } else {
                let instr = Self::parse_instruction(s);
                match &mut self.state {
                    ParserState::Log(log_item) => log_item.instr = instr,
                    ParserState::Instruction => self.out_2.push(instr),
                    _ => panic!("Invalid parser state (3) {:?}", self.state),
                }
            }
        }
    }

    fn parse_reg_values(s: &str) -> Registers {
        let (_, values) = s.split_once(": ").expect("Invalid before/after state");
        let mut registers = [0; REGISTERS_COUNT];
        remove_first_and_last(values.trim())
            .split(", ")
            .map(|c| {
                let error = format!("Register should be int values {}", c);
                c.parse::<MachineInt>().expect(&error)
            })
            .enumerate()
            .for_each(|(i, val)| {
                registers[i] = val;
            });
        registers
    }

    fn parse_instruction(s: &str) -> Instruction {
        s.split(' ')
            .map(|x| {
                x.parse::<MachineInt>()
                    .expect("Instruction values should be integers")
            })
            .collect::<Vec<MachineInt>>()
            .try_into()
            .expect("Amount of instruction parameters is invalid")
    }
}

pub struct AoC2018_16 {
    input_1: Vec<TraceData>,
    input_2: Vec<Instruction>,
}

impl AoC2018_16 {
    pub fn new() -> io::Result<Self> {
        let arr = read_file_as_lines("input/aoc2018_16")?;
        let mut parser = Parser::new(arr);
        parser.parse();
        Ok(Self {
            input_1: parser.out_1,
            input_2: parser.out_2,
        })
    }
}

impl Solution for AoC2018_16 {
    fn part_one(&self) -> String {
        let mut machine = Machine::new();
        self.input_1
            .iter()
            .map(|data| machine.ambiguous_count(data))
            .filter(|x| *x > 2)
            .count()
            .to_string()
    }

    fn part_two(&self) -> String {
        let mut count = 0usize;
        let mut machine = Machine::new();
        loop {
            self.input_1.iter().for_each(|data| machine.try_remap(data));
            let next = machine.remap_count();
            if next == count {
                break;
            }
            count = next;
        }
        assert_eq!(
            count,
            machine.instructions_count(),
            "Failed to remap instructions"
        );
        machine.reset();
        self.input_2.iter().for_each(|arg| machine.exec(*arg));
        machine.reg(0).to_string()
    }

    fn description(&self) -> String {
        "AoC 2018/Day 16: Chronal Classification".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_16_input_load_test() -> io::Result<()> {
        let sol = AoC2018_16::new()?;
        assert!(!sol.input_1.is_empty());
        assert!(!sol.input_2.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_16_correctness() -> io::Result<()> {
        let sol = AoC2018_16::new()?;
        assert_eq!(sol.part_one(), "570");
        assert_eq!(sol.part_two(), "503");
        Ok(())
    }
}
