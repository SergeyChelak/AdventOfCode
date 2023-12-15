use crate::solution::Solution;

use std::io;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Lens {
    name: String,
    focal_len: u32,
}

enum Step {
    FocalLength(String, u32),
    Remove(String),
}

impl Step {
    fn name(&self) -> &str {
        match self {
            Self::FocalLength(name, _) | Self::Remove(name) => name.as_str(),
        }
    }
}

impl From<&str> for Step {
    fn from(value: &str) -> Self {
        if value.contains('=') {
            let (name, foc) = value
                .split_once('=')
                .expect("Failed to split label/focal distance");
            let length = foc.parse::<u32>().expect("Focal length must be int");
            Self::FocalLength(name.to_string(), length)
        } else {
            let name = &value[..value.len() - 1];
            Self::Remove(name.to_string())
        }
    }
}

fn hash(s: &str) -> u32 {
    s.chars().fold(0u32, |acc, ch| {
        let cur = ch as u8 as u32;
        (acc + cur) * 17 % 256
    })
}

fn focusing_power(boxes: &[Vec<Lens>]) -> usize {
    boxes
        .iter()
        .enumerate()
        .map(|(box_idx, b)| {
            b.iter()
                .enumerate()
                .map(|(idx, elem)| (box_idx + 1) * (idx + 1) * elem.focal_len as usize)
                .sum::<usize>()
        })
        .sum()
}

pub struct AoC2023_15 {
    input: Vec<String>,
}

impl AoC2023_15 {
    pub fn new() -> io::Result<Self> {
        let input = std::fs::read_to_string("input/aoc2023_15")?;
        Ok(Self::with_str(&input))
    }

    fn with_str(s: &str) -> Self {
        let input = s
            .trim()
            .split(',')
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2023_15 {
    fn part_one(&self) -> String {
        self.input.iter().map(|s| hash(s)).sum::<u32>().to_string()
    }

    fn part_two(&self) -> String {
        let steps = self
            .input
            .iter()
            .map(|val| Step::from(val.as_str()))
            .collect::<Vec<_>>();
        let mut boxes = vec![Vec::<Lens>::new(); 256];
        for step in steps {
            let name = step.name();
            let id = hash(name) as usize;
            let b = &mut boxes[id];
            let position = b.iter().position(|elem| elem.name == name);
            match step {
                Step::FocalLength(_, len) => {
                    if let Some(idx) = position {
                        b[idx].focal_len = len;
                    } else {
                        b.push(Lens {
                            name: name.to_string(),
                            focal_len: len,
                        })
                    }
                }
                Step::Remove(_) => {
                    if let Some(idx) = position {
                        b.remove(idx);
                    }
                }
            }
        }
        focusing_power(&boxes).to_string()
    }

    fn description(&self) -> String {
        "AoC 2023/Day 15: Lens Library".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_15_input_load_test() -> io::Result<()> {
        let sol = AoC2023_15::new()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2023_15_hash() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn=1"), 30);
    }

    #[test]
    fn aoc2023_15_ex1() {
        assert_eq!(puzzle().part_one(), "1320");
    }

    #[test]
    fn aoc2023_15_ex2() {
        assert_eq!(puzzle().part_two(), "145");
    }

    fn puzzle() -> AoC2023_15 {
        AoC2023_15::with_str("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")
    }

    #[test]
    fn aoc2023_15_correctness() -> io::Result<()> {
        let sol = AoC2023_15::new()?;
        assert_eq!(sol.part_one(), "516804");
        assert_eq!(sol.part_two(), "231844");
        Ok(())
    }
}
