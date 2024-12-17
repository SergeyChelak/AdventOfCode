use crate::solution::Solution;

use std::fs::read_to_string;
use std::{io, usize};

type Int = usize;

#[derive(Default, Clone)]
struct Machine {
    ra: Int,
    rb: Int,
    rc: Int,
    pc: usize,
    output: Vec<Int>,
    memory: Vec<Int>,
}

impl Machine {
    fn run(&mut self) {
        while self.pc <= self.memory.len() - 2 {
            let opcode = self.memory[self.pc];
            let operand = self.memory[self.pc + 1];
            match opcode {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => {
                    if self.jnz(operand) {
                        continue;
                    }
                }
                4 => self.bxc(),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => panic!("Unexpected opcode {opcode}"),
            };
            self.pc += 2;
        }
    }

    fn combo_value(&self, operand: Int) -> Int {
        match operand {
            0..=3 => operand,
            4 => self.ra,
            5 => self.rb,
            6 => self.rc,
            _ => panic!("unexpected combo value"),
        }
    }

    fn adv(&mut self, operand: Int) {
        // The adv instruction (opcode 0) performs division. The numerator is the value in the A register.
        // The denominator is found by raising 2 to the power of the instruction's combo operand.
        // (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
        // The result of the division operation is truncated to an integer and then written to the A register.
        self.ra >>= self.combo_value(operand);
    }

    fn bxl(&mut self, operand: Int) {
        // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
        self.rb ^= operand;
    }

    fn bst(&mut self, operand: Int) {
        // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
        self.rb = self.combo_value(operand) % 8;
    }

    fn jnz(&mut self, operand: Int) -> bool {
        // The jnz instruction (opcode 3) does nothing if the A register is 0.
        // However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand;
        // if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
        if self.ra == 0 {
            return false;
        }
        self.pc = operand;
        true
    }

    fn bxc(&mut self) {
        // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C,
        // then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
        self.rb ^= self.rc
    }

    fn out(&mut self, operand: Int) {
        // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value.
        // (If a program outputs multiple values, they are separated by commas.)
        self.output.push(self.combo_value(operand) % 8);
    }

    fn bdv(&mut self, operand: Int) {
        // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
        self.rb = self.ra >> self.combo_value(operand);
    }

    fn cdv(&mut self, operand: Int) {
        // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
        self.rc = self.ra >> self.combo_value(operand);
    }

    fn formatted_output(&self) -> String {
        self.output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let (registers, program) = value.split_once("\n\n").expect("Invalid input format");

        // parse registers
        let mut regs = [0; 4];
        let names = ["A", "B", "C"];
        for s in registers.split('\n') {
            let (prefix, value) = s.split_once(": ").expect("Invalid register format");
            let value = value.parse::<Int>().expect("Register value isn't numeric");
            for (i, name) in names.iter().enumerate() {
                if prefix.ends_with(name) {
                    regs[i] = value;
                    break;
                }
            }
        }

        // parse program
        let program = program
            .split_once(": ")
            .expect("Invalid program format")
            .1
            .split(',')
            .map(|s| s.trim())
            .map(|x| x.parse::<Int>().expect("Invalid program value"))
            .collect::<Vec<_>>();

        Self {
            ra: regs[0],
            rb: regs[1],
            rc: regs[2],
            pc: 0,
            output: Vec::new(),
            memory: program,
        }
    }
}

pub struct AoC2024_17 {
    input: String,
}

impl AoC2024_17 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2024_17")?;
        Ok(Self { input })
    }
}

impl Solution for AoC2024_17 {
    fn part_one(&self) -> String {
        let mut machine = Machine::from(self.input.as_str());
        machine.run();
        machine.formatted_output()
    }

    fn part_two(&self) -> String {
        let original = Machine::from(self.input.as_str());
        let target = original.memory;

        let mut backtrack = vec![0];
        let mut results = Vec::new();

        while let Some(mut acc) = backtrack.pop() {
            let mut has_next = true;
            let mut acc_next = acc;
            while has_next {
                has_next = false;
                for num in 0..8 {
                    acc_next = acc + num;
                    let arr = function(acc_next);
                    match validate(&arr, &target) {
                        MatchResult::Equal => results.push(acc_next),
                        MatchResult::Similar => backtrack.push(acc_next << 3),
                        _ => {}
                    };
                }
                acc = acc_next << 3;
            }
        }

        results.iter().min().expect("Result not found").to_string()
    }

    fn description(&self) -> String {
        "2024/Day 17: Chronospatial Computer".to_string()
    }
}

enum MatchResult {
    Similar,
    Equal,
    Wrong,
}

fn validate(arr: &[usize], target: &[usize]) -> MatchResult {
    if arr.len() > target.len() {
        return MatchResult::Wrong;
    }

    let is_equals = arr
        .iter()
        .rev()
        .zip(target.iter().rev())
        .all(|(a, b)| a == b);

    if !is_equals {
        return MatchResult::Wrong;
    }

    if arr.len() == target.len() {
        MatchResult::Equal
    } else {
        MatchResult::Similar
    }
}

fn function(x: usize) -> Vec<usize> {
    let mut ra = x;
    let mut rb;
    let mut rc;
    let mut out = Vec::new();
    loop {
        rb = ra % 8;
        rb ^= 1;
        rc = ra >> rb;
        rb ^= 5;
        rb ^= rc;
        out.push(rb % 8);
        ra >>= 3;
        if ra == 0 {
            break;
        }
    }
    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_17_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2024_17_parse_machine() {
        let input = "Register A: 729
Register B: 123
Register C: 345

Program: 0,1,5,4,3,0";
        let machine = Machine::from(input);
        assert_eq!(machine.ra, 729);
        assert_eq!(machine.rb, 123);
        assert_eq!(machine.rc, 345);
        assert_eq!(machine.memory, vec![0, 1, 5, 4, 3, 0])
    }

    #[test]
    fn aoc2024_17_program_1() {
        // If register C contains 9, the program 2,6 would set register B to 1.
        let mut machine = Machine {
            rc: 9,
            memory: vec![2, 6],
            ..Default::default()
        };
        machine.run();
        assert_eq!(machine.rb, 1)
    }

    #[test]
    fn aoc2024_17_program_2() {
        // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2
        let mut machine = Machine {
            ra: 10,
            memory: vec![5, 0, 5, 1, 5, 4],
            ..Default::default()
        };
        machine.run();
        assert_eq!(machine.formatted_output(), "0,1,2")
    }

    #[test]
    fn aoc2024_17_program_3() {
        // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A
        let mut machine = Machine {
            ra: 2024,
            memory: vec![0, 1, 5, 4, 3, 0],
            ..Default::default()
        };
        machine.run();
        assert_eq!(machine.formatted_output(), "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(machine.ra, 0);
    }

    #[test]
    fn aoc2024_17_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "4,1,7,6,4,1,0,2,7");
        Ok(())
    }

    #[test]
    fn aoc2024_17_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "164279024971453");
        Ok(())
    }

    #[test]
    fn aoc2024_17_case_2() {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        let puzzle = AoC2024_17 {
            input: input.to_string(),
        };
        assert_eq!(puzzle.part_two(), "117440");
    }

    fn make_solution() -> io::Result<AoC2024_17> {
        AoC2024_17::new()
    }
}
