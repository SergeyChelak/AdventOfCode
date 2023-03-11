use crate::solution::Solution;
use crate::utils::*;

use std::io;
use std::collections::HashSet;

struct Replacement {
    value: String,
    substitute: String
}

impl Replacement {
    fn from_str(s: &str) -> Self {
        let pair = s.split_once(" => ")
            .expect("Invalid replacement string format");
        Self {
            value: pair.0.to_string(),
            substitute: pair.1.to_string(),
        }
    }

    fn inversed(&self) -> Self {
        Self {
            value: self.substitute.clone(),
            substitute: self.value.clone()
        }
    }
}

pub struct AoC2015_19 {
    molecule: String,
    replacement: Vec<Replacement>,
}

impl AoC2015_19 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2015_19")?;
        let index = lines.iter().position(|elem| elem.is_empty())
            .expect("Empty line is expected");
        let replacement = lines[..index].iter()
            .map(|elem| Replacement::from_str(elem))
            .collect::<Vec<Replacement>>();
        let molecule = lines[lines.len() - 1].to_string();
        Ok(Self {
            molecule,
            replacement,
        })
    }

    fn collect_replacements(&self, repl: &Replacement, into: &mut HashSet<String>) {        
        for (pos, _) in self.molecule.match_indices(&repl.value) {
            let modified = self.molecule[..pos].to_string() + &repl.substitute + &self.molecule[(pos + repl.value.len())..];
            into.insert(modified);
        }
    }

    fn find_fewest_steps(&self) -> usize {
        let repl = self.replacement.iter()
            .map(|r| r.inversed())
            .collect::<Vec<Replacement>>();
        let mut count = 0usize;
        let mut mol = self.molecule.clone();
        let mut has_next = true;
        while has_next {
            has_next = false;
            for r in &repl {
                if let Some(pos) = mol.find(&r.value) {
                    mol = mol[..pos].to_string() + &r.substitute + &mol[(pos + r.value.len())..];
                    count += 1;
                    has_next = true;
                }
            }
        }
        count
    }
}

impl Solution for AoC2015_19 {
    fn part_one(&self) -> String {
        let mut variants = HashSet::new();
        for r in &self.replacement {
            self.collect_replacements(r, &mut variants);
        }
        variants.len().to_string()
    }

    fn part_two(&self) -> String {
        self.find_fewest_steps()
            .to_string()
    }

    fn description(&self) -> String {
        "AoC 2015/Day 19: Medicine for Rudolph".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_19_input_load_test() -> io::Result<()> {
        let sol = AoC2015_19::new()?;
        assert!(sol.molecule.len() > 0);
        assert_eq!(sol.replacement.len(), 43);
        Ok(())
    }

    #[test]
    fn aoc2015_19_correctness() -> io::Result<()> {
        let sol = AoC2015_19::new()?;
        assert_eq!(sol.part_one(), "509");
        assert_eq!(sol.part_two(), "195");
        Ok(())
    }

    #[test]
    fn aoc2015_19_search() {
        let repl = vec![
            "e => H",
            "e => O",
            "H => HO",
            "H => OH",
            "O => HH"
        ].iter()
        .map(|&s| s.to_string())
        .map(|s| Replacement::from_str(&s))
        .collect::<Vec<Replacement>>();
        let sol = AoC2015_19 {
            replacement: repl,
            molecule: "HOHOHO".to_string()
        };
        let steps = sol.find_fewest_steps();
        assert_eq!(steps, 6);
    }
}