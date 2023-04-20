use crate::solution::Solution;

use std::{
    io, 
    collections::{HashMap, hash_map::DefaultHasher},
    hash::{Hash, Hasher}
};

const T: bool = true;
const F: bool = false;

type Level = Vec<bool>;
type Facility = Vec<Level>;
type FacilityHash = u64;

fn min_steps(facility: &mut Facility, level: usize, steps: usize, output: &mut usize, memo: &mut HashMap<FacilityHash, usize>) {
    let last_idx = facility.len() - 1;
    if level == last_idx && is_level_completed(&facility[last_idx]) {
        *output = steps.min(*output);
    } else {
        // if steps >= *output {
        //     return;
        // }
        let hash = get_hash(facility);
        if let Some(prev_steps) = memo.get(&hash) {
            if *prev_steps <= steps {
                return;
            }
        }
        memo.insert(hash, steps);
        let positions = safe_indices(&mut facility[level]);
        // println!("{:?}", positions);
        let allowed_levels = if level == 0 {
            vec![1]
        } else if level == last_idx {
            vec![level - 1]
        } else {
            vec![level - 1, level + 1]
        };
        for lvl in allowed_levels {
            for pos in &positions {
                if !is_safe_move(pos, &mut facility[level]) {
                    continue;
                }
                apply_position(pos, &mut facility[level]);
                apply_position(pos, &mut facility[lvl]);
                min_steps(facility, lvl, steps + 1, output, memo);
                // revert
                apply_position(pos, &mut facility[level]);
                apply_position(pos, &mut facility[lvl]);
            }
        }
    }
}

fn get_hash(facility: &Facility) -> FacilityHash {
    let mut hasher = DefaultHasher::new();
    facility.hash(&mut hasher);
    hasher.finish()
}

fn safe_indices(level: &mut Level) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let len = level.len();
    // singles
    for i in 0..len {
        if level[i] {            
            result.push(vec![i]);
        }
    }
    // pairs
    for i in 0..len - 1 {
        for j in i + 1..len {
            if !(level[i] && level[j]) { continue; }
            let mod_i = i % 2;
            let mod_j = j % 2;
            // 2 generators or 2 chips or generator + compatible chip
            let is_safe = mod_i == mod_j || mod_i == 0 && j - i == 1;
            if is_safe {
                result.push(vec![i, j]);
            }
        }
    }
    result.into_iter()
        .filter(|pos| is_safe_move(pos, level))
        .collect()
}

fn is_level_completed(level: &Level) -> bool {
    level.iter().map(|v| *v as usize).sum::<usize>() == level.len()
}

fn is_safe_move(position: &[usize], level: &mut Level) -> bool {
    apply_position(position, level);
    let is_safe = is_safe_level(level);
    apply_position(position, level);
    is_safe
}

fn is_safe_level(level: &Level) -> bool {
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

fn apply_position(position: &[usize], level: &mut Level) {
    position.iter().for_each(|&i| level[i] = !level[i]);
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

        let mut facility = vec![
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
        let mut facility = self.input.clone();
        let mut output = usize::MAX;
        min_steps(&mut facility, 0, 0, &mut output, &mut HashMap::new());
        output.to_string()
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