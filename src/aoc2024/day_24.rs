use crate::solution::Solution;

use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    io,
};

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

    fn part_two(&self) -> String {
        // full-adder:
        // Sum = A xor B xor CarryIn
        // CarryOut = A and B or (A xor B) and CarryIN
        //
        // x## & y## are inputs
        //

        // z00 = x00 xor y00
        //           Carry00         half-adder
        // z01 = (x00 and y00) xor (x01 xor y01)
        //           Carry00                       Carry01                   half-adder
        // z02 = ( ((y00 and x00) and (y01 Xor x01)) or (y01 And x01) ) xor (y02 Xor x02)
        let mut gates = self.gates.clone();
        let get_wrong_index = |gates: &HashMap<String, Gate>| -> Option<usize> {
            for i in 0.. {
                let name = format!("z{i:02}");
                if !validate_output(gates, &name, i) {
                    return Some(i);
                }
            }
            None
        };
        let mut target = get_wrong_index(&gates);
        let keys = gates.keys().cloned().collect::<Vec<_>>();
        let mut result = Vec::new();
        for _ in 0..4 {
            'next: for (i, wire_i) in keys.iter().enumerate() {
                for wire_j in keys.iter().skip(i + 1) {
                    let val_i = gates.get(wire_i).unwrap().clone();
                    let val_j = gates.get(wire_j).unwrap().clone();
                    gates.insert(wire_i.clone(), val_j.clone());
                    gates.insert(wire_j.clone(), val_i.clone());
                    let progress = get_wrong_index(&gates);
                    if progress > target {
                        result.push(wire_i.clone());
                        result.push(wire_j.clone());
                        target = progress;
                        continue 'next;
                    }
                    // revert
                    gates.insert(wire_i.clone(), val_i);
                    gates.insert(wire_j.clone(), val_j);
                }
            }
        }
        result.sort();
        result.join(",")
    }

    fn description(&self) -> String {
        "2024/Day 24: Crossed Wires".to_string()
    }
}

// z-gates are outputs
fn validate_output(gates: &Gates, wire: &str, idx: usize) -> bool {
    let Some(gate) = gates.get(wire) else {
        return false;
    };
    if !matches!(gate.op, Operator::Xor) {
        return false;
    }
    if idx == 0 {
        return is_valid_inputs(&gate.l, &gate.r, 0);
    }
    validate_half_adder(gates, &gate.l, idx) && validate_carry(gates, &gate.r, idx)
        || validate_half_adder(gates, &gate.r, idx) && validate_carry(gates, &gate.l, idx)
}

fn validate_half_adder(gates: &Gates, wire: &str, idx: usize) -> bool {
    let Some(gate) = gates.get(wire) else {
        return false;
    };
    if !matches!(gate.op, Operator::Xor) {
        return false;
    }
    is_valid_inputs(&gate.l, &gate.r, idx)
}

fn validate_carry(gates: &Gates, wire: &str, idx: usize) -> bool {
    let Some(gate) = gates.get(wire) else {
        return false;
    };
    if idx == 1 {
        if !matches!(gate.op, Operator::And) {
            return false;
        }
        return is_valid_inputs(&gate.l, &gate.r, idx - 1);
    }
    if !matches!(gate.op, Operator::Or) {
        return false;
    }
    validate_direct_carry(gates, &gate.l, idx - 1)
        && validate_indirect_carry(gates, &gate.r, idx - 1)
        || validate_direct_carry(gates, &gate.r, idx - 1)
            && validate_indirect_carry(gates, &gate.l, idx - 1)
}

fn validate_direct_carry(gates: &Gates, wire: &str, idx: usize) -> bool {
    let Some(gate) = gates.get(wire) else {
        return false;
    };
    if !matches!(gate.op, Operator::And) {
        return false;
    }
    is_valid_inputs(&gate.l, &gate.r, idx)
}

fn validate_indirect_carry(gates: &Gates, wire: &str, idx: usize) -> bool {
    let Some(gate) = gates.get(wire) else {
        return false;
    };
    if !matches!(gate.op, Operator::And) {
        return false;
    }
    validate_half_adder(gates, &gate.l, idx) && validate_carry(gates, &gate.r, idx)
        || validate_half_adder(gates, &gate.r, idx) && validate_carry(gates, &gate.l, idx)
}

fn is_valid_inputs(wire1: &str, wire2: &str, idx: usize) -> bool {
    let set = HashSet::from([format!("x{idx:02}"), format!("y{idx:02}")]);
    set.contains(wire1) && set.contains(wire2)
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
        assert_eq!(sol.part_two(), "dnt,gdf,gwc,jst,mcm,z05,z15,z30");
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
