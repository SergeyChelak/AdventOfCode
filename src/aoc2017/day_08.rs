use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

type Value = i32;

struct Expression {
    target_reg: String,
    op_sign: Value,
    op_value: Value,
    cond_reg: String,
    cond_type: String,
    cond_value: Value,
}

impl Expression {
    fn parse(s: &str) -> Self {
        let tokens = s.split(' ').collect::<Vec<&str>>();
        let target_reg = tokens[0].to_string();
        let op_sign = if tokens[1] == "dec" { -1 } else { 1 };
        let op_value = tokens[2]
            .parse::<Value>()
            .expect("Integer values expected as operation argument");
        let cond_reg = tokens[4].to_string();
        let cond_type = tokens[5].to_string();
        let cond_value = tokens[6]
            .parse::<Value>()
            .expect("Integer values expected as argument");
        Self {
            target_reg,
            op_sign,
            op_value,
            cond_reg,
            cond_type,
            cond_value,
        }
    }
}

struct Machine {
    register: HashMap<String, Value>,
    reg_max: Value,
}

impl Machine {
    fn new() -> Self {
        Self {
            register: HashMap::new(),
            reg_max: Value::MIN,
        }
    }

    fn run(&mut self, program: &[Expression]) {
        program.iter().for_each(|expr| self.execute(expr))
    }

    fn execute(&mut self, expr: &Expression) {
        let mut reg_val = *self.register.entry(expr.target_reg.clone()).or_insert(0);

        let cond_reg_val = *self.register.entry(expr.cond_reg.clone()).or_insert(0);
        let val = expr.cond_value;
        let cond_val = match expr.cond_type.as_str() {
            ">" => cond_reg_val > val,
            ">=" => cond_reg_val >= val,
            "<" => cond_reg_val < val,
            "<=" => cond_reg_val <= val,
            "==" => cond_reg_val == val,
            "!=" => cond_reg_val != val,
            _ => panic!("Unexpected condition {}", expr.cond_type)
        };
        if cond_val {
            reg_val += expr.op_sign * expr.op_value;
            self.register.insert(expr.target_reg.clone(), reg_val);
        }
        self.reg_max = self.reg_max.max(reg_val);
    }

    fn largest_reg_value(&self) -> Value {
        self
            .register
            .values()
            .copied()
            .max()
            .expect("Registers set shouldn't be empty")
    }
}

pub struct AoC2017_08 {
    expressions: Vec<Expression>,
}

impl AoC2017_08 {
    pub fn new() -> io::Result<Self> {
        let expressions = read_file_as_lines("input/aoc2017_08")?
            .iter()
            .map(|s| Expression::parse(s))
            .collect::<Vec<Expression>>();
        Ok(Self { expressions })
    }
}

impl Solution for AoC2017_08 {
    fn part_one(&self) -> String {
        let mut machine = Machine::new();
        machine.run(&self.expressions);
        machine.largest_reg_value().to_string()
    }

    fn part_two(&self) -> String {
        let mut machine = Machine::new();
        machine.run(&self.expressions);
        machine.reg_max.to_string()
    }

    fn description(&self) -> String {
        "AoC 2017/Day 8: I Heard You Like Registers".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_08_input_load_test() -> io::Result<()> {
        let sol = AoC2017_08::new()?;
        assert!(!sol.expressions.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_08_correctness() -> io::Result<()> {
        let sol = AoC2017_08::new()?;
        assert_eq!(sol.part_one(), "5966");
        assert_eq!(sol.part_two(), "6347");
        Ok(())
    }
}
