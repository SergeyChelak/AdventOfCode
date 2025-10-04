use crate::solution::Solution;

use std::{collections::VecDeque, io};

type Int = usize;

pub struct AoC2020_22 {
    player1: Vec<Int>,
    player2: Vec<Int>,
}

impl AoC2020_22 {
    pub fn new() -> io::Result<Self> {
        let input = std::fs::read_to_string("input/aoc2020_22")?;
        Ok(Self::parse(&input))
    }

    fn parse(input: &str) -> Self {
        let (inp1, inp2) = input.split_once("\n\n").expect("Invalid input format");

        let parse = |s: &str| -> Vec<Int> {
            s.split('\n')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .skip(1)
                .map(|s| s.parse::<Int>().expect("Invalid player card value"))
                .collect()
        };

        Self {
            player1: parse(inp1),
            player2: parse(inp2),
        }
    }
}

impl Solution for AoC2020_22 {
    fn part_one(&self) -> String {
        let mut queue1 = VecDeque::from(self.player1.clone());
        let mut queue2 = VecDeque::from(self.player2.clone());

        while !queue1.is_empty() && !queue2.is_empty() {
            let card1 = queue1
                .pop_front()
                .expect("Unreachable: player1 queue is empty");
            let card2 = queue2
                .pop_front()
                .expect("Unreachable: player2 queue is empty");
            assert_ne!(card1, card2);

            if card1 > card2 {
                queue1.push_back(card1);
                queue1.push_back(card2);
            } else {
                queue2.push_back(card2);
                queue2.push_back(card1);
            }
        }

        let scores = |a: &VecDeque<Int>| -> Int {
            let size = a.len();
            a.iter()
                .enumerate()
                // .inspect(|(idx, x)| println!("{} * {}", x, size - idx))
                .map(|(idx, x)| x * (size - idx))
                .sum()
        };
        (scores(&queue1) + scores(&queue2)).to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 22: Crab Combat".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_22_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.player1.is_empty());
        assert!(!sol.player2.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2020_22_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "35397");
        Ok(())
    }

    #[test]
    fn aoc2020_22_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_22> {
        AoC2020_22::new()
    }

    #[test]
    fn aoc2020_22_case1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "306");
    }

    fn make_test_solution() -> AoC2020_22 {
        let input = "Player 1:
        9
        2
        6
        3
        1

        Player 2:
        5
        8
        4
        7
        10";
        AoC2020_22::parse(input)
    }
}
