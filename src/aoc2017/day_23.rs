use crate::solution::Solution;
use crate::utils::*;

use std::io;

use super::vm_utils::*;

// Most of this code is copy-paste from the day 18
// TODO: find the way to generalize Virtual Machine
struct Machine<'a> {
    register: [Value; 26],
    pc: usize,
    ops: &'a [Op],
    mul_cnt: usize,
}

impl<'a> Machine<'a> {
    fn new(ops: &'a [Op]) -> Self {
        Self {
            register: [0; 26],
            pc: 0,
            ops,
            mul_cnt: 0,
        }
    }

    fn run(&mut self) {
        while self.pc < self.ops.len() {
            match &self.ops[self.pc] {
                Op::Set(reg, op_value) => self.op_set(*reg, op_value),
                Op::Sub(reg, op_value) => self.op_sub(*reg, op_value),
                Op::Mul(reg, op_value) => self.op_mul(*reg, op_value),
                Op::Jnz(op_value, offset) => self.op_jnz(op_value, offset),
                _ => panic!("Unsupported operation"),
            }
        }
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

    fn op_set(&mut self, reg: usize, op_value: &OpValue) {
        let val = self.get_value(op_value);
        self.register[reg] = val;
        self.pc += 1;
    }

    fn op_sub(&mut self, reg: usize, op_value: &OpValue) {
        let val = self.get_value(op_value);
        self.register[reg] -= val;
        self.pc += 1;
    }

    fn op_mul(&mut self, reg: usize, op_value: &OpValue) {
        self.mul_cnt += 1;
        let val = self.get_value(op_value);
        self.register[reg] *= val;
        self.pc += 1;
    }

    fn op_jnz(&mut self, op_value: &OpValue, offset: &OpValue) {
        let x = self.get_value(op_value);
        if x != 0 {
            let offset = self.get_value(offset);
            self.pc_offset(offset);
        } else {
            self.pc += 1;
        }
    }
}

pub struct AoC2017_23 {
    ops: Vec<Op>,
}

impl AoC2017_23 {
    pub fn new() -> io::Result<Self> {
        let ops = read_file_as_lines("input/aoc2017_23")?
            .iter()
            .map(|s| Op::from_str(s))
            .collect();
        Ok(Self { ops })
    }
}

impl Solution for AoC2017_23 {
    fn part_one(&self) -> String {
        let mut vm = Machine::new(&self.ops);
        vm.run();
        vm.mul_cnt.to_string()
    }

    fn part_two(&self) -> String {
        // init
        let a: Value = 1;
        let mut h: Value = 0;
        let mut from: Value = 81;
        let mut to: Value = from;
        if a != 0 {
            from = from * 100 + 100000;
            to = from + 17000;
        }
        // calc
        for b in (from..=to).step_by(17) {
            'first: for d in 2..b {
                'second: for e in 2..b {
                    match b.cmp(&(d * e)) {
                        std::cmp::Ordering::Equal => {
                            h += 1;
                            break 'first;
                        }
                        std::cmp::Ordering::Less => {
                            break 'second;
                        }
                        _ => {}
                    }
                }
            }
        }
        h.to_string()
    }

    fn description(&self) -> String {
        "AoC 2017/Day 23: Coprocessor Conflagration".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_23_input_load_test() -> io::Result<()> {
        let sol = AoC2017_23::new()?;
        assert!(!sol.ops.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_23_correctness() -> io::Result<()> {
        let sol = AoC2017_23::new()?;
        assert_eq!(sol.part_one(), "6241");
        assert_eq!(sol.part_two(), "909");
        Ok(())
    }
}
