use crate::solution::Solution;

use std::{
    io, 
    collections::{HashMap, hash_map::DefaultHasher, HashSet},
    hash::{Hash, Hasher}
};

const T: bool = true;
const F: bool = false;

type Level = Vec<bool>;
type Facility = Vec<Level>;
type StateHash = u64;

fn dbg_print(step: usize, level: usize, facility: &Facility) {
    println!("Step #{step}");
    for i in (0..facility.len()).rev() {
        let row = &facility[i];
        let ch = if i == level { '>' } else { ' ' };
        print!("{ch:3}");
        for j in 0..row.len() {
            let el = row[j];
            let sym = if j % 2 == 0 { 'G' } else { 'M' };
            let ch = if el { sym } else { '.' };
            print!("{ch:3}");
        }
        println!();
    }
    println!();
}

fn min_steps(facility: &Facility) -> usize {
    fn perform(facility: Facility,
        level: usize, 
        steps: usize, 
        backtrack: &mut usize, 
        memo: &mut HashSet<StateHash>) {       
            // dbg_print(steps, level, &facility);
            
            let hash = get_hash(&facility, level);
            if memo.contains(&hash) { return; }
            
            let last_idx = facility.len() - 1;
            if last_idx == level && is_level_completed(&facility[last_idx]) {
                *backtrack = steps.min(*backtrack);
                println!("{backtrack}");            
                return;
            }
            
        if steps < *backtrack {
            memo.insert(hash);
            let adjacent = adjacent_levels(level, last_idx);
            let possible_idx = possible_comp_idx(&facility, level);
            for pos in possible_idx {
                for adj in &adjacent {
                    if let Some(next) = transfer(&facility, level, *adj, &pos) {
                        perform(next, *adj, steps + 1, backtrack, memo);                                                    
                    }
                }
            }
        }
    }

    fn possible_comp_idx(facility: &Facility, level: usize) -> Vec<Vec<usize>> {
        let mut result = Vec::new();
        let len = facility.len();
        for a in 0..len - 1 {
            if !facility[level][a] {
                continue;
            }
            result.push(vec![a]);
            for b in a + 1..len {
                if !facility[level][b] {
                    continue;
                }
                result.push(vec![a, b]);
            }
        }
        result
    }

    fn transfer(facility: &Facility, from: usize, to: usize, position: &[usize]) -> Option<Facility> {
        let mut next = facility.clone();
        for i in position {
            next[from][*i] = !next[from][*i];
            next[to][*i] = !next[to][*i];
        }
        if is_valid(&next) {
            Some(next)
        } else {
            None
        }
    }

    fn adjacent_levels(level: usize, last_level: usize) -> Vec<usize> {
        let mut levels = Vec::with_capacity(2);
        if level > 0 {
            levels.push(level - 1);
        }
        if level < last_level {
            levels.push(level + 1);
        }
        levels
    }

    fn get_hash(facility: &Facility, level: usize) -> StateHash {
        let mut hasher = DefaultHasher::new();
        facility.hash(&mut hasher);
        // TODO: not sure...
        level.hash(&mut hasher);
        hasher.finish()
    }

    fn is_level_completed(level: &Level) -> bool {
        level.iter().map(|v| *v as usize).sum::<usize>() == level.len()
    }

    fn is_valid(facility: &Facility) -> bool {
        for level in facility {
            if !is_valid_level(level) {
                return false;
            }
        }
        true
    }

    fn is_valid_level(level: &Level) -> bool {
        let len = level.len();
        let has_generators = (0..len).step_by(2)
            .filter(|v| level[*v])
            .count() > 0;
        for i in (1..len).step_by(2) {
            if !level[i] {
                continue;
            } else if !level[i - 1] && has_generators {
                return false;
            }
        }
        true
    }

    let mut output = usize::MAX;
    perform(facility.clone(), 0, 0, &mut output, &mut HashSet::new());
    output
}


pub struct AoC2016_11 {
    input: Facility
}

impl AoC2016_11 {
    pub fn new() -> io::Result<Self> {
        // The first floor contains a thulium generator, a thulium-compatible microchip, a plutonium generator, and a strontium generator.
        // The second floor contains a plutonium-compatible microchip and a strontium-compatible microchip.
        // The third floor contains a promethium generator, a promethium-compatible microchip, a ruthenium generator, and a ruthenium-compatible microchip.
        // The fourth floor contains nothing relevant.
        let input = vec![
            //   |T |  |P |  |S |  |Pr|  |Rut
            vec![T, T, T, F, T, F, F, F, F, F],
            vec![F, F, F, T, F, T, F, F, F, F],
            vec![F, F, F, F, F, F, T, T, T, T],
            vec![F, F, F, F, F, F, F, F, F, F],
        ];

        let facility = vec![
            vec![F, T, F, T],
            vec![T, F, F, F],
            vec![F, F, T, F],
            vec![F, F, F, F],
        ];
        Ok(Self {
            input: facility,
        })
    }
}

impl Solution for AoC2016_11 {
    fn part_one(&self) -> String {
        min_steps(&self.input.clone())
            .to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2016/Day 11: Radioisotope Thermoelectric Generators".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_11_correctness() -> io::Result<()> {
        let sol = AoC2016_11::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    // #[test]
    // fn aoc2016_11_demo() {
    //     let mut facility = vec![
    //         vec![F, T, F, T],
    //         vec![T, F, F, F],
    //         vec![F, F, T, F],
    //         vec![F, F, F, F],
    //     ];
    //     let mut output = usize::MAX;
    //     min_steps(&mut facility, 0, 0, &mut output);
    //     assert_eq!(output, 11);
    // }
}