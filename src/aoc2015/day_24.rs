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
}

fn layout_boxes(weight: &Vec<usize>, target: usize) -> Option<(usize, usize)> {
    let mut tab: Vec<Option<(usize, usize)>> = vec![None; target + 1];
    for &w in weight {
        if w <= target {
            tab[w] = Some((1, w));
        }
    }
    for i in 1..=target {
        if let Some((count, quantum_entanglement)) = tab[i] {
            for &w in weight {
                let idx = i + w;
                if idx > target {
                    continue;
                }
                let next_cnt = 1 + count;
                let next_qe = quantum_entanglement * w;
                tab[idx] = if let Some((c, q)) = tab[idx] {
                    if c > next_cnt {
                        Some((next_cnt, next_qe))
                    } else if c == next_cnt {
                        Some((c, next_qe.min(q)))
                    } else {
                        tab[idx]
                    }
                } else {
                    Some((next_cnt, next_qe))
                }
            }
        }
    }
    tab[target]
}

impl Solution for AoC2015_24 {
    fn part_one(&self) -> String {
        let sum: usize = self.input.iter().sum();
        if sum % 3 == 0 {
            let target = sum / 3;
            if let Some((_, qe)) = layout_boxes(&self.input, target) {
                qe.to_string()
            } else {
                "Can't align boxes to get target weight"
                .to_string()    
            }
            
        } else {
            "Can't divide input weights with equal parts"
                .to_string()
        }
    }

    // fn part_two(&self) -> String {
    // }

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
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2015_24_layout_boxes() {
        let weights = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        let sum: usize = weights.iter().sum();
        assert_eq!(sum % 3, 0);
        let target = sum / 3;
        let (count, qe) = layout_boxes(&weights, target).unwrap();
        assert_eq!(count, 2);
        assert_eq!(qe, 99);
    }
}