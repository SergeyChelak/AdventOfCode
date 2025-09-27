use crate::solution::Solution;

use std::{collections::HashMap, io};

type Int = usize;

#[derive(Debug, Clone)]
struct Turn {
    current: usize,
    previous: Option<usize>,
}

impl Turn {
    fn initial(value: Int) -> Self {
        Self {
            current: value,
            previous: None,
        }
    }

    fn distance(&self) -> usize {
        let Some(prev) = self.previous else {
            return 0;
        };
        self.current - prev
    }
}

type StoreData = HashMap<Int, Turn>;

struct Store {
    data: StoreData,
    last: Int,
    turns: usize,
}

impl Store {
    fn new(initial: &[Int]) -> Self {
        assert!(!initial.is_empty());
        let mut last = 0;

        let mut data = StoreData::new();
        let mut turns = 0;
        for (i, val) in initial.iter().enumerate() {
            assert!(!data.contains_key(val));
            data.insert(*val, Turn::initial(i + 1));
            last = *val;
            turns += 1;
        }

        Self { data, last, turns }
    }

    fn next_turn(&mut self) {
        let turn = self
            .data
            .get(&self.last)
            .expect("Last element should be always present");
        let next = turn.distance();
        self.push(next);
    }

    fn push(&mut self, value: Int) {
        self.turns += 1;
        self.last = value;

        let Some(turn) = self.data.get(&value).cloned() else {
            self.data.insert(value, Turn::initial(self.turns));
            return;
        };

        let updated = Turn {
            current: self.turns,
            previous: Some(turn.current),
        };

        self.data.insert(value, updated);
    }
}

pub struct AoC2020_15 {
    input: Vec<Int>,
}

impl AoC2020_15 {
    pub fn new() -> io::Result<Self> {
        let input = std::fs::read_to_string("input/aoc2020_15")?;
        Ok(Self::parse(&input))
    }

    fn parse(data: &str) -> Self {
        let input = data
            .split(",")
            .map(|s| s.trim())
            .map(|x| x.parse::<Int>().expect("Invalid input value"))
            .collect();
        Self { input }
    }
}

impl Solution for AoC2020_15 {
    fn part_one(&self) -> String {
        let mut store = Store::new(&self.input);
        while store.turns < 2020 {
            store.next_turn();
        }
        store.last.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 15: Rambunctious Recitation".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_15_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2020_15_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "492");
        Ok(())
    }

    #[test]
    fn aoc2020_15_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_15> {
        AoC2020_15::new()
    }

    #[test]
    fn aoc2020_15_cases1() {
        let data = [
            ("1,3,2", "1"),
            ("2,1,3", "10"),
            ("1,2,3", "27"),
            ("2,3,1", "78"),
            ("3,2,1", "438"),
            ("3,1,2", "1836"),
        ];
        data.into_iter().for_each(|(inp, out)| {
            let sol = AoC2020_15::parse(inp);
            assert_eq!(sol.part_one(), out);
        });
    }
}
