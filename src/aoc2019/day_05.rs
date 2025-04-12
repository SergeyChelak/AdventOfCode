use crate::solution::Solution;

use std::fs::read_to_string;
use std::io;

type Int = i64;
type Memory = Vec<Int>;

#[derive(Debug, Clone, Copy)]
enum OpCode {
    Add,
    Mul,
    Inp,
    Out,
    Jit,
    Jif,
    Lt,
    Eq,
    Hlt,
}

impl From<Int> for OpCode {
    fn from(value: Int) -> Self {
        match value {
            1 => OpCode::Add,
            2 => OpCode::Mul,
            3 => OpCode::Inp,
            4 => OpCode::Out,
            5 => OpCode::Jit,
            6 => OpCode::Jif,
            7 => OpCode::Lt,
            8 => OpCode::Eq,
            99 => OpCode::Hlt,
            _ => panic!("Unexpected opcode {value}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Mode {
    Position,
    Immediate,
}

impl From<Int> for Mode {
    fn from(value: Int) -> Self {
        match value {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => panic!("Unexpected position"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    op_code: OpCode,
    mode_arg1: Mode,
    mode_arg2: Mode,
    // mode_arg3: Mode,
}

impl From<Int> for Instruction {
    fn from(value: Int) -> Self {
        assert!(value >= 0);
        let mut code = value;
        let op_code = OpCode::from(value % 100);
        code /= 100;
        let mut modes = Vec::with_capacity(3);
        for _ in 0..3 {
            let mode = Mode::from(code % 10);
            modes.push(mode);
            code /= 10;
        }
        Self {
            op_code,
            mode_arg1: modes[0],
            mode_arg2: modes[1],
            // mode_arg3: modes[2],
        }
    }
}

struct IntcodeComputer {
    memory: Memory,
    input: Int,
    output: Int,
    pc: usize,
}

impl IntcodeComputer {
    fn new(memory: Memory, input: Int) -> Self {
        Self {
            memory,
            input,
            output: 0,
            pc: 0,
        }
    }

    fn run(&mut self) {
        loop {
            let instr = Instruction::from(self.consume());
            match instr.op_code {
                OpCode::Add => {
                    let left = self.consume_read(instr.mode_arg1);
                    let right = self.consume_read(instr.mode_arg2);
                    self.consume_write(left + right);
                }
                OpCode::Mul => {
                    let left = self.consume_read(instr.mode_arg1);
                    let right = self.consume_read(instr.mode_arg2);
                    self.consume_write(left * right);
                }
                OpCode::Inp => {
                    let val = self.consume();
                    assert!(val >= 0);
                    self.memory[val as usize] = self.input;
                }
                OpCode::Out => {
                    let val = self.consume();
                    assert!(val >= 0);
                    self.output = self.memory[val as usize];
                }
                OpCode::Hlt => break,
                OpCode::Jit => {
                    let value = self.consume_read(instr.mode_arg1);
                    let addr = self.consume_read(instr.mode_arg2);
                    if value != 0 {
                        assert!(addr >= 0);
                        self.pc = addr as usize;
                    }
                }
                OpCode::Jif => {
                    let value = self.consume_read(instr.mode_arg1);
                    let addr = self.consume_read(instr.mode_arg2);
                    if value == 0 {
                        assert!(addr >= 0);
                        self.pc = addr as usize;
                    }
                }
                OpCode::Lt => {
                    let first = self.consume_read(instr.mode_arg1);
                    let second = self.consume_read(instr.mode_arg2);
                    self.consume_write(if first < second { 1 } else { 0 });
                }
                OpCode::Eq => {
                    let first = self.consume_read(instr.mode_arg1);
                    let second = self.consume_read(instr.mode_arg2);
                    self.consume_write(if first == second { 1 } else { 0 });
                }
            }
        }
    }

    fn consume_read(&mut self, mode: Mode) -> Int {
        let val = self.consume();
        self.value(val, mode)
    }

    fn consume_write(&mut self, value: Int) {
        let addr = self.consume();
        assert!(addr >= 0);
        self.memory[addr as usize] = value;
    }

    fn consume(&mut self) -> Int {
        let value = self.memory[self.pc];
        self.pc += 1;
        value
    }

    fn value(&mut self, value: Int, mode: Mode) -> Int {
        match mode {
            Mode::Position => {
                assert!(value >= 0);
                let value = value as usize;
                self.memory[value]
            }
            Mode::Immediate => value,
        }
    }
}

pub struct AoC2019_05 {
    input: Vec<Int>,
}

impl AoC2019_05 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2019_05")?;
        Ok(Self::with_str(&input))
    }

    fn with_str(input: &str) -> Self {
        let input = input
            .trim()
            .split(',')
            .map(|x| x.parse::<Int>().expect("Invalid input"))
            .collect();
        Self { input }
    }
}

impl Solution for AoC2019_05 {
    fn part_one(&self) -> String {
        let mut computer = IntcodeComputer::new(self.input.clone(), 1);
        computer.run();
        computer.output.to_string()
    }

    fn part_two(&self) -> String {
        let mut computer = IntcodeComputer::new(self.input.clone(), 5);
        computer.run();
        computer.output.to_string()
    }

    fn description(&self) -> String {
        "Day 5: Sunny with a Chance of Asteroids".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_05_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_05_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "7692125");
        Ok(())
    }

    #[test]
    fn aoc2019_05_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "14340395");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_05> {
        AoC2019_05::new()
    }
}
