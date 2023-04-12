use crate::solution::Solution;
use std::fs::read_to_string;
use std::io;

use serde_json::*;

type Json = Map<String, Value>;

fn sum_in_object(json: &Json, skip: &str) -> i64 {
    let skip_count = json.iter().map(|(_, v)| v).filter(|v| *v == skip).count();
    if skip_count == 0 {
        json.iter()
            .fold(0, |acc, (_, value)| acc + sum_in_value(value, skip))
    } else {
        0
    }
}

fn sum_in_value(value: &Value, skip: &str) -> i64 {
    match value {
        Value::Number(number) => number.as_i64().unwrap_or(0),
        Value::Object(map) => sum_in_object(map, skip),
        Value::Array(items) => items.iter().fold(0, |acc, v| acc + sum_in_value(v, skip)),
        _ => 0,
    }
}
pub struct AoC2015_12 {
    json: Json,
}

impl AoC2015_12 {
    pub fn new() -> io::Result<Self> {
        let json_str = read_to_string("input/aoc2015_12")?;
        let json: Json = serde_json::from_str(&json_str)?;
        Ok(Self { json })
    }
}

impl Solution for AoC2015_12 {
    fn part_one(&self) -> String {
        sum_in_object(&self.json, "").to_string()
    }

    fn part_two(&self) -> String {
        sum_in_object(&self.json, "red").to_string()
    }

    fn description(&self) -> String {
        "AoC 2015/Day 12: JSAbacusFramework.io".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_12_input_load_test() -> io::Result<()> {
        let sol = AoC2015_12::new()?;
        assert!(sol.json.len() > 0);
        Ok(())
    }

    #[test]
    fn aoc2015_12_correctness() -> io::Result<()> {
        let sol = AoC2015_12::new()?;
        assert_eq!(sol.part_one(), "111754");
        assert_eq!(sol.part_two(), "65402");
        Ok(())
    }
}
