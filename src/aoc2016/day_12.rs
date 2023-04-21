use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Reg = usize;
type Val = i32;

#[derive(Clone)]
enum Op {
    CpyReg(Reg, Reg),
    Cpy(Val, Reg),
    Inc(Reg),
    Dec(Reg),
    JnzReg(Reg, Val),
    Jnz(Val, Val)
}

impl Op {
    fn parse(s: &str) -> Self {
        let tokens = s.split(' ').collect::<Vec<&str>>();
        match tokens[0] {
            "cpy" => Self::parse_cpy(&tokens),
            "inc" => Self::parse_inc(&tokens),
            "dec" => Self::parse_dec(&tokens),
            "jnz" => Self::parse_jnz(&tokens),
            _ => panic!("Unexpected instruction {}", tokens[0]),
        }
    }

    fn parse_inc(tokens: &[&str]) -> Self {
        Self::Inc(Self::parse_reg(tokens[1]))
    }

    fn parse_dec(tokens: &[&str]) -> Self {
        Self::Dec(Self::parse_reg(tokens[1]))
    }

    fn parse_cpy(tokens: &[&str]) -> Self {
        let dest_reg = Self::parse_reg(tokens[2]);
        if let Ok(value) = tokens[1].parse::<Val>() {
            Self::Cpy(value, dest_reg)
        } else {
            let src_reg = Self::parse_reg(tokens[1]);
            Self::CpyReg(src_reg, dest_reg)
        }
    }

    fn parse_jnz(tokens: &[&str]) -> Self {
        let offset = tokens[2].parse::<Val>()
            .expect("jnz offset should be int");
        if let Ok(value) = tokens[1].parse::<Val>() {
            Self::Jnz(value, offset)
        } else {
            let reg = Self::parse_reg(tokens[1]);
            Self::JnzReg(reg, offset)
        }
    }

    fn parse_reg(s: &str) -> Reg {
        match s {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            _ => panic!("Unsupported register name {s}")
        }
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
                Op::CpyReg(src, dest) => self.op_copy_reg(src, dest),
                Op::Cpy(value, dest) => self.op_copy(value, dest),
                Op::Inc(reg) => self.op_inc(reg),
                Op::Dec(reg) => self.op_dec(reg),
                Op::JnzReg(reg, offset) => self.op_jnz_reg(reg, offset),
                Op::Jnz(value, offset) => self.op_jnz(value, offset),
            }
        }
    }

    fn op_inc(&mut self, reg: Reg) {
        self.reg[reg] += 1;
        self.pc += 1;
    }

    fn op_dec(&mut self, reg: Reg) {
        self.reg[reg] -= 1;
        self.pc += 1;
    }

    fn op_copy(&mut self, val: Val, reg: Reg) {
        self.reg[reg] = val;
        self.pc += 1;
    }

    fn op_copy_reg(&mut self, src: Reg, dest: Reg) {
        let val = self.reg[src];
        self.op_copy(val, dest);
    }

    fn op_jnz(&mut self, val: Val, offset: Val) {
        if val != 0 {
            let new_pc = self.pc as Val + offset;
            self.pc = new_pc as usize;
        } else {
            self.pc += 1;
        }
    }

    fn op_jnz_reg(&mut self, reg: Reg, offset: Val) {
        self.op_jnz(self.reg[reg], offset)
    }

    fn reg_a(&self) -> Val {
        self.reg[0]
    }

    fn set_reg_c(&mut self, value: Val) {
        self.reg[2] = value
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

    fn part_two(&self) -> String {
        let mut machine = Machine::with_program(self.program.clone());
        machine.set_reg_c(1);
        machine.run();
        machine.reg_a().to_string()
    }

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
        assert_eq!(sol.part_one(), "318007");
        assert_eq!(sol.part_two(), "9227661");
        Ok(())
    }
}