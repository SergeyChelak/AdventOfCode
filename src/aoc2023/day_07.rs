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

fn remap_hand(s: &str, priority: &str) -> String {
    let map: HashMap<char, char> = priority
        .chars()
        .rev()
        .enumerate()
        .map(|(idx, ch)| (ch, (b'a' + idx as u8) as char))
        .collect();
    s.chars()
        .map(|ch| map.get(&ch).expect("Unexpected char"))
        .collect()
}

#[repr(u8)]
#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

fn hand_kind(s: &str) -> HandType {
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
        return HandType::FiveOfKind;
    }
    if arr[4] == 1 {
        return HandType::FourOfKind;
    }
    if arr[3] == 1 && arr[2] == 1 {
        return HandType::FullHouse;
    }
    if arr[3] == 1 {
        return HandType::ThreeOfKind;
    }
    if arr[2] == 2 {
        return HandType::TwoPair;
    }
    if arr[2] == 1 {
        return HandType::OnePair;
    }
    HandType::HighCard
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

    fn remap_hands(&self, priority: &str) -> Vec<Item> {
        self.input
            .iter()
            .map(|item| Item {
                hand: remap_hand(&item.hand, priority),
                ..*item
            })
            .collect()
    }

    fn total_winnings(&self, priority: &str, kind_cmp: &dyn Fn(&str) -> HandType) -> Int {
        let mut input = self.remap_hands(priority);
        input.sort_by(|a, b| {
            let kind_a = kind_cmp(&a.hand);
            let kind_b = kind_cmp(&b.hand);
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
    }
}

impl Solution for AoC2023_07 {
    fn part_one(&self) -> String {
        self.total_winnings("AKQJT98765432", &hand_kind).to_string()
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
            ("23456", HandType::HighCard),
            ("32T3K", HandType::OnePair),
            ("T55J5", HandType::ThreeOfKind),
            ("KK677", HandType::TwoPair),
            ("KTJJT", HandType::TwoPair),
            ("QQQJA", HandType::ThreeOfKind),
            ("AAAAA", HandType::FiveOfKind),
            ("AA8AA", HandType::FourOfKind),
            ("23332", HandType::FullHouse),
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
