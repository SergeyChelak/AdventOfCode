use crate::solution::Solution;
use crate::utils::*;

use std::io;

/*
Addition:

    addr (add register) stores into register C the result of adding register A and register B.
    addi (add immediate) stores into register C the result of adding register A and value B.

Multiplication:

    mulr (multiply register) stores into register C the result of multiplying register A and register B.
    muli (multiply immediate) stores into register C the result of multiplying register A and value B.

Bitwise AND:

    banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
    bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.

Bitwise OR:

    borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
    bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.

Assignment:

    setr (set register) copies the contents of register A into register C. (Input B is ignored.)
    seti (set immediate) stores value A into register C. (Input B is ignored.)

Greater-than testing:

    gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
    gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
    gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.

Equality testing:

    eqir (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
    eqri (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
    eqrr (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
*/

#[derive(Clone, Copy, Debug)]
enum ParserState {
    None,
    Separator(usize),
    Log(TraceData),
    Instruction,
}

type Registers = [i32; 4];
type Instruction = [usize; 4];

#[derive(Default, Clone, Copy, Debug)]
struct TraceData {
    before: Registers,
    instr: Instruction,
    after: Registers,
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
        remove_first_and_last(values.trim())
            .split(", ")
            .map(|c| {
                let error = format!("Register should be int values {}", c);
                c.parse::<i32>().expect(&error)
            })
            .collect::<Vec<i32>>()
            .try_into()
            .expect("Amount for registers is incorrect")
    }

    fn parse_instruction(s: &str) -> Instruction {
        s.split(' ')
            .map(|x| {
                x.parse::<usize>()
                    .expect("Instruction values should be integers")
            })
            .collect::<Vec<usize>>()
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
        "".to_string()
    }

    // fn part_two(&self) -> String {
    // }

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
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
