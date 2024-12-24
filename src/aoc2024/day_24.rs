use crate::solution::Solution;

use std::{collections::HashMap, fs::read_to_string, io};

#[derive(Debug, Clone)]
struct Gate {
    l: String,
    r: String,
    op: Operator,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    And,
    Or,
    Xor,
}

type Value = bool;
type Gates = HashMap<String, Gate>;
type Values = HashMap<String, Value>;

pub struct AoC2024_24 {
    gates: Gates,
    values: Values,
}

impl AoC2024_24 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2024_24")?;
        Ok(Self::with_input(&input))
    }

    fn with_input(input: &str) -> Self {
        let (values, gates) = input.split_once("\n\n").expect("Invalid input format");
        let gates = parse(gates, parse_gate);
        let values = parse(values, parse_value);
        Self { gates, values }
    }
}

impl Solution for AoC2024_24 {
    fn part_one(&self) -> String {
        let output = calculate_output(&self.gates, &self.values);
        wires_value(&output, 'z').to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "2024/Day 24: Crossed Wires".to_string()
    }
}

fn wires_value(output: &Values, wire: char) -> usize {
    output
        .iter()
        .filter(|(key, _)| key.starts_with(wire))
        .fold(0usize, |acc, val| {
            let (name, value) = val;
            if !value {
                return acc;
            }
            let shift = name[1..].parse::<usize>().expect("Invalid wire name");
            acc | 1 << shift
        })
}

fn calculate_output(gates: &Gates, values: &Values) -> Values {
    fn update(gates: &Gates, key: &str, output: &mut Values) -> bool {
        if let Some(value) = output.get(key) {
            return *value;
        }
        let Some(gate) = gates.get(key) else {
            panic!("Missing element {key}");
        };
        let value = match gate.op {
            Operator::And => update(gates, &gate.l, output) && update(gates, &gate.r, output),
            Operator::Or => update(gates, &gate.l, output) || update(gates, &gate.r, output),
            Operator::Xor => update(gates, &gate.l, output) ^ update(gates, &gate.r, output),
        };
        output.insert(key.to_string(), value);
        value
    }

    let mut output = values.clone();
    for key in gates.keys() {
        update(gates, key, &mut output);
    }
    output
}

fn parse_gate(s: &str) -> (String, Gate) {
    let (gate, name) = s.split_once(" -> ").expect("Invalid gate format");
    let tokens = gate.split_whitespace().collect::<Vec<_>>();
    assert_eq!(tokens.len(), 3, "invalid gate parameters");
    let l = tokens[0].to_string();
    let r = tokens[2].to_string();
    let gate = match tokens[1] {
        "XOR" => Gate {
            l,
            r,
            op: Operator::Xor,
        },
        "OR" => Gate {
            l,
            r,
            op: Operator::Or,
        },
        "AND" => Gate {
            l,
            r,
            op: Operator::And,
        },
        x => panic!("Unknown gate {x}"),
    };
    (name.to_string(), gate)
}

fn parse_value(s: &str) -> (String, Value) {
    let (name, value) = s.split_once(": ").expect("Invalid value format");
    (name.to_string(), value == "1")
}

fn parse<T, P>(data: &str, parser: P) -> HashMap<String, T>
where
    P: Fn(&str) -> (String, T),
{
    let mut map = HashMap::new();
    data.split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .for_each(|s| {
            let (name, value) = parser(s);
            map.insert(name, value);
        });
    map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_24_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.gates.is_empty());
        assert!(!sol.values.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2024_24_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "47666458872582");
        Ok(())
    }

    #[test]
    fn aoc2024_24_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2024_24_small() {
        let input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
        let s = AoC2024_24::with_input(input);
        assert_eq!("4", s.part_one())
    }

    #[test]
    fn aoc2024_24_case_1() {
        let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
        let s = AoC2024_24::with_input(input);
        assert_eq!("2024", s.part_one())
    }

    fn make_solution() -> io::Result<AoC2024_24> {
        AoC2024_24::new()
    }
}
