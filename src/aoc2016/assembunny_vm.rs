// 
// Shared Assembunny Virtual Machine for AoC 2016
// Applied to day 12, 23 and 25
//
use std::cmp::Ordering;

type Reg = usize;
type Val = i32;

#[derive(Copy, Clone)]
enum Arg {
    Reg(Reg),
    Value(Val),
}

#[derive(Copy, Clone)]
enum Op {
    Cpy(Arg, Arg),
    Inc(Reg),
    Dec(Reg),
    Jnz(Arg, Arg),
    Tgl(Reg),
}

impl Op {
    fn parse(s: &str) -> Self {
        let tokens = s.split(' ').collect::<Vec<&str>>();
        match tokens[0] {
            "cpy" => Self::parse_cpy(&tokens),
            "inc" => Self::parse_inc(&tokens),
            "dec" => Self::parse_dec(&tokens),
            "jnz" => Self::parse_jnz(&tokens),
            "tgl" => Self::parse_tgl(&tokens),
            _ => panic!("Unexpected instruction {}", tokens[0]),
        }
    }

    fn parse_tgl(tokens: &[&str]) -> Self {
        Self::Tgl(Self::parse_reg(tokens[1]))
    }

    fn parse_inc(tokens: &[&str]) -> Self {
        Self::Inc(Self::parse_reg(tokens[1]))
    }

    fn parse_dec(tokens: &[&str]) -> Self {
        Self::Dec(Self::parse_reg(tokens[1]))
    }

    fn parse_cpy(tokens: &[&str]) -> Self {
        let arg1 = Self::parse_arg(tokens[1]);
        let arg2 = Self::parse_arg(tokens[2]);
        Self::Cpy(arg1, arg2)
    }

    fn parse_jnz(tokens: &[&str]) -> Self {
        let arg1 = Self::parse_arg(tokens[1]);
        let arg2 = Self::parse_arg(tokens[2]);
        Self::Jnz(arg1, arg2)
    }

    fn parse_arg(s: &str) -> Arg {
        if let Ok(val) = s.parse::<Val>() {
            Arg::Value(val)
        } else {
            let reg_idx = Self::parse_reg(s);
            Arg::Reg(reg_idx)
        }
    }

    fn parse_reg(s: &str) -> Reg {
        match s {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            _ => panic!("Unsupported register name {s}"),
        }
    }
}

pub struct Machine {
    reg: [i32; 4],
    pc: usize,
    program: Vec<Op>,
}

impl Machine {
    pub fn with_lines(lines: &[String]) -> Self {
        let program = lines.iter().map(|s| Op::parse(s)).collect();
        Self::with_program(program)
    }

    fn with_program(program: Vec<Op>) -> Self {
        Self {
            reg: [0; 4],
            pc: 0,
            program,
        }
    }

    pub fn run(&mut self) {
        while self.pc < self.program.len() {
            match self.program[self.pc] {
                Op::Cpy(src, dest) => self.op_copy(src, dest),
                Op::Inc(reg) => self.op_inc(reg),
                Op::Dec(reg) => self.op_dec(reg),
                Op::Jnz(value, offset) => self.op_jnz(value, offset),
                Op::Tgl(reg) => self.op_tgl(reg),
            }
        }
    }

    fn op_tgl(&mut self, reg: Reg) {
        if let Some(pos) = self.offset_pointer(self.reg[reg]) {
            match self.program[pos] {
                Op::Cpy(src, dest) => self.program[pos] = Op::Jnz(src, dest),
                Op::Inc(reg) => self.program[pos] = Op::Dec(reg),
                Op::Dec(reg) | Op::Tgl(reg) => self.program[pos] = Op::Inc(reg),
                Op::Jnz(value, offset) => self.program[pos] = Op::Cpy(value, offset),
            }
        }
        self.pc += 1;
    }

    fn op_inc(&mut self, reg: Reg) {
        self.reg[reg] += 1;
        self.pc += 1;
    }

    fn op_dec(&mut self, reg: Reg) {
        self.reg[reg] -= 1;
        self.pc += 1;
    }

    fn op_copy(&mut self, src: Arg, dest: Arg) {
        match (src, dest) {
            (Arg::Value(value), Arg::Reg(reg_idx)) => self.reg[reg_idx] = value,
            (Arg::Reg(src_reg_idx), Arg::Reg(reg_idx)) => self.reg[reg_idx] = self.reg[src_reg_idx],
            _ => {
                // invalid operation, just ignore
            }
        }
        self.pc += 1;
    }

    fn op_jnz(&mut self, val: Arg, offset: Arg) {
        let value = self.value(val);
        if value != 0 {
            let offset = self.value(offset);
            if let Some(new_pc) = self.offset_pointer(offset) {
                self.pc = new_pc;
                return;
            }
        }
        self.pc += 1;
    }

    fn value(&self, arg: Arg) -> Val {
        match arg {
            Arg::Value(v) => v,
            Arg::Reg(idx) => self.reg[idx],
        }
    }

    fn offset_pointer(&self, offset: Val) -> Option<usize> {
        match offset.cmp(&0) {
            Ordering::Equal => Some(self.pc),
            Ordering::Greater => {
                let position = offset as usize + self.pc;
                if position < self.program.len() {
                    Some(position)
                } else {
                    None
                }
            }
            Ordering::Less => {
                let offset = (-offset) as usize;
                if offset < self.pc {
                    Some(self.pc - offset)
                } else {
                    None
                }
            }
        }
    }

    pub fn reg_a(&self) -> Val {
        self.reg[0]
    }

    pub fn set_reg_a(&mut self, value: Val) {
        self.reg[0] = value
    }

    pub fn set_reg_c(&mut self, value: Val) {
        self.reg[2] = value
    }
}
