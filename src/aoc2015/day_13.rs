use crate::solution::Solution;
use crate::utils::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
type Graph = HashMap<(usize, usize), i32>;

pub struct AoC2015_13 {
    graph: RefCell<Graph>,
    count: RefCell<usize>,
}

impl AoC2015_13 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2015_13")?;
        let (graph, count) = Self::parse_input(&lines);
        Ok(Self {
            graph: RefCell::new(graph),
            count: RefCell::new(count),
        })
    }

    fn parse_input(lines: &Vec<String>) -> (Graph, usize) {
        let mut graph = Graph::new();
        let mut id_mapper = String2IdMapper::new();
        for line in lines {
            let (name_from, name_to, value) = Self::parse_line(line);
            let from = id_mapper.get_id(&name_from);
            let to = id_mapper.get_id(&name_to);
            graph.insert((from, to), value);
        }
        (graph, id_mapper.len())
    }

    fn parse_line(line: &str) -> (String, String, i32) {
        let line = &line[..line.len() - 1];
        let arr = line.split(" ").collect::<Vec<&str>>();
        let from = arr[0].to_string();
        let to = arr[arr.len() - 1].to_string();
        let sign = if arr[2] == "lose" { -1 } else { 1 };
        let value = arr[3].parse::<i32>().expect("Integer value is expected");
        (from, to, sign * value)
    }

    fn calculate(&self) -> String {
        let count = *self.count.borrow();
        let mut arr: Vec<usize> = vec![0; count];
        for i in 0..count {
            arr[i] = i;
        }
        let val = arr.permut_iter()
            .map(|v| self.fit(&v))
            .fold(None, |acc, v| bigger_option(&acc, &v));
        if let Some(v) = val {
            v.to_string()
        } else {
            "Not found".to_string()
        }
    }

    fn fit(&self, order: &Vec<usize>) -> Option<i32> {
        let mut sum = 0i32;
        let n = order.len();
        let graph = self.graph.borrow();
        for i in 0..n {
            let prev_idx = if i > 0 { i - 1 } else { n - 1};
            let prev = (order[i], order[prev_idx]);
            let next = (order[i], order[(i + 1) % n]);
            if let (Some(val1), Some(val2)) = (graph.get(&prev), graph.get(&next)) {
                sum += val1 + val2;
            } else {
                return None;
            }
        }
        Some(sum)
    }
}

impl Solution for AoC2015_13 {
    fn part_one(&self) -> String {
        self.calculate()
    }

    fn part_two(&self) -> String {
        let mut graph = self.graph.borrow().clone();
        let my_id = *self.count.borrow();
        for i in 0..my_id {
            graph.insert((my_id, i), 0);
            graph.insert((i, my_id), 0);
        }
        self.graph.replace(graph);
        self.count.replace(my_id + 1);
        self.calculate()
    }

    fn description(&self) -> String {
        "AoC 2015/Day 13: Knights of the Dinner Table".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_13_input_load_test() -> io::Result<()> {
        let sol = AoC2015_13::new()?;
        assert!(*sol.count.borrow() > 0);
        assert!(sol.graph.borrow().len() > 0);
        Ok(())
    }

    #[test]
    fn aoc2015_13_correctness() -> io::Result<()> {
        let sol = AoC2015_13::new()?;
        assert_eq!(sol.part_one(), "733");
        assert_eq!(sol.part_two(), "725");
        Ok(())
    }
}