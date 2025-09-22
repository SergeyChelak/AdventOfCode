use crate::solution::Solution;
use crate::utils::not_found;

use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;
use std::io;

use super::intcode_computer::*;

const IDLE_THRESHOLD: usize = 5_000;

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
        let mut computers = setup_network(count, &self.program);
        let mut nic = NetworkInterfaceController::new(count, false);
        while nic.nat_data.is_none() {
            computers.iter_mut().for_each(|comp| {
                assert!(comp.is_alive());
                comp.execute_step();
            });
            _ = nic.process(&mut computers);
        }
        nic.nat_data
            .map(|(_, val)| val.to_string())
            .unwrap_or(not_found())
    }

    fn part_two(&self) -> String {
        let count: usize = 50;
        let mut computers = setup_network(count, &self.program);
        let mut nic = NetworkInterfaceController::new(count, true);

        let mut prev_data: Option<PackageData> = None;
        loop {
            computers.iter_mut().for_each(|comp| {
                assert!(comp.is_alive());
                comp.execute_step();
            });
            let status = nic.process(&mut computers);
            if matches!(status, NicStatus::Normal) {
                continue;
            }
            match (nic.nat_data, prev_data) {
                (Some(data), Some(prev_data)) if data.1 == prev_data.1 => break data.1,
                _ => {}
            };
            prev_data = nic.nat_data;
        }
        .to_string()
    }

    fn description(&self) -> String {
        "Day 23: Category Six".to_string()
    }
}

fn setup_network(count: usize, program: &[Int]) -> Vec<Computer> {
    (0..count)
        .map(|addr| {
            let mut comp = IntcodeComputer::with_memory(program);
            comp.push_input(addr as Int);
            comp
        })
        .map(Computer::with_computer)
        .collect::<Vec<_>>()
}

enum NicStatus {
    Normal,
    Resumed,
}

struct NetworkInterfaceController {
    message_queue: Vec<VecDeque<Int>>,
    nat_enabled: bool,
    nat_data: Option<PackageData>,
    idle_counter: usize,
}

impl NetworkInterfaceController {
    fn new(computers: usize, nat_enabled: bool) -> Self {
        let message_queue = vec![VecDeque::new(); computers];
        Self {
            message_queue,
            nat_data: None,
            nat_enabled,
            idle_counter: 0,
        }
    }

    fn process(&mut self, computers: &mut [Computer]) -> NicStatus {
        self.receive(computers);
        self.send(computers)
    }

    fn send(&mut self, computers: &mut [Computer]) -> NicStatus {
        let mut status = NicStatus::Normal;
        let mut packages = Vec::<Package>::new();
        for queue in self.message_queue.iter_mut() {
            if queue.len() < 3 {
                continue;
            }
            let addr = queue.pop_front().expect("Failed pop address");
            let data = (
                queue.pop_front().expect("Failed pop X value"),
                queue.pop_front().expect("Failed pop Y value"),
            );

            if addr == 255 {
                self.nat_data = Some(data);
                continue;
            }

            let package = Package { addr, data };
            packages.push(package);
        }

        if self.nat_enabled && packages.is_empty() {
            self.idle_counter += 1;
            let is_idle = self.idle_counter > IDLE_THRESHOLD && computers[0].is_waiting_input();

            if let Some(data) = self
                .nat_data
                .and_then(|data| if is_idle { Some(data) } else { None })
            {
                packages.push(Package { data, addr: 0 });
                status = NicStatus::Resumed;
            }
        }

        let mut no_input_addresses = (0..computers.len()).collect::<HashSet<_>>();
        for package in packages {
            self.idle_counter = 0;
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
        status
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
        assert_eq!(sol.part_two(), "13286");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_23> {
        AoC2019_23::new()
    }
}
