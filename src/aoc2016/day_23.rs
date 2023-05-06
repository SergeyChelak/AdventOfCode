use crate::solution::Solution;
use crate::utils::*;

use super::assembunny_vm::*;
use std::io;

pub struct AoC2016_23 {
    program: Vec<String>,
}

impl AoC2016_23 {
    pub fn new() -> io::Result<Self> {
        let program = read_file_as_lines("input/aoc2016_23")?;
        Ok(Self { program })
    }
}

impl Solution for AoC2016_23 {
    fn part_one(&self) -> String {
        let mut machine = Machine::with_lines(&self.program);
        machine.set_reg_a(7);
        machine.run();
        machine.reg_a().to_string()
    }

    fn part_two(&self) -> String {
        let mut machine = Machine::with_lines(&self.program);
        machine.set_reg_a(12);
        machine.run();
        machine.reg_a().to_string()
    }

    fn description(&self) -> String {
        "AoC 2016/Day 23: Safe Cracking".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_23_input_load_test() -> io::Result<()> {
        let sol = AoC2016_23::new()?;
        assert!(!sol.program.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2016_23_correctness() -> io::Result<()> {
        let sol = AoC2016_23::new()?;
        assert_eq!(sol.part_one(), "11500");
        assert_eq!(sol.part_two(), "479008060");
        Ok(())
    }
}
