use crate::solution::Solution;
use crate::utils::*;

use std::io;

#[derive(Clone)]
enum Instruction {
    Hlf(usize),         // hlf r sets register r to half its current value, then continues with the next instruction.
    Tpl(usize),         // tpl r sets register r to triple its current value, then continues with the next instruction.
    Inc(usize),         // inc r increments register r, adding 1 to it, then continues with the next instruction.
    Jmp(isize),         // jmp offset is a jump; it continues with the instruction offset away relative to itself.
    Jie(usize, isize),  // jie r, offset is like jmp, but only jumps if register r is even ("jump if even").
    Jio(usize, isize)   // jio r, offset is like jmp, but only jumps if register r is 1 ("jump if one", not odd).
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let (instr, param) = s.split_once(" ")
            .expect("Incorrect instruction");
        match instr {
            "hlf" => Self::Hlf(Self::parse_reg(param)),
            "tpl" => Self::Tpl(Self::parse_reg(param)),
            "inc" => Self::Inc(Self::parse_reg(param)),
            "jmp" => Self::Jmp(Self::parse_offset(param)),
            "jie" => {
                let (reg, offs) = Self::parse_reg_offset(param);
                Self::Jie(reg, offs)
            },
            "jio" => {
                let (reg, offs) = Self::parse_reg_offset(param);
                Self::Jio(reg, offs)
            },
            _ => panic!("Unexpected instruction")
        }
    }

    fn parse_reg(s: &str) -> usize {
        match s {
            "a" => 0,
            "b" => 1,
            _ => panic!("Unexpected register name {s}")
        }
    }

    fn parse_offset(s: &str) -> isize {
        s.parse()
            .expect("Signed value expected for offset value")
    }

    fn parse_reg_offset(s: &str) -> (usize, isize) {
        let (reg, offs) = s.split_once(", ")
            .expect("Invalid format of <reg, offset> arguments");
        (Self::parse_reg(reg), Self::parse_offset(offs))
    }
}

struct Computer {
    pc: usize,              // program counter
    register: [usize; 2],
    memory: Vec<Instruction>,
}

impl Computer {
    fn with_program(memory: Vec<Instruction>) -> Self {
        Self {
            pc: 0,
            register: [0usize; 2],
            memory: memory
        }
    }

    fn run(&mut self) {
        todo!()
    }
}

pub struct AoC2015_23 {
    program: Vec<Instruction>
}

impl AoC2015_23 {
    pub fn new() -> io::Result<Self> {
        let program = read_file_as_lines("input/aoc2015_23")?
            .iter()
            .map(|s| Instruction::from_str(s))
            .collect::<Vec<Instruction>>();
        Ok(Self {
            program
        })
    }
}

impl Solution for AoC2015_23 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2015/Day 23: Opening the Turing Lock".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_23_input_load_test() -> io::Result<()> {
        let sol = AoC2015_23::new()?;
        assert_eq!(sol.program.len(), 46);
        Ok(())
    }

    #[test]
    fn aoc2015_23_correctness() -> io::Result<()> {
        let sol = AoC2015_23::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}