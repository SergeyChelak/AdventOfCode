use crate::solution::Solution;
use crate::utils::*;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

type Int = i64;

struct Item {
    hand: String,
    bid: Int,
}

fn remap_hand(s: &str) -> String {
    s.chars()
        .map(|ch| match ch {
            'A' => 'm',
            'K' => 'l',
            'Q' => 'k',
            'J' => 'j',
            'T' => 'i',
            '9' => 'h',
            '8' => 'g',
            '7' => 'f',
            '6' => 'e',
            '5' => 'd',
            '4' => 'c',
            '3' => 'b',
            '2' => 'a',
            _ => panic!("Unexpected char {ch}"),
        })
        .collect()
}

const FIVE_OF_A_KIND: u8 = 9;
const FOUR_OF_A_KIND: u8 = 8;
const FULL_HOUSE: u8 = 7;
const THREE_OF_A_KIND: u8 = 6;
const TWO_PAIR: u8 = 5;
const ONE_PAIR: u8 = 4;
const HIGH_CARD: u8 = 3;

fn hand_kind(s: &str) -> u8 {
    let mut map: HashMap<char, u8> = HashMap::new();
    s.chars().for_each(|ch| {
        let entry = map.entry(ch).or_default();
        *entry += 1;
    });
    let mut arr = [0u8; 6];
    for &val in map.values() {
        arr[val as usize] += 1;
    }
    if arr[5] == 1 {
        return FIVE_OF_A_KIND;
    }
    if arr[4] == 1 {
        return FOUR_OF_A_KIND;
    }
    if arr[3] == 1 && arr[2] == 1 {
        return FULL_HOUSE;
    }
    if arr[3] == 1 {
        return THREE_OF_A_KIND;
    }
    if arr[2] == 2 {
        return TWO_PAIR;
    }
    if arr[2] == 1 {
        return ONE_PAIR;
    }
    HIGH_CARD
}

pub struct AoC2023_07 {
    input: Vec<Item>,
}

impl AoC2023_07 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_07")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let input = lines
            .iter()
            .filter_map(|s| {
                let (h, b) = s.split_once(' ')?;
                let bid = b.parse::<Int>().ok()?;
                Some(Item {
                    hand: h.to_string(),
                    bid,
                })
            })
            .collect::<Vec<_>>();
        Self { input }
    }

    fn remap_hands(&self) -> Vec<Item> {
        self.input
            .iter()
            .map(|item| Item {
                hand: remap_hand(&item.hand),
                ..*item
            })
            .collect()
    }
}

impl Solution for AoC2023_07 {
    fn part_one(&self) -> String {
        let mut input = self.remap_hands();
        input.sort_by(|a, b| {
            let kind_a = hand_kind(&a.hand);
            let kind_b = hand_kind(&b.hand);
            let cmp = kind_a.cmp(&kind_b);
            match cmp {
                Ordering::Equal => a.hand.cmp(&b.hand),
                _ => cmp,
            }
        });
        input
            .iter()
            .enumerate()
            .map(|(rank, item)| (rank + 1) as Int * item.bid)
            .sum::<Int>()
            .to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2023/Day 7: Camel Cards".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_07_input_load_test() -> io::Result<()> {
        let sol = AoC2023_07::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_07_ex1() {
        let lines = [
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let sol = AoC2023_07::with_lines(&lines);
        assert_eq!(sol.part_one(), "6440")
    }

    #[test]
    fn aoc2023_07_kind() {
        [
            ("23456", HIGH_CARD),
            ("32T3K", ONE_PAIR),
            ("T55J5", THREE_OF_A_KIND),
            ("KK677", TWO_PAIR),
            ("KTJJT", TWO_PAIR),
            ("QQQJA", THREE_OF_A_KIND),
            ("AAAAA", FIVE_OF_A_KIND),
            ("AA8AA", FOUR_OF_A_KIND),
            ("23332", FULL_HOUSE),
        ]
        .iter()
        .for_each(|(pattern, kind)| {
            assert_eq!(*kind, hand_kind(pattern));
        });
    }

    #[test]
    fn aoc2023_07_correctness() -> io::Result<()> {
        let sol = AoC2023_07::new()?;
        assert_eq!(sol.part_one(), "250957639");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
