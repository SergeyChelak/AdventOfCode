use crate::solution::Solution;

use std::io;
use std::ops::RangeInclusive;

type Int = isize;

struct Field {
    name: String,
    ranges: Vec<RangeInclusive<Int>>,
}

impl From<&str> for Field {
    fn from(value: &str) -> Self {
        let (name, data) = value.split_once(": ").expect("Invalid input format");

        let ranges = data
            .split(" or ")
            .map(|s| {
                let (low, high) = s.split_once('-').expect("Invalid range format");
                let low = low.parse::<Int>().expect("Low range value must be integer");
                let high = high
                    .parse::<Int>()
                    .expect("High range value must be integer");
                low..=high
            })
            .collect::<Vec<_>>();

        Self {
            name: name.to_string(),
            ranges,
        }
    }
}

impl Field {
    fn any_match(&self, value: Int) -> bool {
        self.ranges.iter().any(|r| r.contains(&value))
    }
}

struct Ticket {
    data: Vec<Int>,
}

impl From<&str> for Ticket {
    fn from(value: &str) -> Self {
        let data = value
            .split(',')
            .map(|x| {
                x.trim()
                    .parse::<Int>()
                    .unwrap_or_else(|_| panic!("Ticket data must be integers: {value}"))
            })
            .collect::<Vec<_>>();
        Self { data }
    }
}

impl Ticket {
    fn error_rate(&self, fields: &[Field]) -> Int {
        self.data
            .iter()
            .filter(|val| !fields.iter().any(|f| f.any_match(**val)))
            .sum()
    }
}

pub struct AoC2020_16 {
    fields: Vec<Field>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl AoC2020_16 {
    pub fn new() -> io::Result<Self> {
        let input = std::fs::read_to_string("input/aoc2020_16")?;
        Ok(Self::parse(&input))
    }

    fn parse(input: &str) -> Self {
        let blocks = input.split("\n\n").collect::<Vec<_>>();
        assert_eq!(blocks.len(), 3);

        let fields = blocks[0]
            .split('\n')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(Field::from)
            .collect::<Vec<_>>();

        let my_ticket = {
            let (_, data) = blocks[1]
                .split_once('\n')
                .expect("Invalid my ticket format");
            Ticket::from(data)
        };

        let nearby_tickets = blocks[2]
            .split('\n')
            .skip(1)
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(Ticket::from)
            .collect::<Vec<_>>();

        Self {
            fields,
            my_ticket,
            nearby_tickets,
        }
    }
}

impl Solution for AoC2020_16 {
    fn part_one(&self) -> String {
        self.nearby_tickets
            .iter()
            .map(|t| t.error_rate(&self.fields))
            .sum::<Int>()
            .to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 16: Ticket Translation".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_16_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.fields.is_empty());
        assert!(!sol.nearby_tickets.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2020_16_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "21956");
        Ok(())
    }

    #[test]
    fn aoc2020_16_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_16> {
        AoC2020_16::new()
    }

    #[test]
    fn aoc2020_16_case1() {
        let input = "class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50

        your ticket:
        7,1,14

        nearby tickets:
        7,3,47
        40,4,50
        55,2,20
        38,6,12";
        let sol = AoC2020_16::parse(input);
        assert_eq!(sol.part_one(), "71");
    }
}
