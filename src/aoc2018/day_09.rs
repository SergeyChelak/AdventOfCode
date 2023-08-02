use crate::solution::Solution;

use std::collections::{HashMap, LinkedList};
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
    let mut circle: Vec<usize> = Vec::with_capacity(marbles);
    circle.push(0);
    circle.push(1);
    let mut scores: HashMap<usize, usize> = HashMap::new();
    let mut player = 1usize;
    let mut position = 1usize;
    for marble in 2..=marbles {
        if marble % 23 == 0 {
            let entry = scores.entry(player).or_insert(0);
            *entry += marble;
            let index = {
                let mut p = position;
                if p < 7 {
                    p += circle.len();
                }
                p - 7
            };
            let val = circle[index];
            *entry += val;
            circle.remove(index);
            position = index;
        } else {
            let mut next = position + 2;
            let len = circle.len();
            if next == len {
                circle.push(marble);
            } else {
                next %= len;
                circle.insert(next, marble);
            }
            position = next;
        }
        player = (player + 1) % players;
    }
    *scores
        .iter()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .expect("Scores map is empty")
        .1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_09_example1() {
        // players, marbles, high scores
        let data = vec![
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
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
