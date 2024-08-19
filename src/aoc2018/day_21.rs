use crate::aoc2018::machine::Machine;
use crate::solution::Solution;
use crate::utils::*;

use std::io;

use super::machine::MachineProgram;

pub struct AoC2018_21 {
    input: MachineProgram,
}

impl AoC2018_21 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2018_21")?;
        let input = MachineProgram::try_from(lines)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("{:?}", err)))?;
        Ok(Self { input })
    }
}

impl Solution for AoC2018_21 {
    fn part_one(&self) -> String {
        let mut value: Option<isize> = None;
        let mut machine = Machine::with_program(&self.input);
        // machine.set_debug(true);
        // machine.debug_disasm();
        while machine.exec_cycle() {
            if machine.ip() == 28 {
                value = Some(machine.reg(4));
                break;
            }
        }
        value.map(|x| x.to_string()).unwrap_or_default()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 21: Chronal Conversion".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_21_input_load_test() -> io::Result<()> {
        let sol = AoC2018_21::new()?;
        assert!(!sol.input.program.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_21_correctness() -> io::Result<()> {
        let sol = AoC2018_21::new()?;
        assert_eq!(sol.part_one(), "7129803");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
