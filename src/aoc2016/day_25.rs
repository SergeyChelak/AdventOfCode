use crate::solution::Solution;
use crate::utils::*;

use super::assembunny_vm::*;
use std::io;

pub struct AoC2016_25 {
    program: Vec<String>,
}

impl AoC2016_25 {
    pub fn new() -> io::Result<Self> {
        let program = read_file_as_lines("input/aoc2016_25")?;
        Ok(Self { program })
    }
}

impl Solution for AoC2016_25 {
    fn part_one(&self) -> String {
        let mut machine = Machine::with_lines(&self.program);
        let size = 10;
        let mut reg_a = 0;
        'outer: loop {
            machine.set_reg_a(reg_a);
            while machine.is_running() {
                machine.do_step();
                let buf = machine.get_output_buf();
                let len = buf.len();
                if len > 0 {
                    let last_idx = buf.len() - 1;
                    if last_idx % 2 != buf[last_idx] as usize {
                        break;
                    }
                    if last_idx + 1 == size {
                        break 'outer;
                    }
                }
            }
            reg_a += 1;
            machine.reset();
        }
        reg_a.to_string()
    }

    fn part_two(&self) -> String {
        "".to_string()
    }

    fn description(&self) -> String {
        "AoC 2016/Day 25: Clock Signal".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_25_input_load_test() -> io::Result<()> {
        let sol = AoC2016_25::new()?;
        assert!(!sol.program.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2016_25_correctness() -> io::Result<()> {
        let sol = AoC2016_25::new()?;
        assert_eq!(sol.part_one(), "180");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
