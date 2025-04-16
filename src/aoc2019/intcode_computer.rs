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
            99 => Ok(OpCode::Hlt),
            _ => Err(format!("Unexpected opcode {value}")),
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
pub enum InterruptReason {
    Halted,
    WaitForInput,
    WrongInstruction { code: Int, pc: usize },
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    op_code: OpCode,
    mode_arg1: Mode,
    mode_arg2: Mode,
    // mode_arg3: Mode,
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
            // mode_arg3: modes[2],
        })
    }
}

pub struct IntcodeComputer {
    memory: Memory,
    input: Vec<Int>,
    output: Vec<Int>,
    pc: usize,
}

impl IntcodeComputer {
    pub fn with_memory(memory: Memory) -> Self {
        Self {
            memory,
            input: Vec::new(),
            output: Vec::new(),
            pc: 0,
        }
    }

    pub fn new(memory: Memory, input: Int) -> Self {
        let mut computer = Self::with_memory(memory);
        computer.push_input(input);
        computer
    }

    pub fn push_input(&mut self, value: Int) {
        self.input.push(value);
    }

    pub fn output(&mut self) -> Option<Int> {
        self.output.pop()
    }

    pub fn run(&mut self) -> InterruptReason {
        loop {
            let instruction_pc = self.pc;
            let code = self.consume();
            let Ok(instr) = Instruction::try_from(code) else {
                return InterruptReason::WrongInstruction {
                    code,
                    pc: instruction_pc,
                };
            };
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
                    if self.input.is_empty() {
                        // redo instruction
                        self.pc = instruction_pc;
                        return InterruptReason::WaitForInput;
                    };
                    let val = self.consume();
                    assert!(val >= 0);
                    self.memory[val as usize] = self.input.remove(0)
                }
                OpCode::Out => {
                    let val = self.consume();
                    assert!(val >= 0);
                    self.output.push(self.memory[val as usize]);
                }
                OpCode::Hlt => return InterruptReason::Halted,
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

pub fn parse_program(input: &str) -> Memory {
    input
        .trim()
        .split(',')
        .map(|x| x.parse::<Int>().expect("Invalid input"))
        .collect()
}
