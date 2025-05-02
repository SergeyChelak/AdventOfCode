use crate::solution::Solution;

use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;
use std::io;

use super::intcode_computer::*;

pub struct AoC2019_23 {
    program: Memory,
}

impl AoC2019_23 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2019_23")?;
        Ok(Self::with_str(&input))
    }

    fn with_str(input: &str) -> Self {
        Self {
            program: parse_program(input),
        }
    }
}

impl Solution for AoC2019_23 {
    fn part_one(&self) -> String {
        let count: usize = 50;
        let mut computers = (0..count)
            .map(|addr| {
                let mut comp = IntcodeComputer::with_memory(&self.program);
                comp.push_input(addr as Int);
                comp
            })
            .map(|comp| Computer::with_computer(comp))
            .collect::<Vec<_>>();
        let mut nic = NetworkInterfaceController::new(count);
        while nic.addr255.is_none() {
            computers.iter_mut().for_each(|comp| {
                assert!(comp.is_alive());
                comp.execute_step();
            });
            nic.send(&mut computers);
            nic.receive(&mut computers);
        }
        nic.addr255
            .map(|(_, val)| val.to_string())
            .unwrap_or("Not found".to_string())
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 23: Category Six".to_string()
    }
}

struct NetworkInterfaceController {
    message_queue: Vec<VecDeque<Int>>,
    addr255: Option<PackageData>,
}

impl NetworkInterfaceController {
    fn new(computers: usize) -> Self {
        let message_queue = vec![VecDeque::new(); computers];
        Self {
            message_queue,
            addr255: None,
        }
    }

    fn send(&mut self, computers: &mut [Computer]) {
        let mut packages = Vec::<Package>::new();
        for queue in self.message_queue.iter_mut() {
            if queue.len() < 3 {
                continue;
            }
            let addr = queue.pop_front().expect("Failed pop address");
            let x = queue.pop_front().expect("Failed pop X value");
            let y = queue.pop_front().expect("Failed pop Y value");
            let package = Package { addr, data: (x, y) };
            packages.push(package);
        }

        let mut no_input_addresses = (0..computers.len()).collect::<HashSet<_>>();
        for package in packages {
            if package.addr == 255 {
                self.addr255 = Some(package.data);
                continue;
            }
            let addr = package.addr as usize;
            no_input_addresses.remove(&addr);
            computers[addr].send_package(package.data);
        }

        for addr in no_input_addresses {
            if !computers[addr].is_waiting_input() {
                continue;
            }
            computers[addr].send(-1);
        }
    }

    fn receive(&mut self, computers: &mut [Computer]) {
        for (addr, comp) in computers.iter_mut().enumerate() {
            while let Some(val) = comp.receive() {
                self.message_queue[addr].push_back(val);
            }
        }
    }
}

struct Package {
    data: PackageData,
    addr: Int,
}

type PackageData = (Int, Int);

#[derive(Clone, Copy)]
enum ComputerState {
    None,
    Executing,
    WaitForInput,
    Halted,
    Error,
}

struct Computer {
    inner: IntcodeComputer,
    state: ComputerState,
}

impl Computer {
    fn with_computer(computer: IntcodeComputer) -> Self {
        Self {
            inner: computer,
            state: ComputerState::None,
        }
    }

    fn execute_step(&mut self) {
        let result = self.inner.execute_step();
        self.state = match result {
            Ok(()) => ComputerState::Executing,
            Err(ExecutionStatus::Halted) => ComputerState::Halted,
            Err(ExecutionStatus::WaitForInput) => ComputerState::WaitForInput,
            Err(ExecutionStatus::WrongInstruction { .. }) => ComputerState::Error,
        }
    }

    fn send_package(&mut self, data: PackageData) {
        self.inner.push_input(data.0);
        self.inner.push_input(data.1);
    }

    fn send(&mut self, value: Int) {
        self.inner.push_input(value);
    }

    fn receive(&mut self) -> Option<Int> {
        self.inner.pop_output()
    }

    fn is_waiting_input(&self) -> bool {
        matches!(self.state, ComputerState::WaitForInput)
    }

    fn is_alive(&self) -> bool {
        !self.is_failed() && !self.is_halted()
    }

    fn is_failed(&self) -> bool {
        matches!(self.state, ComputerState::Error)
    }

    fn is_halted(&self) -> bool {
        matches!(self.state, ComputerState::Halted)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_23_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.program.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2019_23_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "18513");
        Ok(())
    }

    #[test]
    fn aoc2019_23_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_23> {
        AoC2019_23::new()
    }
}
