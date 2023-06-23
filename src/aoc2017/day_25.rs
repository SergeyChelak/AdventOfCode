use crate::solution::Solution;
use std::collections::HashMap;
use std::fs::read_to_string;

use std::io;

fn split_block(input: &str) -> Vec<String> {
    input
        .split('\n')
        .map(|x| x.trim())
        .map(|x| x[0..x.len() - 1].to_string())
        .collect()
}

fn token_rev_index(input: &str, index: usize) -> Option<&str> {
    input.split(' ').rev().skip(index).take(1).last()
}

#[derive(Debug)]
enum TapeDirection {
    Left,
    Right,
}

type StateId = char;

#[derive(Debug)]
struct Command {
    write: u8,
    tape_direction: TapeDirection,
    next_state_id: StateId,
}

struct State {
    id: StateId,
    commands: [Command; 2],
}

pub struct AoC2017_25 {
    initial_state_id: StateId,
    steps: usize,
    states: HashMap<StateId, [Command; 2]>,
}

impl AoC2017_25 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2017_25")?;
        Ok(Self::with_str(&input))
    }

    fn with_str(input: &str) -> Self {
        let blocks = input.trim().split("\n\n").collect::<Vec<&str>>();
        let (initial_state_id, steps) = Self::parse_header(blocks[0]);
        let mut states: HashMap<StateId, [Command; 2]> = HashMap::new();
        blocks
            .iter()
            .skip(1)
            .map(|s| Self::parse_state(s))
            .for_each(|state| {
                states.insert(state.id, state.commands);
            });
        Self {
            initial_state_id,
            steps,
            states,
        }
    }

    fn parse_header(input: &str) -> (char, usize) {
        let lines = split_block(input);
        let initial_state = token_rev_index(&lines[0], 0)
            .expect("Start state declaration shouldn't be empty")
            .parse::<char>()
            .expect("Start state should be char");
        let steps = token_rev_index(&lines[1], 1)
            .expect("Step's count for checksum not found")
            .parse::<usize>()
            .expect("Steps count should be usize");
        (initial_state, steps)
    }

    fn parse_state(input: &str) -> State {
        let lines = split_block(input);
        let state_id = token_rev_index(&lines[0], 0)
            .expect("State id string shouldn't be empty")
            .parse::<char>()
            .expect("Failed parse state id");
        let commands = [&lines[1..5], &lines[5..]]
            .iter()
            .map(|x| {
                let val = token_rev_index(&x[1], 0)
                    .expect("String with value to write is empty")
                    .parse::<u8>()
                    .expect("Value to write should be integer");
                let tape_direction =
                    token_rev_index(&x[2], 0).expect("String with direction is empty");
                let tape_direction = if tape_direction == "left" {
                    TapeDirection::Left
                } else {
                    TapeDirection::Right
                };
                let next_state_id = token_rev_index(&x[3], 0)
                    .expect("String with next state id is empty")
                    .parse::<char>()
                    .expect("Next state id should be char");
                Command {
                    write: val,
                    tape_direction,
                    next_state_id,
                }
            })
            .collect::<Vec<Command>>();
        State {
            id: state_id,
            commands: commands.try_into().expect("Incorrect amount of commands"),
        }
    }
}

impl Solution for AoC2017_25 {
    fn part_one(&self) -> String {
        let mut tape: HashMap<isize, u8> = HashMap::new();
        let mut ptr = 0isize;
        let mut cur_state_id = self.initial_state_id;
        for _ in 0..self.steps {
            let val = tape.entry(ptr).or_insert(0);
            let command = &self.states.get(&cur_state_id).expect("State not found")[*val as usize];
            *val = command.write;
            ptr += match command.tape_direction {
                TapeDirection::Left => -1,
                TapeDirection::Right => 1,
            };
            cur_state_id = command.next_state_id;
        }
        tape.values()
            .map(|x| *x as usize)
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        ":)".to_string()
    }

    fn description(&self) -> String {
        "AoC 2017/Day 25: The Halting Problem".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_25_input_load_test() -> io::Result<()> {
        let sol = AoC2017_25::new()?;
        assert!(!sol.states.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_25_correctness() -> io::Result<()> {
        let sol = AoC2017_25::new()?;
        assert_eq!(sol.part_one(), "2870");
        assert_eq!(sol.part_two(), ":)");
        Ok(())
    }
}
