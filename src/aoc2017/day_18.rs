use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Register = usize;
type Value = i64;
enum OpValue {
    Val(Value),
    Reg(Register),
}

enum Op {
    Snd(Register),
    Set(Register, OpValue),
    Add(Register, OpValue),
    Mul(Register, OpValue),
    Mod(Register, OpValue),
    Rcv(Register),
    Jgz(OpValue, OpValue),
}

impl Op {
    fn from_str(s: &str) -> Self {
        let tokens = s.split(' ').collect::<Vec<&str>>();
        match tokens[0] {
            "snd" => Self::Snd(Self::parse_reg(tokens[1])),
            "set" => Self::Set(Self::parse_reg(tokens[1]), Self::parse_op_value(tokens[2])),
            "add" => Self::Add(Self::parse_reg(tokens[1]), Self::parse_op_value(tokens[2])),
            "mul" => Self::Mul(Self::parse_reg(tokens[1]), Self::parse_op_value(tokens[2])),
            "mod" => Self::Mod(Self::parse_reg(tokens[1]), Self::parse_op_value(tokens[2])),
            "rcv" => Self::Rcv(Self::parse_reg(tokens[1])),
            "jgz" => Self::Jgz(
                Self::parse_op_value(tokens[1]),
                Self::parse_op_value(tokens[2]),
            ),
            _ => panic!("Unexpected command {}", tokens[0]),
        }
    }

    fn parse_op_value(s: &str) -> OpValue {
        if let Ok(val) = s.parse::<Value>() {
            OpValue::Val(val)
        } else {
            OpValue::Reg(Self::parse_reg(s))
        }
    }

    fn parse_reg(s: &str) -> Register {
        let ch = s.parse::<char>().expect("Register name should be char");
        assert!(ch.is_alphabetic(), "{} isn't alphabetic character", ch);
        (ch as u8 - b'a') as usize
    }
}

trait Module {
    fn op_snd(&mut self, value: Value);

    fn op_rcv(&mut self, reg_value: &mut Value) -> bool;

    fn output(&self) -> Option<Value>;
}

struct SoundModule {
    sound_freq: Option<Value>,
}

impl SoundModule {
    fn new() -> Self {
        Self {
            sound_freq: None,
        }
    }
}

impl Module for SoundModule {
    fn op_snd(&mut self, value: Value) {
        self.sound_freq = Some(value);
    }

    fn op_rcv(&mut self, reg_value: &mut Value) -> bool {
        *reg_value == 0
    }

    fn output(&self) -> Option<Value> {
        self.sound_freq
    }
}

impl Default for SoundModule {
    fn default() -> Self {
        Self::new()
    }
}

struct Machine<'a> {
    register: [Value; 26],
    pc: usize,
    ops: &'a [Op],
    is_suspended: bool,
    module: Box<dyn Module>,
}

impl<'a> Machine<'a> {
    fn new(ops: &'a [Op], module: Box<dyn Module>) -> Self {
        Self {
            register: [0; 26],
            pc: 0,
            ops,
            is_suspended: false,
            module
        }
    }

    fn run(&mut self) {
        while self.pc < self.ops.len() && !self.is_suspended {
            match &self.ops[self.pc] {
                Op::Snd(reg) => self.op_snd(*reg),
                Op::Set(reg, op_value) => self.op_set(*reg, op_value),
                Op::Add(reg, op_value) => self.op_add(*reg, op_value),
                Op::Mul(reg, op_value) => self.op_mul(*reg, op_value),
                Op::Mod(reg, op_value) => self.op_mod(*reg, op_value),
                Op::Rcv(reg) => self.op_rcv(*reg),
                Op::Jgz(op_value, offset) => self.op_jgz(op_value, offset),
            }
        }
    }

    fn op_set(&mut self, reg: usize, op_value: &OpValue) {
        let val = self.get_value(op_value);
        self.register[reg] = val;
        self.pc += 1;
    }

    fn op_add(&mut self, reg: usize, op_value: &OpValue) {
        let val = self.get_value(op_value);
        self.register[reg] += val;
        self.pc += 1;
    }

    fn op_mul(&mut self, reg: usize, op_value: &OpValue) {
        let val = self.get_value(op_value);
        self.register[reg] *= val;
        self.pc += 1;
    }

    fn op_mod(&mut self, reg: usize, op_value: &OpValue) {
        let val = self.get_value(op_value);
        self.register[reg] %= val;
        self.pc += 1;
    }

    fn op_jgz(&mut self, op_value: &OpValue, offset: &OpValue) {
        let x = self.get_value(op_value);
        if x > 0 {
            let offset = self.get_value(offset);
            self.pc_offset(offset);
        } else {
            self.pc += 1;
        }
    }

    fn op_snd(&mut self, reg: usize) {
        let val = self.register[reg];
        self.module.op_snd(val);
        self.pc += 1;
    }

    fn op_rcv(&mut self, reg: usize) {
        let val = &mut self.register[reg];
        if !self.module.op_rcv(val) {
            self.is_suspended = true;
        }
        self.pc += 1;
    }

    fn get_value(&self, op_value: &OpValue) -> Value {
        match op_value {
            OpValue::Reg(reg) => self.register[*reg],
            OpValue::Val(val) => *val,
        }
    }

    fn pc_offset(&mut self, offset: Value) {
        if offset > 0 {
            self.pc += offset as usize;
        } else {
            let offset = (-offset) as usize;
            self.pc -= offset;
        }
    }

    fn module_output(&self) -> Option<Value> {
        self.module.output()
    }
}

pub struct AoC2017_18 {
    ops: Vec<Op>,
}

impl AoC2017_18 {
    pub fn new() -> io::Result<Self> {
        let ops = read_file_as_lines("input/aoc2017_18")?
            .iter()
            .map(|s| Op::from_str(s))
            .collect();
        Ok(Self { ops })
    }
}

impl Solution for AoC2017_18 {
    fn part_one(&self) -> String {
        let mut machine = Machine::new(&self.ops, Box::<SoundModule>::default());
        machine.run();
        machine.module_output().unwrap().to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2017/Day 18: Duet".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_18_input_load_test() -> io::Result<()> {
        let sol = AoC2017_18::new()?;
        assert!(!sol.ops.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_18_example1() {
        let ops = "
            set a 1
            add a 2
            mul a a
            mod a 5
            snd a
            set a 0
            rcv a
            jgz a -1
            set a 1
            jgz a -2
     "
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(Op::from_str)
        .collect::<Vec<Op>>();
        assert_eq!(ops.len(), 10);
        let sol = AoC2017_18 { ops };
        assert_eq!(sol.part_one(), "4")
    }

    #[test]
    fn aoc2017_18_correctness() -> io::Result<()> {
        let sol = AoC2017_18::new()?;
        assert_eq!(sol.part_one(), "9423");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
