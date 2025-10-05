use crate::solution::Solution;

use std::{
    collections::{HashSet, LinkedList},
    io,
};

type Int = usize;
type Cups = Vec<Int>;

pub struct AoC2020_23 {
    input: Cups,
}

impl AoC2020_23 {
    pub fn new() -> io::Result<Self> {
        let input = std::fs::read_to_string("input/aoc2020_23")?;
        Ok(Self::parse(&input))
    }

    fn parse(input: &str) -> Self {
        let v = input
            .trim()
            .chars()
            .map(|x| {
                x.to_digit(10)
                    .expect("Input shouldn't contain non digit characters")
            })
            .map(|x| x as Int)
            .collect::<Vec<_>>();
        Self { input: v }
    }
}

impl Solution for AoC2020_23 {
    fn part_one(&self) -> String {
        let mut game = CrabGame::with(&self.input);
        game.perform(100);
        let digits = game.dbg_vec();
        let start = digits
            .iter()
            .enumerate()
            .find(|(_, x)| **x == 1)
            .expect("Unreachable: 1 not found")
            .0;
        let size = digits.len();
        (1..size)
            .map(|idx| digits[(idx + start) % size])
            .map(|x| x.to_string())
            .collect::<String>()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 23: Crab Cups".to_string()
    }
}

struct CrabGame {
    list: LinkedList<Int>,
}

impl CrabGame {
    fn with(input: &Cups) -> Self {
        let mut list = LinkedList::new();
        for val in input {
            list.push_back(*val);
        }
        Self { list }
    }

    fn expand(&mut self) {
        todo!()
    }

    fn bounds(&self) -> (Int, Int) {
        let min = *self.list.iter().min().expect("Min should be present");
        let max = *self.list.iter().max().expect("Max should be present");
        (min, max)
    }

    fn perform_step(&mut self, min: Int, max: Int) {
        let mut in_use = HashSet::new();

        let head = self
            .list
            .pop_front()
            .expect("Pop head: List can't be empty");
        self.list.push_back(head);
        in_use.insert(head);

        let mut picked = [0; 3];

        (0..3).for_each(|idx| {
            let val = self.list.pop_front().expect("Picked values");
            picked[idx] = val;
            in_use.insert(val);
        });

        let mut destination = head;
        while in_use.contains(&destination) {
            if destination == min {
                destination = max;
            } else {
                destination -= 1;
            }
        }

        let index = self
            .list
            .iter()
            .enumerate()
            .find(|(_, val)| **val == destination)
            .expect("Value must exist in list")
            .0;
        let index = (index + 1) % self.list.len();
        let mut tail = self.list.split_off(index);

        picked
            .into_iter()
            .rev()
            .for_each(|elem| tail.push_front(elem));

        self.list.append(&mut tail);
    }

    fn perform(&mut self, steps: usize) {
        let (min, max) = self.bounds();
        for _ in 0..steps {
            self.perform_step(min, max);
        }
    }

    fn dbg_vec(&self) -> Vec<Int> {
        self.list.iter().copied().collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_23_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(sol.input.iter().all(|x| *x != 0));
        Ok(())
    }

    #[test]
    fn aoc2020_23_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "38756249");
        Ok(())
    }

    #[test]
    fn aoc2020_23_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_23> {
        AoC2020_23::new()
    }

    #[test]
    fn aoc2020_23_simulate_step() {
        // in - out
        let data = [
            (
                vec![3, 8, 9, 1, 2, 5, 4, 6, 7],
                vec![2, 8, 9, 1, 5, 4, 6, 7, 3],
            ),
            (
                vec![2, 8, 9, 1, 5, 4, 6, 7, 3],
                vec![5, 4, 6, 7, 8, 9, 1, 3, 2],
            ),
            (
                vec![5, 4, 6, 7, 8, 9, 1, 3, 2],
                vec![8, 9, 1, 3, 4, 6, 7, 2, 5],
            ),
        ];
        for (input, output) in data {
            let mut game = CrabGame::with(&input);
            game.perform(1);
            assert_eq!(output, game.dbg_vec());
        }
    }
}
