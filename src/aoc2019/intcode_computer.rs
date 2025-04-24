use std::mem::swap;

pub type Int = i64;
pub type Memory = Vec<Int>;

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
    Arb,
    Hlt,
}

impl TryFrom<Int> for OpCode {
    type Error = String;

    fn try_from(value: Int) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(OpCode::Add),
            2 => Ok(OpCode::Mul),
            3 => Ok(OpCode::Inp),
            4 => Ok(OpCode::Out),
            5 => Ok(OpCode::Jit),
            6 => Ok(OpCode::Jif),
            7 => Ok(OpCode::Lt),
            8 => Ok(OpCode::Eq),
            9 => Ok(OpCode::Arb),
            99 => Ok(OpCode::Hlt),
            _ => Err(format!("Unexpected opcode {value}")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl From<Int> for Mode {
    fn from(value: Int) -> Self {
        match value {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!("Unexpected position"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ExecutionStatus {
    Halted,
    WaitForInput,
    WrongInstruction { code: Int, pc: usize },
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    op_code: OpCode,
    mode_arg1: Mode,
    mode_arg2: Mode,
    mode_arg3: Mode,
}

impl TryFrom<Int> for Instruction {
    type Error = String;

    fn try_from(value: Int) -> Result<Self, Self::Error> {
        assert!(value >= 0);
        let mut code = value;
        let op_code = OpCode::try_from(value % 100)?;
        code /= 100;
        let mut modes = Vec::with_capacity(3);
        for _ in 0..3 {
            let mode = Mode::from(code % 10);
            modes.push(mode);
            code /= 10;
        }
        Ok(Self {
            op_code,
            mode_arg1: modes[0],
            mode_arg2: modes[1],
            mode_arg3: modes[2],
        })
    }
}

#[derive(Clone)]
pub struct IntcodeComputer {
    memory: Memory,
    input: Vec<Int>,
    output: Vec<Int>,
    pc: usize,
    relative_base: Int,
}

impl IntcodeComputer {
    pub fn with_size(memory_size: usize) -> Self {
        Self {
            memory: vec![0; memory_size],
            input: Vec::new(),
            output: Vec::new(),
            pc: 0,
            relative_base: 0,
        }
    }

    pub fn with_memory(memory: &[Int]) -> Self {
        let mut instance = Self::with_size(memory.len());
        instance.load_program(memory);
        instance
    }

    pub fn new(memory: &[Int], input: Int) -> Self {
        let mut computer = Self::with_memory(memory);
        computer.push_input(input);
        computer
    }

    pub fn load_program(&mut self, data: &[Int]) {
        self.reset();
        self.memory[..data.len()].clone_from_slice(data);
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.relative_base = 0;
        self.input.clear();
        self.output.clear();
    }

    pub fn push_input(&mut self, value: Int) {
        self.input.push(value);
    }

    pub fn push_input_str(&mut self, value: &str) {
        value
            .chars()
            .map(|ch| ch as u8 as Int)
            .for_each(|x| self.push_input(x));
    }

    pub fn pop_output(&mut self) -> Option<Int> {
        self.output.pop()
    }

    pub fn outputs(&self) -> &[Int] {
        &self.output
    }

    #[allow(dead_code)]
    pub fn sink_outputs(&mut self) -> Vec<Int> {
        let mut val = Vec::new();
        swap(&mut self.output, &mut val);
        val
    }

    pub fn run(&mut self) -> ExecutionStatus {
        loop {
            let result = self.execute_step();
            if let Err(status) = result {
                break status;
            }
        }
    }

    pub fn execute_step(&mut self) -> Result<(), ExecutionStatus> {
        let instruction_pc = self.pc;
        let code = self.consume();
        let Ok(instr) = Instruction::try_from(code) else {
            return Err(ExecutionStatus::WrongInstruction {
                code,
                pc: instruction_pc,
            });
        };
        match instr.op_code {
            OpCode::Add => {
                let left = self.consume_read(instr.mode_arg1);
                let right = self.consume_read(instr.mode_arg2);
                self.consume_write(left + right, instr.mode_arg3);
            }
            OpCode::Mul => {
                let left = self.consume_read(instr.mode_arg1);
                let right = self.consume_read(instr.mode_arg2);
                self.consume_write(left * right, instr.mode_arg3);
            }
            OpCode::Inp => {
                if self.input.is_empty() {
                    // redo instruction
                    self.pc = instruction_pc;
                    return Err(ExecutionStatus::WaitForInput);
                };
                let mut val = self.consume();
                if matches!(instr.mode_arg1, Mode::Relative) {
                    val += self.relative_base;
                }
                assert!(val >= 0);
                self.memory[val as usize] = self.input.remove(0)
            }
            OpCode::Out => {
                let val = self.consume_read(instr.mode_arg1);
                self.output.push(val);
            }
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
                self.consume_write_bool(first < second, instr.mode_arg3);
            }
            OpCode::Eq => {
                let first = self.consume_read(instr.mode_arg1);
                let second = self.consume_read(instr.mode_arg2);
                self.consume_write_bool(first == second, instr.mode_arg3);
            }
            OpCode::Arb => {
                // adjust relative base
                let parameter = self.consume_read(instr.mode_arg1);
                self.relative_base += parameter;
            }
            OpCode::Hlt => return Err(ExecutionStatus::Halted),
        }
        Ok(())
    }

    fn consume_read(&mut self, mode: Mode) -> Int {
        let val = self.consume();
        self.value(val, mode)
    }

    fn consume_address(&mut self, mode: Mode) -> usize {
        let mut val = self.consume();
        assert!(
            !matches!(mode, Mode::Immediate),
            "Immediate mode not expected in address, value = {val}"
        );
        if matches!(mode, Mode::Relative) {
            val += self.relative_base;
        }
        assert!(val >= 0, "Invalid address");
        val as usize
    }

    fn consume_write(&mut self, value: Int, mode: Mode) {
        let addr = self.consume_address(mode);
        self.memory[addr] = value;
    }

    fn consume_write_bool(&mut self, value: bool, mode: Mode) {
        self.consume_write(if value { 1 } else { 0 }, mode);
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
            Mode::Relative => {
                let address = self.relative_base + value;
                assert!(address >= 0, "Bad relative address");
                self.memory[address as usize]
            }
            Mode::Immediate => value,
        }
    }
}

pub fn parse_program(input: &str) -> Memory {
    input
        .trim()
        .split(',')
        .map(|x| x.parse::<Int>().expect("Invalid input"))
        .collect()
}
