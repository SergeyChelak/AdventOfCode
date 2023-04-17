use crate::solution::Solution;
use crate::utils::*;

use std::cmp::Ordering;
use std::io;

pub struct AoC2016_06 {
    lines: Vec<String>,
}

impl AoC2016_06 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2016_06")?;
        Ok(Self { lines })
    }
}

impl Solution for AoC2016_06 {
    fn part_one(&self) -> String {
        correct_message(&self.lines, Ordering::Less)
    }

    fn part_two(&self) -> String {
        correct_message(&self.lines, Ordering::Greater)
    }

    fn description(&self) -> String {
        "AoC 2016/Day 6: Signals and Noise".to_string()
    }
}

fn correct_message(lines: &Vec<String>, ordering: Ordering) -> String {
    let len = lines[0].len();
    let mut freq_matrix = vec![vec![0; 26]; len];
    for line in lines {
        let chars = line.chars().into_iter().collect::<Vec<char>>();
        for i in 0..chars.len() {
            let pos = chars[i] as u8 - b'a';
            freq_matrix[i][pos as usize] += 1;
        }
    }
    freq_matrix
        .iter()
        .map(|freq| {
            let mut iter = freq.iter().enumerate();
            let init = iter.next().expect("Array shouldn't be empty");
            iter.fold(init, |(res_idx, res_val), (v_idx, v_val)| {
                let ord = res_val.cmp(v_val);
                if ord == ordering {
                    (v_idx, v_val)
                } else {
                    (res_idx, res_val)
                }
            })
            .0
        })
        .map(|val| (val as u8 + b'a') as char)
        .collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_06_input_load_test() -> io::Result<()> {
        let sol = AoC2016_06::new()?;
        assert!(!sol.lines.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2016_06_correctness() -> io::Result<()> {
        let sol = AoC2016_06::new()?;
        assert_eq!(sol.part_one(), "qrqlznrl");
        assert_eq!(sol.part_two(), "kgzdfaon");
        Ok(())
    }

    #[test]
    fn aoc2016_06_demo_case1() {
        let lines = [
            "eedadn", "drvtee", "eandsr", "raavrd", "atevrs", "tsrnev", "sdttsa", "rasrtv",
            "nssdts", "ntnada", "svetve", "tesnvt", "vntsnd", "vrdear", "dvrsen", "enarar",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        let sol = AoC2016_06 { lines };
        assert_eq!(sol.part_one(), "easter");
    }
}
