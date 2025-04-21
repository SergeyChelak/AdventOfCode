use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone, Copy)]
enum ParseError {
    ElementFormat,
    InvalidAmount,
    EquationFormat,
}

#[derive(Debug, Clone)]
struct Element {
    name: String,
    amount: usize,
}

impl Element {
    fn new(name: &str, amount: usize) -> Self {
        Self {
            name: name.to_string(),
            amount,
        }
    }

    fn one_fuel() -> Self {
        Self::new("FUEL", 1)
    }
}

impl TryFrom<&str> for Element {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let Some((amount, name)) = value.split_once(' ') else {
            return Err(ParseError::ElementFormat);
        };

        let amount = amount
            .parse::<usize>()
            .map_err(|_| ParseError::InvalidAmount)?;

        Ok(Self {
            name: name.to_string(),
            amount,
        })
    }
}

struct Equation {
    inputs: Vec<Element>,
    output: Element,
}

impl TryFrom<&str> for Equation {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let Some((inputs, output)) = value.split_once(" => ") else {
            return Err(ParseError::EquationFormat);
        };

        let output = Element::try_from(output)?;

        let (inputs, failures): (Vec<_>, Vec<_>) = inputs
            .split(',')
            .map(|x| x.trim())
            .map(Element::try_from)
            .partition(Result::is_ok);

        if let Some(err) = failures.first().and_then(|x| x.as_ref().err()) {
            return Err(*err);
        }

        let inputs = inputs.into_iter().map(|x| x.unwrap()).collect::<Vec<_>>();
        Ok(Self { inputs, output })
    }
}

pub struct AoC2019_14 {
    equations: Vec<Equation>,
}

impl AoC2019_14 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2019_14")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let equations = lines
            .iter()
            .map(|x| x.as_ref())
            .map(|x| Equation::try_from(x).unwrap())
            .collect::<Vec<_>>();
        Self { equations }
    }
}

impl Solution for AoC2019_14 {
    fn part_one(&self) -> String {
        let element = Element::one_fuel();
        ore_amount(&self.equations, &mut HashMap::new(), &element).to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 14: Space Stoichiometry".to_string()
    }
}

fn ore_amount(
    equations: &[Equation],
    leftovers: &mut HashMap<String, usize>,
    from: &Element,
) -> usize {
    if from.name == "ORE" {
        return from.amount;
    }

    let mut requested = from.amount;
    if let Some(&val) = leftovers.get(&from.name) {
        let leftover = val.saturating_sub(requested);
        leftovers.remove(&from.name);
        if leftover > 0 {
            leftovers.insert(from.name.clone(), leftover);
        }
        requested = requested.saturating_sub(val);
    }

    if requested == 0 {
        return 0;
    }

    let Some(element) = equations.iter().find(|x| x.output.name == from.name) else {
        unreachable!("WTF?");
    };

    let mut amount = 0;
    for component in element.inputs.iter() {
        amount += ore_amount(equations, leftovers, component);
    }

    let received = element.output.amount;
    if requested > received {
        let rest = Element::new(&from.name, requested - received);
        amount += ore_amount(equations, leftovers, &rest);
    } else {
        leftovers.remove(&from.name);
        if received > requested {
            leftovers.insert(from.name.clone(), received - requested);
        }
    }
    amount
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_14_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.equations.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_14_parse_test() {
        let input = "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL";
        let equation = Equation::try_from(input).unwrap();
        assert_eq!(equation.inputs.len(), 7);
        assert_eq!(equation.inputs[0].name, "ZLQW");
        assert_eq!(equation.inputs[0].amount, 7);

        assert_eq!(equation.inputs[6].name, "RJRHP");
        assert_eq!(equation.inputs[6].amount, 1);

        assert_eq!(equation.output.name, "PLWSL");
        assert_eq!(equation.output.amount, 4);
    }

    #[test]
    fn aoc2019_14_case_1() {
        let input = [
            "10 ORE => 10 A",
            "1 ORE => 1 B",
            "7 A, 1 B => 1 C",
            "7 A, 1 C => 1 D",
            "7 A, 1 D => 1 E",
            "7 A, 1 E => 1 FUEL",
        ];
        let puzzle = AoC2019_14::with_lines(&input);
        assert_eq!(puzzle.part_one(), "31")
    }

    #[test]
    fn aoc2019_14_case_2() {
        let input = [
            "9 ORE => 2 A",
            "8 ORE => 3 B",
            "7 ORE => 5 C",
            "3 A, 4 B => 1 AB",
            "5 B, 7 C => 1 BC",
            "4 C, 1 A => 1 CA",
            "2 AB, 3 BC, 4 CA => 1 FUEL",
        ];
        let puzzle = AoC2019_14::with_lines(&input);

        /*
        Consume 15 B, 21 C to produce 3 BC.
        Consume 16 C, 4 A to produce 4 CA.
        Consume 2 AB, 3 BC, 4 CA to produce 1 FUEL.
        */
        let equations = &puzzle.equations;
        {
            // Consume 45 ORE to produce 10 A.
            let val = ore_amount(&equations, &mut HashMap::new(), &Element::new("A", 10));
            assert_eq!(val, 45);
        }

        {
            // Consume 64 ORE to produce 24 B.
            let val = ore_amount(&equations, &mut HashMap::new(), &Element::new("B", 24));
            assert_eq!(val, 64);
        }

        {
            // Consume 56 ORE to produce 40 C.
            let val = ore_amount(&equations, &mut HashMap::new(), &Element::new("C", 40));
            assert_eq!(val, 56);
        }
        {
            // Consume 6 A, 8 B to produce 2 AB.
            let mut leftovers = HashMap::new();
            let a6 = ore_amount(&equations, &mut leftovers, &Element::new("A", 6));
            let b8 = ore_amount(&equations, &mut leftovers, &Element::new("B", 8));

            let ab2 = ore_amount(&equations, &mut HashMap::new(), &Element::new("AB", 2));
            assert_eq!(a6 + b8, ab2)
        }

        assert_eq!(puzzle.part_one(), "165")
    }

    #[test]
    fn aoc2019_14_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "273638");
        Ok(())
    }

    #[test]
    fn aoc2019_14_case_3() {
        let input = [
            "157 ORE => 5 NZVS",
            "165 ORE => 6 DCFZ",
            "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL",
            "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ",
            "179 ORE => 7 PSHF",
            "177 ORE => 5 HKGWZ",
            "7 DCFZ, 7 PSHF => 2 XJWVT",
            "165 ORE => 2 GPVTF",
            "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        ];
        let puzzle = AoC2019_14::with_lines(&input);
        assert_eq!(puzzle.part_one(), "13312")
    }

    #[test]
    fn aoc2019_14_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_14> {
        AoC2019_14::new()
    }
}
