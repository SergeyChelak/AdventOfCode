use crate::solution::Solution;
use crate::utils::*;

use std::io;

use super::machine::{Instruction, Machine, Operation};

#[derive(Clone)]
struct InputData {
    program: Vec<Instruction>,
    bind_reg: usize,
}

impl TryFrom<Vec<String>> for InputData {
    type Error = SolutionError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut ip_bind_reg: Option<usize> = None;
        let mut program = Vec::<Instruction>::new();
        for s in value {
            if s.starts_with("#ip") {
                ip_bind_reg = parse_ip_bound(&s);
            } else {
                let instr = instruction_from(s.as_str())?;
                program.push(instr);
            }
        }
        let Some(ip_bind_reg) = ip_bind_reg else {
            return Err(SolutionError::IpNotBound);
        };
        Ok(InputData {
            program,
            bind_reg: ip_bind_reg,
        })
    }
}

fn parse_ip_bound(val: &str) -> Option<usize> {
    let (_, reg_idx) = val.split_once(' ')?;
    reg_idx.parse::<usize>().ok()
}

pub struct AoC2018_19 {
    input: InputData,
}

impl AoC2018_19 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2018_19")?;
        let input = InputData::try_from(lines)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("{:?}", err)))?;
        Ok(Self { input })
    }
}

impl Solution for AoC2018_19 {
    fn part_one(&self) -> String {
        let mut machine = Machine::default();
        let mut ip = 0usize;
        let bind_reg = self.input.bind_reg;
        loop {
            let Some(&instruction) = self.input.program.get(ip) else {
                break;
            };
            machine.exec(instruction);
            if machine.last_modified_register() == Some(bind_reg) {
                ip = machine.reg(bind_reg) as usize;
            }
            ip += 1;
            if ip >= self.input.program.len() {
                break;
            }
            machine.set_reg(bind_reg, ip as i32);
        }
        machine.reg(0).to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 19: Go With The Flow".to_string()
    }
}

#[derive(Debug)]
enum SolutionError {
    UnexpectedInstructionFormat,
    UnknownInstruction,
    NonIntegerArgumentValue,
    IpNotBound,
}

fn instruction_from(value: &str) -> Result<Instruction, SolutionError> {
    let tokens = value.split(' ').collect::<Vec<&str>>();
    if tokens.len() != 4 {
        return Err(SolutionError::UnexpectedInstructionFormat);
    }
    let mut result = [0; 4];
    for (i, val) in tokens[1..].iter().enumerate() {
        result[i + 1] = val
            .parse::<i32>()
            .map_err(|_| SolutionError::NonIntegerArgumentValue)?
    }
    let id = Operation::try_from(tokens[0]).map_err(|_| SolutionError::UnknownInstruction)? as i32;
    result[0] = id;
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_19_input_load_test() -> io::Result<()> {
        let sol = AoC2018_19::new()?;
        assert!(!sol.input.program.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_19_case_1() {
        let program = [
            "#ip 0",
            "seti 5 0 1",
            "seti 6 0 2",
            "addi 0 1 0",
            "addr 1 2 3",
            "setr 1 0 0",
            "seti 8 0 4",
            "seti 9 0 5",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
        let input = InputData::try_from(program).ok().unwrap();
        let sol = AoC2018_19 { input };
        assert_eq!("6", sol.part_one())
    }

    #[test]
    fn aoc2018_19_correctness() -> io::Result<()> {
        let sol = AoC2018_19::new()?;
        assert_eq!(sol.part_one(), "2640");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
