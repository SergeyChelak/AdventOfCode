use crate::utils::*;
use crate::{aoc2018::machine::MachineInt, solution::Solution};

use std::io;

use super::machine::{Machine, MachineProgram};

pub struct AoC2018_19 {
    input: MachineProgram,
}

impl AoC2018_19 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2018_19")?;
        let input = MachineProgram::try_from(lines)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("{:?}", err)))?;
        Ok(Self { input })
    }

    fn exec(&self, machine: &mut Machine, breakpoint: Option<usize>) {
        loop {
            if Some(machine.ip()) == breakpoint {
                break;
            }
            if !machine.exec_cycle() {
                break;
            }
        }
    }
}

impl Solution for AoC2018_19 {
    fn part_one(&self) -> String {
        let mut machine = Machine::with_program(&self.input);
        self.exec(&mut machine, None);
        machine.reg(0).to_string()
    }

    fn part_two(&self) -> String {
        let mut machine = Machine::with_program(&self.input);
        machine.set_reg(0, 1);
        self.exec(&mut machine, Some(1));
        reversed_func_improved(machine.reg(2)).to_string()
    }

    fn description(&self) -> String {
        "AoC 2018/Day 19: Go With The Flow".to_string()
    }
}

fn reversed_func_improved(r2: MachineInt) -> MachineInt {
    let mut r0 = 0;
    for r3 in 1..=r2 {
        for r1 in 1..=r2 {
            let val = r3 * r1;
            if val > r2 {
                break;
            }
            if val == r2 {
                r0 += r3;
                break;
            }
        }
    }
    r0
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
        let input = MachineProgram::try_from(program).ok().unwrap();
        let sol = AoC2018_19 { input };
        assert_eq!("6", sol.part_one())
    }

    #[test]
    fn aoc2018_19_correctness() -> io::Result<()> {
        let sol = AoC2018_19::new()?;
        assert_eq!(sol.part_one(), "2640");
        assert_eq!(sol.part_two(), "27024480");
        Ok(())
    }
}
