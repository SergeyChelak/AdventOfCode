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

trait Validable {
    fn is_valid(&self) -> bool;
}

impl Validable for Level {
    fn is_valid(&self) -> bool {
        let len = self.len();
        if (0..len).step_by(2).filter(|v| self[*v]).count() == 0 {
            return true;
        }
        for i in (1..len).step_by(2) {
            if self[i] && !self[i - 1] {
                return false;
            }
        }
        true
    }
}

fn copy(src: &Level, dest: &mut Level) {
    for (place, data) in dest.iter_mut().zip(src.iter()) {
        *place = *data
    }
}

type Facility = Vec<Level>;
type StateHash = u64;

#[derive(Hash)]
struct State {
    facility: Facility,
    level: usize,
    _buf1: Level,
    _buf2: Level,
}

impl State {
    fn new(facility: Facility, level: usize) -> Self {
        let buf_size = facility[0].len();
        Self {
            facility,
            level,
            _buf1: vec![O; buf_size],
            _buf2: vec![O; buf_size],
        }
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

    fn with_move(&mut self, next_level: usize, position: &[usize]) -> Option<State> {
        copy(&self.facility[self.level], &mut self._buf1);
        copy(&self.facility[next_level], &mut self._buf2);

        for i in position {
            self._buf1[*i] = !self._buf1[*i];
            self._buf2[*i] = !self._buf2[*i];
        }

        if self._buf1.is_valid() && self._buf2.is_valid() {
            let mut next = self.facility.clone();
            copy(&self._buf1, &mut next[self.level]);
            copy(&self._buf2, &mut next[next_level]);
            Some(State::new(next, next_level))
        } else {
            None
        }
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
    let mut states = vec![State::new(facility.clone(), 0)];
    'ml: while !states.is_empty() {
        let mut next_states = Vec::with_capacity(2 * states.len());
        step += 1;
        for state in states.iter_mut() {
            for pos in state.all_movable_indices() {
                for level in state.adjacent_levels() {
                    if let Some(next) = state.with_move(level, &pos) {
                        let hash = next.hash();
                        if visited.contains(&hash) {
                            continue;
                        } else if next.is_completed() {
                            break 'ml;
                        }
                        visited.insert(hash);
                        next_states.push(next);
                    }
                }
            }
        }
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
