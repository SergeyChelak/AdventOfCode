use crate::solution::Solution;

use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    hash::{Hash, Hasher},
    io,
};

// generator
const G: bool = true;
// microchip
const M: bool = true;
// open cell
const O: bool = false;

type Level = Vec<bool>;
type Facility = Vec<Level>;
type StateHash = u64;

#[derive(Hash)]
struct State {
    facility: Facility,
    level: usize,
}

impl State {
    fn new(facility: &Facility, level: usize) -> Self {
        Self {
            facility: facility.clone(),
            level,
        }
    }

    fn is_valid(&self) -> bool {
        for row in &self.facility {
            if !Self::is_valid_level(row) {
                return false;
            }
        }
        true
    }

    #[allow(clippy::needless_range_loop)]
    fn all_movable_indices(&self) -> Vec<Vec<usize>> {
        let mut result = Vec::new();
        let level = &self.facility[self.level];
        let len = level.len();
        for a in 0..len {
            if !level[a] {
                continue;
            }
            result.push(vec![a]);
            for b in a + 1..len {
                if !level[b] {
                    continue;
                }
                result.push(vec![a, b]);
            }
        }
        result
    }

    fn adjacent_levels(&self) -> Vec<usize> {
        let mut adjacent = Vec::with_capacity(2);
        if self.level > 0 {
            adjacent.push(self.level - 1);
        }
        if self.level < self.facility.len() - 1 {
            adjacent.push(self.level + 1);
        }
        adjacent
    }

    fn is_completed(&self) -> bool {
        if self.level != self.facility.len() - 1 {
            false
        } else {
            let lvl = &self.facility[self.level];
            lvl.iter().map(|v| *v as usize).sum::<usize>() == lvl.len()
        }
    }

    fn with_move(&self, next_level: usize, position: &[usize]) -> Option<State> {
        let mut next = self.facility.clone();
        for i in position {
            next[self.level][*i] = !next[self.level][*i];
            next[next_level][*i] = !next[next_level][*i];
        }
        let state = State::new(&next, next_level);
        if state.is_valid() {
            Some(state)
        } else {
            None
        }
    }

    fn is_valid_level(level: &Level) -> bool {
        let len = level.len();
        let has_generators = (0..len).step_by(2).filter(|v| level[*v]).count() > 0;
        for i in (1..len).step_by(2) {
            if !level[i] {
                continue;
            } else if !level[i - 1] && has_generators {
                return false;
            }
        }
        true
    }

    fn hash(&self) -> StateHash {
        let mut hasher = DefaultHasher::new();
        self.facility.hash(&mut hasher);
        self.level.hash(&mut hasher);
        hasher.finish()
    }
}

fn min_steps(facility: &Facility) -> usize {
    let mut visited: HashSet<StateHash> = HashSet::new();
    let mut step = 0;
    let mut states = vec![State::new(facility, 0)];
    'ml: while !states.is_empty() {
        let mut next_states = Vec::new();
        for state in &states {
            if state.is_completed() {
                break 'ml;
            }
            let hash = state.hash();
            if visited.contains(&hash) {
                continue;
            }
            visited.insert(hash);
            for pos in state.all_movable_indices() {
                for level in state.adjacent_levels() {
                    if let Some(next) = state.with_move(level, &pos) {
                        next_states.push(next);
                    }
                }
            }
        }
        step += 1;
        states = next_states;
    }
    step
}

pub struct AoC2016_11;

impl AoC2016_11 {
    pub fn new() -> io::Result<Self> {
        Ok(Self)
    }
}

impl Solution for AoC2016_11 {
    fn part_one(&self) -> String {
        let input = vec![
            vec![G, M, G, O, G, O, O, O, O, O],
            vec![O, O, O, M, O, M, O, O, O, O],
            vec![O, O, O, O, O, O, G, M, G, M],
            vec![O, O, O, O, O, O, O, O, O, O],
        ];
        min_steps(&input).to_string()
    }

    fn part_two(&self) -> String {
        let input = vec![
            vec![G, M, G, O, G, O, O, O, O, O, G, M, G, M],
            vec![O, O, O, M, O, M, O, O, O, O, O, O, O, O],
            vec![O, O, O, O, O, O, G, M, G, M, O, O, O, O],
            vec![O, O, O, O, O, O, O, O, O, O, O, O, O, O],
        ];
        min_steps(&input).to_string()
    }

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
        assert_eq!(sol.part_one(), "31");
        assert_eq!(sol.part_two(), "55");
        Ok(())
    }
}
