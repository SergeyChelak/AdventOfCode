use crate::solution::Solution;
use crate::utils::*;
use regex::Regex;
use std::io;

#[derive(Debug, Clone, PartialEq)]
struct Command {
    count: usize,
    from: usize,
    to: usize,
}

pub struct AoC2022_05 {
    stacks: Vec2<char>,
    commands: Vec<Command>,
}

impl AoC2022_05 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2022_05")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        let (stacks, commands) = data.split_once("\n\n").expect("Invalid input");
        let stacks = Self::parse_stacks(stacks);
        let commands = Self::parse_commands(commands);
        Self { stacks, commands }
    }

    fn parse_stacks(data: &str) -> Vec2<char> {
        let stacks = data
            .split('\n')
            .map(|row| row.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .transposed()
            .flipped_horizontally()
            .into_iter()
            .filter_map(|chars| {
                if chars.first().map(|ch| ch.is_ascii_digit()) != Some(true) {
                    return None;
                }
                Some(
                    chars[1..]
                        .iter()
                        .filter(|ch| !ch.is_ascii_whitespace())
                        .cloned()
                        .collect(),
                )
            })
            .collect::<Vec<_>>();
        stacks
    }

    fn parse_commands(data: &str) -> Vec<Command> {
        let Ok(regex) = Regex::new(r"move (\d+) from (\d+) to (\d+)") else {
            panic!("Invalid regexp")
        };
        let mut commands = Vec::new();
        for line in data.split('\n').map(|x| x.trim()).filter(|x| !x.is_empty()) {
            let captures = regex.captures(line).expect("Invalid command format");
            let values = (1..=3)
                .map(|i| {
                    captures
                        .get(i)
                        .map(|x| x.as_str().parse::<usize>().unwrap())
                        .expect("Invalid 'count' value")
                })
                .collect::<Vec<_>>();

            let command = Command {
                count: values[0],
                from: values[1] - 1,
                to: values[2] - 1,
            };
            commands.push(command)
        }
        commands
    }
}

impl Solution for AoC2022_05 {
    fn part_one(&self) -> String {
        let mut stacks = self.stacks.clone();
        for cmd in self.commands.iter() {
            for _ in 0..cmd.count {
                let value = stacks[cmd.from].pop().unwrap();
                stacks[cmd.to].push(value);
            }
        }
        top_chars(&stacks)
    }

    fn part_two(&self) -> String {
        let mut stacks = self.stacks.clone();
        for cmd in self.commands.iter() {
            let mut tail = stacks[cmd.from]
                .iter()
                .rev()
                .take(cmd.count)
                .rev()
                .cloned()
                .collect::<Vec<_>>();
            stacks[cmd.to].append(&mut tail);

            let len = stacks[cmd.from].len();
            stacks[cmd.from].truncate(len - cmd.count);
        }
        top_chars(&stacks)
    }

    fn description(&self) -> String {
        "Day 5: Supply Stacks".to_string()
    }
}

fn top_chars(stacks: &Vec2<char>) -> String {
    stacks
        .iter()
        .filter_map(|arr| arr.last())
        .collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_05_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.commands.is_empty());
        assert!(!sol.stacks.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2022_05_parse_command() {
        let input = "move 1 from 2 to 3";
        let arr = AoC2022_05::parse_commands(input);
        assert_eq!(
            arr,
            vec![Command {
                count: 1,
                from: 2,
                to: 3
            }]
        )
    }

    #[test]
    fn aoc2022_05_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "RFFFWBPNS");
        Ok(())
    }

    #[test]
    fn aoc2022_05_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "CQQBBJFCS");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2022_05> {
        AoC2022_05::new()
    }
}
