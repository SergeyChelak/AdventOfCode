use crate::solution::Solution;
use crate::utils::*;

use std::io;

type VmInt = isize;

struct Vm<'l> {
    pc: VmInt,
    memory: &'l [Instruction],
    value: VmInt,
}

impl<'l> Vm<'l> {
    fn new(memory: &'l [Instruction]) -> Self {
        Self {
            memory,
            pc: 0,
            value: 0,
        }
    }

    fn run(&mut self) -> Exit {
        let len = self.memory.len();
        let mut executed = vec![false; len];
        loop {
            if self.pc < 0 {
                break Exit::Error;
            }
            let pc = self.pc as usize;
            let Some(instr) = self.memory.get(pc) else {
                break Exit::Normal;
            };
            if executed[pc] {
                break Exit::Break;
            }
            executed[pc] = true;
            match instr {
                Instruction::Acc(val) => self.value += *val,
                Instruction::Nop(_) => {
                    // no op
                }
                Instruction::Jmp(val) => {
                    self.pc += *val;
                    continue;
                }
            }
            self.pc += 1;
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Exit {
    Break,
    Normal,
    Error,
}

#[derive(Debug, Clone)]
enum Instruction {
    Nop(VmInt),
    Acc(VmInt),
    Jmp(VmInt),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let (name, value) = value.split_once(" ").expect("Invalid instruction format");
        let value = value.parse::<VmInt>().expect("Value must be integer");

        match name {
            "nop" => Self::Nop(value),
            "acc" => Self::Acc(value),
            "jmp" => Self::Jmp(value),
            _ => unreachable!("invalid instruction"),
        }
    }
}

pub struct AoC2020_08 {
    input: Vec<Instruction>,
}

impl AoC2020_08 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2020_08")?;
        Ok(Self::parse(&lines))
    }

    fn parse<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(Instruction::from)
            .collect();
        Self { input }
    }
}

impl Solution for AoC2020_08 {
    fn part_one(&self) -> String {
        let mut vm = Vm::new(&self.input);
        let exit = vm.run();
        assert!(matches!(exit, Exit::Break));
        vm.value.to_string()
    }

    fn part_two(&self) -> String {
        let mut memory = self.input.clone();
        for i in 0..memory.len() {
            let preserve = memory[i].clone();
            let replace = match preserve {
                Instruction::Acc(_) => continue,
                Instruction::Jmp(val) => Instruction::Nop(val),
                Instruction::Nop(val) => Instruction::Jmp(val),
            };
            memory[i] = replace;
            let mut vm = Vm::new(&memory);
            let exit = vm.run();
            if matches!(exit, Exit::Normal) {
                return vm.value.to_string();
            }
            memory[i] = preserve;
        }
        not_found()
    }

    fn description(&self) -> String {
        "Day 8: Handheld Halting".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_08_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2020_08_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "1930");
        Ok(())
    }

    #[test]
    fn aoc2020_08_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "1688");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_08> {
        AoC2020_08::new()
    }
}
