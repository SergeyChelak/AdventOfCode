use crate::solution::Solution;
use crate::utils::*;

use std::io;

use chrono::NaiveDateTime;

enum Event {
    Begin(NaiveDateTime, usize),
    WakeUp(NaiveDateTime),
    Asleep(NaiveDateTime),
}

impl Event {
    fn from_str(s: &str) -> Self {
        let index = s.find(']').expect("Closing square bracket not found");
        let formatted_date = &s[1..index];
        let date = NaiveDateTime::parse_from_str(formatted_date, "%Y-%m-%d %H:%M")
            .expect("Failed to parse date");
        let tokens = s[index + 1..].trim().split(' ').collect::<Vec<&str>>();
        match tokens[0] {
            "wakes" => Self::WakeUp(date),
            "falls" => Self::Asleep(date),
            "Guard" => {
                let id = tokens[1][1..]
                    .parse::<usize>()
                    .expect("Failed to parse guard id");
                Self::Begin(date, id)
            }
            _ => panic!("Unexpected token {}", tokens[0]),
        }
    }

    fn duration(&self, other: &Self) -> i64 {
        (self.time() - other.time()).num_minutes()
    }

    fn time(&self) -> NaiveDateTime {
        match self {
            Self::Asleep(time) => *time,
            Self::WakeUp(time) => *time,
            Self::Begin(time, _) => *time,
        }
    }
}

pub struct AoC2018_04 {
    records: Vec<Event>,
}

impl AoC2018_04 {
    pub fn new() -> io::Result<Self> {
        let mut records = read_file_as_lines("input/aoc2018_04")?;
        records.sort();
        let records = records
            .iter()
            .map(|s| Event::from_str(s))
            .collect::<Vec<Event>>();
        Ok(Self { records })
    }
}

impl Solution for AoC2018_04 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 4: Repose Record".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_04_input_load_test() -> io::Result<()> {
        let sol = AoC2018_04::new()?;
        assert!(!sol.records.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_04_correctness() -> io::Result<()> {
        let sol = AoC2018_04::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
