use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Reg = usize;
type Val = i32;

#[derive(Clone)]
enum Op {
    CpyReg(Reg, Reg),     // copies value from register to register
    CpyVal(Val, Reg),     // copies value to dest register
    Inc(Reg),
    Dec(Reg),
    JnzReg(Reg, Val),
    JnzVal(Val, Val)
}

impl Op {
    fn parse(s: &str) -> Self {
        todo!()
    }
}

struct Machine {
    reg: [i32; 4],
    pc: usize,
    program: Vec<Op>,
}

impl Machine {
    fn with_program(program: Vec<Op>) -> Self {
        Self {
            reg: [0; 4],
            pc: 0,
            program,
        }
    }

    fn run(&mut self) {
        while self.pc < self.program.len() {
            match self.program[self.pc] {
                Op::CpyReg(src, dest) => todo!(),
                Op::CpyVal(value, dest) => todo!(),
                Op::Inc(reg) => todo!(),
                Op::Dec(reg) => todo!(),
                Op::JnzReg(reg, offset) => todo!(),
                Op::JnzVal(value, offset) => todo!(),
            }
        }
    }

    fn reg_a(&self) -> Val {
        self.reg[0]
    }
}

pub struct AoC2016_12 {
    program: Vec<Op>,
}

impl AoC2016_12 {
    pub fn new() -> io::Result<Self> {
        let program = read_file_as_lines("input/aoc2016_12")?
            .iter()
            .map(|s| Op::parse(s))
            .collect();
        Ok(Self {
            program
        })
    }
}

impl Solution for AoC2016_12 {
    fn part_one(&self) -> String {
        let mut machine = Machine::with_program(self.program.clone());
        machine.run();
        machine.reg_a().to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2016/Day 12: Leonardo's Monorail".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_12_input_load_test() -> io::Result<()> {
        let sol = AoC2016_12::new()?;
        assert!(!sol.program.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2016_12_correctness() -> io::Result<()> {
        let sol = AoC2016_12::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}