use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, VecDeque};
use std::io;

const MODULE_BROADCAST: &str = "broadcaster";
const SENDER_BUTTON: &str = "button";

#[derive(Clone, Copy, Debug)]
enum Pulse {
    High,
    Low,
}

// input/output mapping
type Link = HashMap<String, Vec<String>>;
type Inputs = HashMap<String, Pulse>;

#[derive(Clone)]
enum Module {
    FlipFlop(bool),
    Conjunction(Inputs),
    Broadcast,
}

struct ModuleSystem {
    outputs: Link,
    modules: HashMap<String, Module>,
    low_count: usize,
    high_count: usize,
}

impl ModuleSystem {
    fn new(outputs: Link, modules: HashMap<String, Module>) -> Self {
        Self {
            outputs,
            modules,
            low_count: 0,
            high_count: 0,
        }
    }

    fn run(&mut self) {
        let mut queue = VecDeque::from([(SENDER_BUTTON, MODULE_BROADCAST, Pulse::Low)]);
        while let Some((sender, current, pulse)) = queue.pop_front() {
            if matches!(pulse, Pulse::High) {
                self.high_count += 1;
            } else {
                self.low_count += 1;
            }
            let Some(module) = self.modules.get(current) else {
                continue;
            };
            let output = self.outputs.get(current).expect("Outputs not found (2)");
            match module {
                Module::Broadcast => {
                    for next in output {
                        queue.push_back((current, next, pulse));
                    }
                }
                Module::FlipFlop(is_active) => {
                    if matches!(pulse, Pulse::Low) {
                        let value = if *is_active { Pulse::Low } else { Pulse::High };
                        for next in output {
                            queue.push_back((current, next, value));
                        }
                        self.modules
                            .insert(current.to_string(), Module::FlipFlop(!*is_active));
                    }
                }
                Module::Conjunction(inputs) => {
                    let mut tmp = inputs.clone();
                    tmp.insert(sender.to_string(), pulse);
                    let value = if tmp.values().all(|x| matches!(x, Pulse::High)) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };
                    for next in output {
                        queue.push_back((current, next, value));
                    }
                    self.modules
                        .insert(current.to_string(), Module::Conjunction(tmp));
                }
            }
        }
    }

    fn dump(&self) {
        let mut keys = self.modules.keys().map(|x| x).collect::<Vec<_>>();
        keys.sort();
        for key in keys {
            let module = self.modules.get(key).unwrap();
            match module {
                Module::Broadcast => println!("Broadcast"),
                Module::FlipFlop(state) => println!("%{key} {state}"),
                Module::Conjunction(inputs) => {
                    print!("&{key} ");
                    for inp in inputs {
                        print!("{}:{:?} ", inp.0, inp.1);
                    }
                    println!();
                }
            }
        }
    }
}

pub struct AoC2023_20 {
    outputs: Link,
    modules: HashMap<String, Module>,
}

impl AoC2023_20 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2023_20")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let mut inputs = Link::new();
        let mut outputs = Link::new();
        let mut conjunction = Vec::new();
        let mut flip_flop = Vec::new();
        for line in lines {
            let (module, dest) = line
                .split_once(" -> ")
                .expect("arrow separator is expected");
            let name = if module == MODULE_BROADCAST {
                module
            } else {
                &module[1..]
            };
            if module.starts_with('%') {
                flip_flop.push(name.to_string());
            } else if module.starts_with('&') {
                conjunction.push(name.to_string());
            }
            let dest_names = dest.split(", ").map(|s| s.to_string()).collect::<Vec<_>>();
            dest_names.iter().for_each(|key| {
                let k = key.clone();
                let entry = inputs.entry(k).or_insert(Vec::new());
                entry.push(name.to_string());
            });
            outputs.insert(name.to_string(), dest_names);
        }

        let mut modules: HashMap<String, Module> = HashMap::new();
        modules.insert(MODULE_BROADCAST.to_string(), Module::Broadcast);

        for module in flip_flop {
            modules.insert(module, Module::FlipFlop(false));
        }

        for module in conjunction {
            let inp = inputs
                .get(&module)
                .expect("Inputs for conjunction not found")
                .iter()
                .map(|x| (x.clone(), Pulse::Low))
                .collect::<HashMap<_, _>>();
            modules.insert(module, Module::Conjunction(inp));
        }
        Self { outputs, modules }
    }
}

impl Solution for AoC2023_20 {
    fn part_one(&self) -> String {
        let mut system = ModuleSystem::new(self.outputs.clone(), self.modules.clone());
        for _ in 0..1000 {
            system.run();
        }
        (system.high_count * system.low_count).to_string()
    }

    fn part_two(&self) -> String {
        let mut system = ModuleSystem::new(self.outputs.clone(), self.modules.clone());
        todo!()
    }

    fn description(&self) -> String {
        "AoC 2023/Day 20: Pulse Propagation".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_20_input_load_test() -> io::Result<()> {
        let sol = AoC2023_20::new()?;
        assert!(!sol.modules.is_empty());
        assert!(!sol.outputs.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_20_ex1_1() {
        let lines = [
            "broadcaster -> a, b, c",
            "%a -> b",
            "%b -> c",
            "%c -> inv",
            "&inv -> a",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let puzzle = AoC2023_20::with_lines(&lines);
        assert_eq!(puzzle.part_one(), "32000000");
    }

    #[test]
    fn aoc2023_20_ex1_2() {
        let lines = [
            "broadcaster -> a",
            "%a -> inv, con",
            "&inv -> b",
            "%b -> con",
            "&con -> output",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let puzzle = AoC2023_20::with_lines(&lines);
        assert_eq!(puzzle.part_one(), "11687500");
    }

    #[test]
    fn aoc2023_20_correctness() -> io::Result<()> {
        let sol = AoC2023_20::new()?;
        assert_eq!(sol.part_one(), "806332748");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
