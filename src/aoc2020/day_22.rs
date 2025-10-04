use crate::solution::Solution;

use std::{
    collections::{HashSet, VecDeque},
    hash::{DefaultHasher, Hash, Hasher},
    io,
};

type Int = usize;
type Queue = VecDeque<Int>;

enum Outcome {
    FirstWin,
    SecondWin,
}

pub struct AoC2020_22 {
    player1: Queue,
    player2: Queue,
}

impl AoC2020_22 {
    pub fn new() -> io::Result<Self> {
        let input = std::fs::read_to_string("input/aoc2020_22")?;
        Ok(Self::parse(&input))
    }

    fn parse(input: &str) -> Self {
        let (inp1, inp2) = input.split_once("\n\n").expect("Invalid input format");

        let parse = |s: &str| -> Queue {
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

    fn simulate(&self, combat: impl Fn(&mut Queue, &mut Queue) -> Outcome) -> String {
        let mut queue1 = self.player1.clone();
        let mut queue2 = self.player2.clone();
        let q = match combat(&mut queue1, &mut queue2) {
            Outcome::FirstWin => queue1,
            Outcome::SecondWin => queue2,
        };
        scores(&q).to_string()
    }
}

impl Solution for AoC2020_22 {
    fn part_one(&self) -> String {
        self.simulate(regular_combat)
    }

    fn part_two(&self) -> String {
        self.simulate(recursive_combat)
    }

    fn description(&self) -> String {
        "Day 22: Crab Combat".to_string()
    }
}

fn regular_combat(queue1: &mut Queue, queue2: &mut Queue) -> Outcome {
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

    if queue2.is_empty() {
        Outcome::FirstWin
    } else {
        Outcome::SecondWin
    }
}

fn recursive_combat(queue1: &mut Queue, queue2: &mut Queue) -> Outcome {
    let mut states = HashSet::new();
    states.insert(queue_hash(queue1));

    // fn play() {
    //     //
    // }

    // play();
    todo!()
}

fn queue_hash(q: &Queue) -> u64 {
    let mut h = DefaultHasher::new();
    q.iter().for_each(|val| val.hash(&mut h));
    h.finish()
}

fn scores(queue: &Queue) -> Int {
    let size = queue.len();
    queue
        .iter()
        .enumerate()
        // .inspect(|(idx, x)| println!("{} * {}", x, size - idx))
        .map(|(idx, x)| x * (size - idx))
        .sum()
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

    #[test]
    fn aoc2020_22_case2() {
        let sol = make_test_solution();
        assert_eq!(sol.part_two(), "291");
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
