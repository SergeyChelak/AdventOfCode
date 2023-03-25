use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2015_24 {
    input: Vec<usize>
}

impl AoC2015_24 {
    pub fn new() -> io::Result<Self> {
        let input = read_file_as_lines("input/aoc2015_24")?
            .iter()
            .map(|s| s.parse::<usize>().expect("Non numerical value found in input"))
            .collect::<Vec<usize>>();
        Ok(Self {
            input
        })
    }

    fn search(&self, value: usize, position: usize, target: usize, items: &mut Vec<usize>, output: &mut (usize, usize)) {
        if value < target {
            for i in position..self.input.len() {
                let val = self.input[i];
                items.push(val);
                self.search(value + val, i + 1, target, items, output);
                items.pop();
            }
        } else if value == target {
            let cur_cnt = items.len();
            let (cnt, qe) = *output;
            let cur_qe = items.iter().fold(1usize, |acc, v| acc.saturating_mul(*v));
            if cur_cnt < cnt {
                *output = (cur_cnt, cur_qe);
            } else if cur_cnt == cnt {
                *output = (cur_cnt, cur_qe.min(qe));
            }
        }
    }

    fn accommodate_boxes(&self, trunks: usize) -> Option<usize> {
        let sum: usize = self.input.iter().sum();        
        if sum % trunks == 0 {
            let target = sum / trunks;
            let mut output = (usize::MAX, usize::MAX);
            self.search(0, 0, target, &mut Vec::new(), &mut output);
            Some(output.1)
        } else {
            None
        }
    }

    fn format_output(&self, value: Option<usize>) -> String {
        if let Some(value) = value {
            value.to_string()
        } else {
            "Can't divide input weights with equal parts"
                .to_string()
        }
    }
}

impl Solution for AoC2015_24 {
    fn part_one(&self) -> String {                
        let result = self.accommodate_boxes(3);
        self.format_output(result)
    }

    fn part_two(&self) -> String {
        let result = self.accommodate_boxes(4);
        self.format_output(result)
    }

    fn description(&self) -> String {
        "AoC 2015/Day 24: It Hangs in the Balance".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_24_input_load_test() -> io::Result<()> {
        let sol = AoC2015_24::new()?;
        assert_eq!(sol.input.len(), 29);
        Ok(())
    }

    #[test]
    fn aoc2015_24_correctness() -> io::Result<()> {
        let sol = AoC2015_24::new()?;
        assert_eq!(sol.part_one(), "10723906903");
        assert_eq!(sol.part_two(), "74850409");
        Ok(())
    }
}