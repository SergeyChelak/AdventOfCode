use crate::{solution::Solution, utils::not_found};

use std::io;

pub struct AoC2021_16 {
    input: Vec<char>,
}

impl AoC2021_16 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2021_16")?;
        Ok(Self::with_str(&data))
    }

    fn with_str(input: &str) -> Self {
        Self {
            input: str_to_chars(input),
        }
    }
}

impl Solution for AoC2021_16 {
    fn part_one(&self) -> String {
        let mut sum = 0;

        let mut stack = vec![Transmission::with_chars(&self.input)];
        while let Some(mut tx) = stack.pop() {
            while let Some(version) = tx.consume_version_checked() {
                sum += version;
                let id = tx.consume_type_id();
                match id {
                    4 => {
                        tx.consume_literal();
                    }
                    _ => {
                        if tx.consume_bool() {
                            _ = tx.consume_sub_packets_count();
                        } else {
                            let len = tx.consume_length();
                            let packet = tx.consume_packet(len);
                            let inner_tx = Transmission::with_bitmap(packet);
                            stack.push(inner_tx);
                        }
                    }
                }
            }
        }
        sum.to_string()
    }

    fn part_two(&self) -> String {
        let bitmap = chars_to_bitmap(&self.input);
        let result = interpret(bitmap);
        assert_eq!(1, result.len());
        result.first().map(|x| x.to_string()).unwrap_or(not_found())
    }

    fn description(&self) -> String {
        "Day 16: Packet Decoder".to_string()
    }
}

#[derive(Debug)]
enum Operator {
    Sum,
    Product,
    Min,
    Max,
    Greater,
    Less,
    Equal,
}

impl From<Int> for Operator {
    fn from(value: Int) -> Self {
        match value {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Min,
            3 => Self::Max,
            5 => Self::Greater,
            6 => Self::Less,
            7 => Self::Equal,
            _ => unreachable!(),
        }
    }
}

enum Token {
    Literal(Int),
    Operation(Operator, usize),
}

fn interpret(bitmap: Vec<Int>) -> Vec<Int> {
    let mut tx = Transmission::with_bitmap(bitmap);
    let mut tokens = Vec::<Token>::new();
    while tx.consume_version_checked().is_some() {
        let id = tx.consume_type_id();
        if id == 4 {
            let literal = tx.consume_literal();
            tokens.push(Token::Literal(literal));
            continue;
        }

        let operator = Operator::from(id);
        let is_amount = tx.consume_bool();
        if is_amount {
            let count = tx.consume_sub_packets_count();
            tokens.push(Token::Operation(operator, count));
            continue;
        }
        let len = tx.consume_length();
        let packet = tx.consume_packet(len);
        let inner = interpret(packet);
        tokens.push(Token::Operation(operator, inner.len()));
        inner
            .into_iter()
            .rev()
            .map(Token::Literal)
            .for_each(|t| tokens.push(t));
    }
    eval_prefix(tokens)
}

fn eval_prefix(mut tokens: Vec<Token>) -> Vec<Int> {
    let mut stack = Vec::<Int>::new();

    while let Some(token) = tokens.pop() {
        match token {
            Token::Literal(val) => stack.push(val),
            Token::Operation(operator, count) => {
                let params = stack.split_off(stack.len() - count);
                let value = eval_operator(&operator, &params);
                stack.push(value)
            }
        }
    }
    stack
}

fn eval_operator(operator: &Operator, params: &[Int]) -> Int {
    match operator {
        Operator::Sum => params.iter().sum::<Int>(),
        Operator::Product => params.iter().product::<Int>(),
        Operator::Min => *params.iter().min().expect("(min) params are empty"),
        Operator::Max => *params.iter().max().expect("(max) params are empty"),
        Operator::Greater => {
            assert_eq!(params.len(), 2);
            (params[1] > params[0]) as Int
        }
        Operator::Less => {
            assert_eq!(params.len(), 2);
            (params[1] < params[0]) as Int
        }
        Operator::Equal => {
            assert_eq!(params.len(), 2);
            (params[1] == params[0]) as Int
        }
    }
}

type Int = usize;

struct Transmission {
    bitmap: Vec<Int>,
    ptr: usize,
}

impl Transmission {
    fn with_chars(input: &[char]) -> Self {
        let bitmap = chars_to_bitmap(input);
        Self::with_bitmap(bitmap)
    }

    fn with_bitmap(bitmap: Vec<Int>) -> Self {
        Self { bitmap, ptr: 0 }
    }

    fn has_more(&self) -> bool {
        self.ptr < self.bitmap.len()
    }

    fn consume(&mut self, count: usize) -> &[Int] {
        let slice = &self.bitmap[self.ptr..self.ptr + count];
        self.ptr += count;
        slice
    }

    fn consume_digit(&mut self, count: usize) -> Int {
        self.consume(count).iter().fold(0, |acc, x| (acc << 1) | x)
    }

    fn consume_bool(&mut self) -> bool {
        let value = self.bitmap[self.ptr];
        self.ptr += 1;
        value == 1
    }

    fn consume_version_checked(&mut self) -> Option<Int> {
        if !self.has_more() {
            return None;
        }
        if self.bitmap.len() - self.ptr < 6 {
            return None;
        }
        Some(self.consume_version())
    }

    fn consume_version(&mut self) -> Int {
        self.consume_digit(3)
    }

    fn consume_type_id(&mut self) -> Int {
        self.consume_digit(3)
    }

    fn consume_length(&mut self) -> Int {
        self.consume_digit(15)
    }

    fn consume_literal(&mut self) -> Int {
        let mut acc = 0;
        let mut has_more = true;
        while has_more {
            let digit = self.consume_digit(5);
            let bit_5 = 1 << 4;
            has_more = digit & bit_5 != 0;
            acc = (acc << 4) | (digit & !bit_5);
        }
        acc
    }

    fn consume_packet(&mut self, count: usize) -> Vec<Int> {
        self.consume(count).to_vec()
    }

    fn consume_sub_packets_count(&mut self) -> Int {
        self.consume_digit(11)
    }
}

fn chars_to_bitmap(chars: &[char]) -> Vec<Int> {
    chars
        .iter()
        .flat_map(|ch| transform(*ch))
        .collect::<Vec<_>>()
}

fn transform(ch: char) -> [Int; 4] {
    match ch {
        '0' => [0, 0, 0, 0],
        '1' => [0, 0, 0, 1],
        '2' => [0, 0, 1, 0],
        '3' => [0, 0, 1, 1],
        '4' => [0, 1, 0, 0],
        '5' => [0, 1, 0, 1],
        '6' => [0, 1, 1, 0],
        '7' => [0, 1, 1, 1],
        '8' => [1, 0, 0, 0],
        '9' => [1, 0, 0, 1],
        'A' => [1, 0, 1, 0],
        'B' => [1, 0, 1, 1],
        'C' => [1, 1, 0, 0],
        'D' => [1, 1, 0, 1],
        'E' => [1, 1, 1, 0],
        'F' => [1, 1, 1, 1],
        _ => unreachable!(),
    }
}

fn str_to_chars(s: &str) -> Vec<char> {
    s.trim().chars().collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_16_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2021_16_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "986");
        Ok(())
    }

    #[test]
    fn aoc2021_16_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "18234816469452");
        Ok(())
    }

    #[test]
    fn aoc2021_16_transmission_consume_literal() {
        let mut tx = Transmission::with_str("D2FE28");
        let version = tx.consume_version();
        assert_eq!(version, 0b110);

        let type_id = tx.consume_version();
        assert_eq!(type_id, 0b100);

        let literal = tx.consume_literal();
        assert_eq!(literal, 0b011111100101);
    }

    #[test]
    fn aoc2021_16_transmission_consume_packets_count() {
        let mut tx = Transmission::with_str("EE00D40C823060");
        assert_eq!(7, tx.consume_version());
        assert_eq!(3, tx.consume_type_id());
        assert!(tx.consume_bool());

        assert_eq!(3, tx.consume_sub_packets_count());

        _ = tx.consume_version();
        assert_eq!(4, tx.consume_type_id());
        assert_eq!(1, tx.consume_literal());

        _ = tx.consume_version();
        assert_eq!(4, tx.consume_type_id());
        assert_eq!(2, tx.consume_literal());

        _ = tx.consume_version();
        assert_eq!(4, tx.consume_type_id());
        assert_eq!(3, tx.consume_literal());
    }

    #[test]
    fn aoc2021_16_transmission_consume_packets() {
        let mut root_tx = Transmission::with_str("38006F45291200");
        let version = root_tx.consume_version();
        assert_eq!(version, 0b001);

        let type_id = root_tx.consume_type_id();
        assert_eq!(type_id, 0b110);

        let len_id = root_tx.consume_bool();
        assert!(!len_id);

        let length = root_tx.consume_length();
        assert_eq!(length, 27);

        let package = root_tx.consume_packet(length);
        let mut tx_1 = Transmission::with_bitmap(package);
        _ = tx_1.consume_version();
        assert_eq!(tx_1.consume_type_id(), 4);
        assert_eq!(tx_1.consume_literal(), 10);
        _ = tx_1.consume_version();
        assert_eq!(tx_1.consume_type_id(), 4);
        assert_eq!(tx_1.consume_literal(), 20);
    }

    #[test]
    fn aoc2021_16_cases_2() {
        let cases = [
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            // cmp
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];
        for (inp, out) in cases {
            let sol = AoC2021_16::with_str(inp);
            assert_eq!(out.to_string(), sol.part_two());
        }
    }

    fn make_solution() -> io::Result<AoC2021_16> {
        AoC2021_16::new()
    }

    impl Transmission {
        fn with_str(s: &str) -> Self {
            let chars = str_to_chars(s);
            Self::with_chars(&chars)
        }
    }
}
