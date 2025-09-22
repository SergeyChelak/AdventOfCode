use crate::solution::Solution;
use crate::utils::*;

use std::io;

type VmInt = isize;

struct Vm {
    pc: VmInt,
    memory: Vec<Instruction>,
    value: VmInt,
}

impl Vm {
    fn new(memory: Vec<Instruction>) -> Self {
        Self {
            memory,
            pc: 0,
            value: 0,
        }
    }

    fn run(&mut self) {
        let len = self.memory.len();
        let mut executed = vec![false; len];
        loop {
            if self.pc < 0 {
                panic!("Invalid pc: {}", self.pc);
            }
            let pc = self.pc as usize;
            if executed[pc] {
                break;
            }
            executed[pc] = true;
            let Some(instr) = self.memory.get(pc) else {
                break;
            };
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
        let mut vm = Vm::new(self.input.clone());
        vm.run();
        vm.value.to_string()
    }

    // fn part_two(&self) -> String {
    // }

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
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_08> {
        AoC2020_08::new()
    }
}
