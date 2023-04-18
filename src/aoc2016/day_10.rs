use crate::solution::Solution;
use crate::utils::*;

use std::io;
use std::collections::HashMap;

#[derive(Clone, Copy)]
struct Bot {
    number: usize,
    low_dest: Receiver,
    high_dest: Receiver,
}

#[derive(Clone, Copy)]
struct Destination {
    receiver: Receiver,
    chip_id: usize,
}

#[derive(Clone, Copy)]
enum Receiver {
    Bot(usize),
    Output(usize)
}

impl Receiver {
    fn new(type_str: &str, value: &str) -> Self {
        let id = value.parse::<usize>().expect("receiver id should be integer");
        match type_str {
            "bot" => Self::Bot(id),
            "output" =>  Self::Output(id),
            _ => panic!("Unexpected receiver type {type_str}"),
        }
    }
}

struct Conveyor {
    input: Vec<Destination>,
    instructions: HashMap<usize, Bot>,
    values: HashMap<usize, Vec<usize>>,
    stack: Vec<Destination>,
    output: HashMap<usize, Vec<usize>>,
}

impl Conveyor {
    fn new(input: &[Destination], bot_instr: &[Bot]) -> Self {
        let mut map = HashMap::new();
        bot_instr.iter().for_each(|inst| {
            map.insert(inst.number, *inst);
        });
        Self {
            input: input.to_vec(),
            instructions: map,
            values: HashMap::new(),
            stack: vec![],
            output: HashMap::new(),
        }
    }

    fn run(&mut self) {
        for inp in &self.input {
            self.stack.push(*inp);
            while let Some(dest) = self.stack.pop() {
                match dest.receiver {
                    Receiver::Bot(bot_id) => {
                        let data = self.values
                                .entry(bot_id)
                                .or_insert(vec![]);
                        data.push(dest.chip_id);
                        if data.len() == 2 {
                            let (a, b) = (data[0], data[1]);
                            let (low, high) = (a.min(b), a.max(b));
                            
                            if low == 17 && high == 61 {
                                println!("Bot #{bot_id} compares {low} and {high}");
                                return;
                            }

                            let instr = self.instructions
                                .get(&bot_id)
                                .expect("Instruction not found");
                            self.stack.push(Destination {
                                chip_id: low,
                                receiver: instr.low_dest,
                            });
                            self.stack.push(Destination {
                                chip_id: high,
                                receiver: instr.high_dest,
                            });
                            data.clear();
                        }
                    },
                    Receiver::Output(out_id) => {
                        self.output
                            .entry(out_id)
                            .or_insert(vec![])
                            .push(dest.chip_id);
                    }
                }
            }
        }
    }
}

pub struct AoC2016_10 {
    input: Vec<Destination>,
    bot_instr: Vec<Bot>,
}

impl AoC2016_10 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2016_10")?;
        let (input, bot_instr) = Self::parse_lines(&lines);
        Ok(Self {
            input,
            bot_instr,
        })
    }

    fn parse_lines(lines: &[String]) -> (Vec<Destination>, Vec<Bot>) {
        let mut input = Vec::new();
        let mut bots = Vec::new();
        for line in lines {
            let tokens = line.split(' ').collect::<Vec<&str>>(); 
            match tokens[0] {
                "value" => {
                    let inp = Self::parse_input(&tokens);
                    input.push(inp);
                },
                "bot" => {
                    let bot = Self::parse_bot(&tokens);
                    bots.push(bot);
                },
                _ => panic!("Unexpected token {}", tokens[0]),
            }
        }
        (input, bots)
    }

    fn parse_input(tokens: &[&str]) -> Destination {
        let chip_id = tokens[1].parse::<usize>().expect("value number should be integer");
        let bot_id = tokens[tokens.len() - 1].parse::<usize>().expect("bot number should be integer");
        Destination { 
            receiver: Receiver::Bot(bot_id), 
            chip_id,
        }
    }

    fn parse_bot(tokens: &[&str]) -> Bot {
        let number = tokens[1].parse::<usize>().expect("bot number should be integer");
        let low_dest = Receiver::new(tokens[5], tokens[6]);
        let high_dest = Receiver::new(tokens[10], tokens[11]);
        Bot {
            number,
            low_dest,
            high_dest
        }
    }
}

impl Solution for AoC2016_10 {
    fn part_one(&self) -> String {
        let mut conv = Conveyor::new(&self.input, &self.bot_instr);
        conv.run();
        "Completed".to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2016/Day 10: Balance Bots".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_10_input_load_test() -> io::Result<()> {
        let sol = AoC2016_10::new()?;
        assert!(!sol.input.is_empty());
        assert!(!sol.bot_instr.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2016_10_correctness() -> io::Result<()> {
        let sol = AoC2016_10::new()?;
        assert_eq!(sol.part_one(), "141");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}