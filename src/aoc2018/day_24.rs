use regex::Regex;

use crate::solution::Solution;
use crate::utils::*;

use std::io;

type AttackTypeMask = u16;
type AttackType = u16;

const ATTACK_TYPE_FIRE: AttackTypeMask = 1 << 0;
const ATTACK_TYPE_RADIATION: AttackTypeMask = 1 << 1;
const ATTACK_TYPE_COLD: AttackTypeMask = 1 << 2;
const ATTACK_TYPE_BLUDGEONING: AttackTypeMask = 1 << 3;
const ATTACK_TYPE_SLASHING: AttackTypeMask = 1 << 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Group {
    units: usize,
    hp: usize,
    weaknesses: AttackTypeMask,
    immunities: AttackTypeMask,
    attack_type: AttackType,
    damage: usize,
    initiative: usize,
}

struct Army {
    groups: Vec<Group>,
}

impl Army {
    fn new() -> Self {
        Self {
            groups: Default::default(),
        }
    }

    fn push(&mut self, group: Group) {
        self.groups.push(group)
    }

    fn total_units(&self) -> usize {
        self.groups.iter().map(|x| x.units).sum()
    }
}

pub struct AoC2018_24 {
    immune: Army,
    infection: Army,
}

impl AoC2018_24 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2018_24")?;
        let parser = Parser::new().expect("Failed to create parser");
        let (immune, infection) = parser.parse(&lines);
        Ok(Self { immune, infection })
    }
}

impl Solution for AoC2018_24 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2018/Day 24: Immune System Simulator 20XX".to_string()
    }
}

struct Parser {
    regex: Regex,
}

impl Parser {
    fn new() -> Option<Self> {
        let Ok(regex) = Regex::new(
            r"(\d+) units each with (\d+) hit points (\([^)]+\) )?with an attack that does (\d+) ([^)]+) damage at initiative (\d+)",
        ) else {
            return None;
        };
        Some(Self { regex })
    }

    fn parse(&self, lines: &[String]) -> (Army, Army) {
        let mut immune_army = Army::new();
        let mut infection_army = Army::new();
        let mut arr = &mut immune_army;
        for line in lines {
            if line.is_empty() {
                continue;
            }
            if line == "Immune System:" {
                arr = &mut immune_army;
                continue;
            }
            if line == "Infection:" {
                arr = &mut infection_army;
                continue;
            }
            let item = self.parse_group(line).expect("Failed to parse group");
            arr.push(item);
        }
        (immune_army, infection_army)
    }

    fn parse_group(&self, inp: &str) -> Option<Group> {
        let captures = self.regex.captures(inp)?;
        if captures.len() != 7 {
            return None;
        }
        let units = captures.get(1).map(|x| x.as_str().parse::<usize>())?.ok()?;
        let hp = captures.get(2).map(|x| x.as_str().parse::<usize>())?.ok()?;
        let (weaknesses, immunities) = captures
            .get(3)
            .map(|x| Self::parse_traits(x.as_str()))
            .unwrap_or((0, 0));
        let damage = captures.get(4).map(|x| x.as_str().parse::<usize>())?.ok()?;
        let attack_type = captures
            .get(5)
            .map(|x| Self::parse_attack_type(x.as_str()))?;
        let initiative = captures.get(6).map(|x| x.as_str().parse::<usize>())?.ok()?;
        let group = Group {
            units,
            hp,
            weaknesses,
            immunities,
            damage,
            attack_type,
            initiative,
        };
        Some(group)
    }

    fn parse_traits(traits: &str) -> (AttackTypeMask, AttackTypeMask) {
        let mut weaknesses: AttackTypeMask = 0;
        let mut immunities: AttackTypeMask = 0;

        let split = &traits[1..traits.len() - 2]
            .split("; ")
            .collect::<Vec<&str>>();
        for s in split {
            if let Some((_, tokens)) = s.split_once("weak to ") {
                weaknesses = Self::parse_attack_tokens(tokens);
            }
            if let Some((_, tokens)) = s.split_once("immune to ") {
                immunities = Self::parse_attack_tokens(tokens);
            }
        }
        (weaknesses, immunities)
    }

    fn parse_attack_tokens(tokens: &str) -> AttackTypeMask {
        tokens
            .split(", ")
            .map(|s| Self::parse_attack_type(s))
            .fold(0, |acc, x| acc | x)
    }

    fn parse_attack_type(s: &str) -> AttackType {
        match s {
            "fire" => ATTACK_TYPE_FIRE,
            "radiation" => ATTACK_TYPE_RADIATION,
            "cold" => ATTACK_TYPE_COLD,
            "bludgeoning" => ATTACK_TYPE_BLUDGEONING,
            "slashing" => ATTACK_TYPE_SLASHING,
            _ => panic!("Unexpected type {}", s),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_24_input_load_test() -> io::Result<()> {
        let sol = AoC2018_24::new()?;
        assert!(!sol.immune.groups.is_empty());
        assert!(!sol.infection.groups.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_24_parser_long_input() {
        let parser = Parser::new().unwrap();
        let s = "2016 units each with 10188 hit points (weak to slashing, radiation; immune to cold, bludgeoning) with an attack that does 47 bludgeoning damage at initiative 14";
        let group = parser.parse_group(s).unwrap();
        let test = Group {
            units: 2016,
            hp: 10188,
            weaknesses: ATTACK_TYPE_SLASHING | ATTACK_TYPE_RADIATION,
            immunities: ATTACK_TYPE_COLD | ATTACK_TYPE_BLUDGEONING,
            damage: 47,
            attack_type: ATTACK_TYPE_BLUDGEONING,
            initiative: 14,
        };
        assert_eq!(test, group)
    }

    #[test]
    fn aoc2018_24_parser_short_input() {
        let parser = Parser::new().unwrap();
        let s = "4154 units each with 3839 hit points with an attack that does 9 slashing damage at initiative 7";
        let group = parser.parse_group(s).unwrap();
        let test = Group {
            units: 4154,
            hp: 3839,
            weaknesses: 0,
            immunities: 0,
            damage: 9,
            attack_type: ATTACK_TYPE_SLASHING,
            initiative: 7,
        };
        assert_eq!(test, group)
    }

    #[test]
    fn aoc2018_24_correctness() -> io::Result<()> {
        let sol = AoC2018_24::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }
}
