use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

use chrono::{NaiveDateTime, Timelike};

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
}

type Stat = [usize; 60];

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

    fn calc_timeline(&self) -> HashMap<usize, Stat> {
        let mut timeline: HashMap<usize, Stat> = HashMap::new();
        let mut guard_id = 0;
        let mut start_asleep: Option<u32> = None;
        for event in &self.records {
            match event {
                Event::Asleep(time) => start_asleep = Some(time.minute()),
                Event::WakeUp(time) => {
                    let entry = timeline.entry(guard_id).or_insert([0usize; 60]);
                    let start =
                        start_asleep.expect("Asleep time should appear before wake up") as usize;
                    let stop = time.minute() as usize;
                    for x in entry.iter_mut().take(stop).skip(start) {
                        *x += 1;
                    }
                }
                Event::Begin(_, id) => guard_id = *id,
            };
        }
        timeline
    }
}

impl Solution for AoC2018_04 {
    fn part_one(&self) -> String {
        let timeline = self.calc_timeline();
        let (&id, time) = timeline
            .iter()
            .max_by(|(_, a), (_, b)| {
                let sum_a: usize = a.iter().sum();
                let sum_b: usize = b.iter().sum();
                sum_a.cmp(&sum_b)
            })
            .expect("Timeline is empty");
        let best_minute = time
            .iter()
            .enumerate()
            .max_by(|(_, &a), (_, &b)| a.cmp(&b))
            .expect("Timeline isn't filled");
        (id * best_minute.0).to_string()
    }

    fn part_two(&self) -> String {
        let timeline = self.calc_timeline();
        let (id, minute, _) = timeline
            .iter()
            .map(|(key, stat)| {
                let (minute, max) = stat
                    .iter()
                    .enumerate()
                    .max_by(|(_, x), (_, y)| x.cmp(y))
                    .expect("Unreachable");
                (*key, minute, max)
            })
            .max_by(|(_, _, &a), (_, _, &b)| a.cmp(&b))
            .expect("Frequent minute not found");
        (id * minute).to_string()
    }

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
        assert_eq!(sol.part_one(), "101194");
        assert_eq!(sol.part_two(), "102095");
        Ok(())
    }

    #[test]
    fn aoc2018_04_example1() {
        let records = vec![
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep",
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep",
            "[1518-11-05 00:55] wakes up",
        ]
        .iter()
        .map(|s| Event::from_str(s))
        .collect::<Vec<Event>>();
        let sol = AoC2018_04 { records };
        assert_eq!(sol.part_one(), "240");
    }
}
