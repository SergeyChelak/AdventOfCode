use crate::solution::Solution;

use std::io;

struct Player {
    hit_points: i32,
    damage: i32,
    armor: i32,
    mana: i32,
}

impl Player {
    fn boss() -> Self {
        // Hit Points: 58
        // Damage: 9
        todo!()
    }

    fn player() -> Self {
        Self {
            hit_points: 50,
            damage: 0,
            armor: 0,
            mana: 500,
        }
    }

    fn make_turn(&self, other: &mut Self) {
        //
    }
}

pub struct AoC2015_22 {
    // place required fields here
}

impl AoC2015_22 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            // initialize solution
        })
    }
}

impl Solution for AoC2015_22 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2015/Day 22: Wizard Simulator 20XX".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_22_correctness() -> io::Result<()> {
        let sol = AoC2015_22::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}