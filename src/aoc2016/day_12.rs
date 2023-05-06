use crate::solution::Solution;
use crate::utils::*;

use super::assembunny_vm::*;
use std::io;

pub struct AoC2016_12 {
    program: Vec<String>,
}

impl AoC2016_12 {
    pub fn new() -> io::Result<Self> {
        let program = read_file_as_lines("input/aoc2016_12")?;
        Ok(Self { program })
    }
}

impl Solution for AoC2016_12 {
    fn part_one(&self) -> String {
        let mut machine = Machine::with_lines(&self.program);
        machine.run();
        machine.reg_a().to_string()
    }

    fn part_two(&self) -> String {
        let mut machine = Machine::with_lines(&self.program);
        machine.set_reg_c(1);
        machine.run();
        machine.reg_a().to_string()
    }

    fn description(&self) -> String {
        "AoC 2016/Day 12: Leonardo's Monorail".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_12_input_load_test() -> io::Result<()> {
        let sol = AoC2016_12::new()?;
        assert!(!sol.program.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2016_12_correctness() -> io::Result<()> {
        let sol = AoC2016_12::new()?;
        assert_eq!(sol.part_one(), "318007");
        assert_eq!(sol.part_two(), "9227661");
        Ok(())
    }
}
