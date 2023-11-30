use crate::solution::Solution;
use crate::utils::*;

use std::io;

type MachineInstruction = dyn Fn(&mut Machine);

struct Machine {
    reg: Registers,
    args: Instruction,
    mapping: Vec<&'static MachineInstruction>,
}

impl Machine {
    fn new() -> Self {
        let mapping: Vec<&'static MachineInstruction> = vec![
            &Self::addr,
            &Self::addi,
            &Self::mulr,
            &Self::muli,
            &Self::banr,
            &Self::bani,
            &Self::borr,
            &Self::bori,
            &Self::setr,
            &Self::seti,
            &Self::gtir,
            &Self::gtri,
            &Self::gtrr,
            &Self::eqir,
            &Self::eqri,
            &Self::eqrr,
        ];
        Self {
            reg: Registers::default(),
            args: Instruction::default(),
            mapping,
        }
    }

    fn addr(&mut self) {
        // addr (add register) stores into register C the result of adding register A and register B.
        self.reg[self.idx_c()] = self.reg[self.idx_a()] + self.reg[self.idx_b()];
    }

    fn addi(&mut self) {
        // addi (add immediate) stores into register C the result of adding register A and value B.
        self.reg[self.idx_c()] = self.reg[self.idx_a()] + self.val_b();
    }

    fn mulr(&mut self) {
        // mulr (multiply register) stores into register C the result of multiplying register A and register B.
        self.reg[self.idx_c()] = self.reg[self.idx_a()] * self.reg[self.idx_b()];
    }

    fn muli(&mut self) {
        // muli (multiply immediate) stores into register C the result of multiplying register A and value B.
        self.reg[self.idx_c()] = self.reg[self.idx_a()] * self.val_b();
    }

    fn banr(&mut self) {
        // banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
        self.reg[self.idx_c()] = self.reg[self.idx_a()] & self.reg[self.idx_b()];
    }

    fn bani(&mut self) {
        // bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
        self.reg[self.idx_c()] = self.reg[self.idx_a()] & self.val_b();
    }

    fn borr(&mut self) {
        // borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
        self.reg[self.idx_c()] = self.reg[self.idx_a()] | self.reg[self.idx_b()];
    }

    fn bori(&mut self) {
        // bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
        self.reg[self.idx_c()] = self.reg[self.idx_a()] | self.val_b();
    }

    fn setr(&mut self) {
        // setr (set register) copies the contents of register A into register C. (Input B is ignored.)
        self.reg[self.idx_c()] = self.reg[self.idx_a()]
    }

    fn seti(&mut self) {
        // seti (set immediate) stores value A into register C. (Input B is ignored.)
        self.reg[self.idx_c()] = self.val_a();
    }

    fn gtir(&mut self) {
        // gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
        self.reg[self.idx_c()] = (self.val_a() > self.reg[self.idx_b()]) as i32
    }

    fn gtri(&mut self) {
        // gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
        self.reg[self.idx_c()] = (self.reg[self.idx_a()] > self.val_b()) as i32
    }

    fn gtrr(&mut self) {
        // gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
        self.reg[self.idx_c()] = (self.reg[self.idx_a()] > self.reg[self.idx_b()]) as i32
    }

    fn eqir(&mut self) {
        // eqir (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
        self.reg[self.idx_c()] = (self.val_a() == self.reg[self.idx_b()]) as i32
    }

    fn eqri(&mut self) {
        // eqri (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
        self.reg[self.idx_c()] = (self.reg[self.idx_a()] == self.val_b()) as i32
    }

    fn eqrr(&mut self) {
        // eqrr (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
        self.reg[self.idx_c()] = (self.reg[self.idx_a()] == self.reg[self.idx_b()]) as i32
    }

    // Accessors
    fn idx_a(&self) -> usize {
        self.args[1] as usize
    }

    fn idx_b(&self) -> usize {
        self.args[2] as usize
    }

    fn idx_c(&self) -> usize {
        self.args[3] as usize
    }

    fn val_a(&self) -> i32 {
        self.args[1]
    }

    fn val_b(&self) -> i32 {
        self.args[2]
    }

    // -------
    fn ambiguous_count(&mut self, data: &TraceData) -> usize {
        let mut count = 0usize;
        for f in self.mapping.clone() {
            self.reg = data.before;
            self.args = data.instr;
            f(self);
            if self.reg == data.after {
                count += 1;
            }
        }
        count
    }
}

#[derive(Clone, Copy, Debug)]
enum ParserState {
    None,
    Separator(usize),
    Log(TraceData),
    Instruction,
}

type Registers = [i32; 4];
type Instruction = [i32; 4];

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

    #[allow(clippy::field_reassign_with_default)]
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
                x.parse::<i32>()
                    .expect("Instruction values should be integers")
            })
            .collect::<Vec<i32>>()
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
        let mut machine = Machine::new();
        self.input_1
            .iter()
            .map(|data| machine.ambiguous_count(data))
            .filter(|x| *x > 2)
            .count()
            .to_string()
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
        assert_eq!(sol.part_one(), "570");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
