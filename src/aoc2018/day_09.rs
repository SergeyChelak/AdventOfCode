use crate::solution::Solution;

use std::collections::LinkedList;
use std::io::Result;

pub struct AoC2018_09 {
    players: usize,
    marbles: usize,
}

impl AoC2018_09 {
    pub fn new() -> Result<Self> {
        Ok(Self {
            players: 410,
            marbles: 72059,
        })
    }
}

impl Solution for AoC2018_09 {
    fn part_one(&self) -> String {
        max_scores(self.marbles, self.players).to_string()
    }

    fn part_two(&self) -> String {
        max_scores(self.marbles * 100, self.players).to_string()
    }

    fn description(&self) -> String {
        "AoC 2018/Day 9: Marble Mania".to_string()
    }
}

fn max_scores(marbles: usize, players: usize) -> usize {
    let mut circle: LinkedList<usize> = LinkedList::new();
    circle.push_back(0);
    let mut scores = vec![0usize; players];
    for marble in 1..=marbles {
        if marble % 23 == 0 {
            let player = (marble - 1) % players;
            for _ in 0..7 {
                let val = circle.pop_back().unwrap();
                circle.push_front(val);
            }
            let val = circle.pop_back().unwrap();
            scores[player] += marble + val;
            let val = circle.pop_front().unwrap();
            circle.push_back(val);
        } else {
            let val = circle.pop_front().unwrap();
            circle.push_back(val);

            circle.push_back(marble);
        }
    }
    *scores.iter().max().expect("Scores map is empty")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_09_example1() {
        // players, marbles, high scores
        let data = [
            (9, 25, 32),
            (10, 1618, 8317),
            (13, 7999, 146373),
            (17, 1104, 2764),
            (21, 6111, 54718),
            (30, 5807, 37305),
        ];
        data.iter().for_each(|x| {
            let sol = AoC2018_09 {
                players: x.0,
                marbles: x.1,
            };
            assert_eq!(sol.part_one(), x.2.to_string());
        });
    }

    #[test]
    fn aoc2018_09_correctness() -> Result<()> {
        let sol = AoC2018_09::new()?;
        assert_eq!(sol.part_one(), "429287");
        assert_eq!(sol.part_two(), "3624387659");
        Ok(())
    }
}
