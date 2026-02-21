use crate::solution::Solution;

use std::{io, iter::Peekable};

pub struct AoC2022_13 {
    input: Vec<String>,
}

impl AoC2022_13 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2022_13")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        let input = data
            .split("\n")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2022_13 {
    fn part_one(&self) -> String {
        self.input
            .chunks(2)
            .enumerate()
            .filter(|(_, chunk)| Node::from(chunk[0].as_str()) <= Node::from(chunk[1].as_str()))
            .map(|(i, _)| i + 1)
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        let mut array = self
            .input
            .iter()
            .map(|x| Node::from(x.as_str()))
            .collect::<Vec<_>>();

        let div1 = Node::from("[[2]]");
        let div2 = Node::from("[[6]]");

        array.push(div1.clone());
        array.push(div2.clone());

        array.sort();

        array
            .iter()
            .enumerate()
            .filter(|(_, x)| **x == div1 || **x == div2)
            .map(|(i, _)| i + 1)
            .product::<usize>()
            .to_string()
    }

    fn description(&self) -> String {
        "Day 13: Distress Signal".to_string()
    }
}

type Int = u32;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Number(Int),
    List(Vec<Node>),
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use Node::*;
        match (self, other) {
            (Number(l), Number(r)) => l.cmp(r),
            (List(l), List(r)) => {
                for (left_item, right_item) in l.iter().zip(r.iter()) {
                    let res = left_item.cmp(right_item);
                    if res != std::cmp::Ordering::Equal {
                        return res;
                    }
                }
                l.len().cmp(&r.len())
            }
            (Number(l), List(_)) => List(vec![Number(*l)]).cmp(other),
            (List(_), Number(r)) => self.cmp(&List(vec![Number(*r)])),
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let mut iter = tokenize(value).into_iter().peekable();
        parse_node_iter(&mut iter)
    }
}

fn parse_node_iter<I>(iter: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = Token>,
{
    match iter.next().unwrap() {
        Token::Number(val) => Node::Number(val),
        Token::Open => {
            let mut array = Vec::new();
            while let Some(token) = iter.peek() {
                match token {
                    Token::Close => {
                        break;
                    }
                    _ => {
                        let node = parse_node_iter(iter);
                        array.push(node);
                    }
                }
            }
            iter.next();
            Node::List(array)
        }
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone)]
enum Token {
    Open,
    Close,
    Number(Int),
}

fn tokenize(s: &str) -> Vec<Token> {
    let mut iter = s.trim().chars().peekable();
    let mut tokens = Vec::new();
    let mut accumulator: Option<Int> = None;
    while let Some(ch) = iter.next() {
        match ch {
            '[' => tokens.push(Token::Open),
            ']' => tokens.push(Token::Close),
            _ if ch.is_ascii_digit() => {
                let digit = ch.to_digit(10).unwrap();
                accumulator = accumulator.map(|x| x * 10 + digit).or(Some(digit));
                let has_more_digits = iter.peek().map(|x| x.is_ascii_digit()).unwrap_or(false);
                if !has_more_digits {
                    tokens.push(Token::Number(accumulator.unwrap()));
                    accumulator = None;
                }
            }
            _ => {
                // no op
            }
        }
    }
    tokens
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_13_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2022_13_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "4809");
        Ok(())
    }

    #[test]
    fn aoc2022_13_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "22600");
        Ok(())
    }

    #[test]
    fn aoc2022_13_case_1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "13")
    }

    #[test]
    fn aoc2022_13_case_2() {
        let sol = make_test_solution();
        assert_eq!(sol.part_two(), "140")
    }

    fn make_solution() -> io::Result<AoC2022_13> {
        AoC2022_13::new()
    }

    fn make_test_solution() -> AoC2022_13 {
        let input = "[1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
";
        AoC2022_13::parse_data(input)
    }
}
