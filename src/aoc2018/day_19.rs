use crate::solution::Solution;
use crate::utils::*;

use std::io;

struct InputData {
    program: Vec<Instruction>,
    ip_bind_reg: usize,
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
                let instr = Instruction::try_from(s.as_str())?;
                program.push(instr);
            }
        }
        let Some(ip_bind_reg) = ip_bind_reg else {
            return Err(SolutionError::IpNotBound);
        };
        Ok(InputData {
            program,
            ip_bind_reg,
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
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 19: Go With The Flow".to_string()
    }
}

#[derive(Debug)]
enum SolutionError {
    UnexpectedInstructionFormat,
    UnknownInstruction(String),
    NonIntegerArgumentValue(String),
    IpNotBound,
}

type Registers = [i32; 6];
type Arguments = [i32; 4];

enum InstructionId {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl TryFrom<&str> for InstructionId {
    type Error = SolutionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use InstructionId::*;
        match value {
            "addr" => Ok(Addr),
            "addi" => Ok(Addi),
            "mulr" => Ok(Mulr),
            "muli" => Ok(Muli),
            "banr" => Ok(Banr),
            "bani" => Ok(Bani),
            "borr" => Ok(Borr),
            "bori" => Ok(Bori),
            "setr" => Ok(Setr),
            "seti" => Ok(Seti),
            "gtir" => Ok(Gtir),
            "gtri" => Ok(Gtri),
            "gtrr" => Ok(Gtrr),
            "eqir" => Ok(Eqir),
            "eqri" => Ok(Eqri),
            "eqrr" => Ok(Eqrr),
            _ => Err(SolutionError::UnknownInstruction(value.to_owned())),
        }
    }
}

struct Instruction {
    id: InstructionId,
    args: Arguments,
}

impl TryFrom<&str> for Instruction {
    type Error = SolutionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let tokens = value.split(' ').collect::<Vec<&str>>();
        if tokens.len() != 4 {
            return Err(SolutionError::UnexpectedInstructionFormat);
        }
        let id = InstructionId::try_from(tokens[0])?;
        let mut args = [0; 4];
        for (i, val) in tokens[1..].iter().enumerate() {
            args[i] = val
                .parse::<i32>()
                .map_err(|x| SolutionError::NonIntegerArgumentValue(x.to_string()))?
        }
        Ok(Instruction { id, args })
    }
}

struct Machine {
    ip: i32,
    registers: Registers,
    program: Vec<Instruction>,
}

impl Machine {
    fn new(ip_bind_reg: usize, program: Vec<Instruction>) -> Self {
        Self {
            ip: 0,
            registers: Default::default(),
            program: Default::default(),
        }
    }

    fn run(&mut self) {
        loop {
            if self.ip < 0 && self.ip as usize >= self.program.len() {
                break;
            }
        }
    }
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
    fn aoc2018_19_correctness() -> io::Result<()> {
        let sol = AoC2018_19::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
