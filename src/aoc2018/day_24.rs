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
enum Army {
    Immune,
    Infection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Group {
    units: usize,
    hp: usize,
    weaknesses: AttackTypeMask,
    immunities: AttackTypeMask,
    attack_type: AttackType,
    damage: usize,
    initiative: usize,
    army: Army,
}

impl Group {
    fn effective_power(&self) -> usize {
        self.units * self.damage
    }

    fn damage_value_for(&self, other: &Self) -> usize {
        if other.units == 0 {
            return 0;
        }
        if other.army == self.army {
            return 0;
        }
        if other.immunities & self.attack_type > 0 {
            return 0;
        }
        let mut damage = self.effective_power();
        if other.weaknesses & self.attack_type > 0 {
            damage *= 2;
        }
        damage
    }

    fn defend(&mut self, other: &Self) {
        let received_damage = other.damage_value_for(self);
        self.units = self.units.saturating_sub(received_damage / self.hp);
    }
}

fn combat(groups: &mut [Group]) -> Option<(Army, usize)> {
    let (mut immune_prev, mut infection_prev) = (0, 0);
    loop {
        let immune_units = total_units(groups, |g| g.army == Army::Immune);
        let infection_units = total_units(groups, |g| g.army == Army::Infection);
        if (immune_prev, infection_prev) == (immune_units, infection_units) {
            // stuck
            break None;
        }
        if immune_units * infection_units == 0 {
            if immune_units > 0 {
                break Some((Army::Immune, immune_units));
            }
            if infection_units > 0 {
                break Some((Army::Infection, infection_units));
            }
            break None;
        }
        combat_round(groups);
        immune_prev = immune_units;
        infection_prev = infection_units;
    }
}

fn total_units(groups: &[Group], f: impl Fn(&Group) -> bool) -> usize {
    groups.iter().filter(|g| f(g)).map(|g| g.units).sum()
}

fn combat_round(groups: &mut [Group]) {
    let targets = target_selection(groups);
    let order = attack_order(groups);
    for idx in order {
        let Some(defend_idx) = targets[idx] else {
            continue;
        };
        let attack_group = groups[idx];
        let defend_group = groups.get_mut(defend_idx).unwrap();
        defend_group.defend(&attack_group);
    }
}

fn target_selection(groups: &[Group]) -> Vec<Option<usize>> {
    let mut selection = vec![None; groups.len()];
    let mut chosen = vec![false; groups.len()];
    let order = effective_power_order(groups);
    for idx in order {
        let group = groups[idx];
        let target = groups
            .iter()
            .enumerate()
            .filter(|(i, g)| !chosen[*i] && g.units > 0)
            .map(|(i, g)| {
                (
                    i,
                    group.damage_value_for(g),
                    g.effective_power(),
                    g.initiative,
                )
            })
            .filter(|(_, dmg, _, _)| *dmg > 0)
            .max_by(|a, b| {
                let (_, a_dmg, a_ep, a_initiative) = a;
                let (_, b_dmg, b_ep, b_initiative) = b;
                a_dmg
                    .cmp(b_dmg)
                    .then(a_ep.cmp(b_ep))
                    .then(a_initiative.cmp(b_initiative))
            })
            .map(|(idx, _, _, _)| idx);
        selection[idx] = target;
        if let Some(target_idx) = target {
            chosen[target_idx] = true
        }
    }
    selection
}

fn effective_power_order(groups: &[Group]) -> Vec<usize> {
    let mut arr = groups.iter().enumerate().collect::<Vec<(usize, &Group)>>();
    arr.sort_by(|a, b| {
        b.1.effective_power()
            .cmp(&a.1.effective_power())
            .then(b.1.initiative.cmp(&a.1.initiative))
    });
    arr.iter().map(|x| x.0).collect()
}

fn attack_order(groups: &[Group]) -> Vec<usize> {
    let mut arr = groups.iter().enumerate().collect::<Vec<(usize, &Group)>>();
    arr.sort_by(|a, b| b.1.initiative.cmp(&a.1.initiative));
    arr.iter().map(|x| x.0).collect()
}

fn boosted(groups: &[Group], value: usize) -> Vec<Group> {
    let mut result = groups.to_owned();
    result.iter_mut().for_each(|g| {
        if g.army != Army::Immune {
            return;
        }
        g.damage += value;
    });
    result
}

pub struct AoC2018_24 {
    groups: Vec<Group>,
}

impl AoC2018_24 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2018_24")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines(lines: &[String]) -> Self {
        let parser = Parser::new().expect("Failed to create parser");
        let groups = parser.parse(lines);
        Self { groups }
    }
}

impl Solution for AoC2018_24 {
    fn part_one(&self) -> String {
        let Some((_, units)) = combat(&mut self.groups.clone()) else {
            return "Stuck/Mutual destruction".to_string();
        };
        units.to_string()
    }

    fn part_two(&self) -> String {
        let (mut left, mut right) = (0, 10_000);
        let mut result: Option<usize> = None;
        while left <= right {
            let mid = (left + right) >> 1;
            let mut groups = boosted(&self.groups, mid);
            let Some((army, units)) = combat(&mut groups) else {
                left = mid + 1;
                continue;
            };
            match army {
                Army::Immune => {
                    result = Some(units);
                    right = mid - 1;
                }
                Army::Infection => {
                    left = mid + 1;
                }
            }
        }
        result.unwrap().to_string()
    }

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

    fn parse(&self, lines: &[String]) -> Vec<Group> {
        let mut current_army = Army::Immune;
        let mut result = Vec::new();
        for line in lines {
            if line.is_empty() {
                continue;
            }
            if line == "Immune System:" {
                current_army = Army::Immune;
                continue;
            }
            if line == "Infection:" {
                current_army = Army::Infection;
                continue;
            }
            let item = self
                .parse_group(line, current_army)
                .expect("Failed to parse group");
            result.push(item);
        }
        result
    }

    fn parse_group(&self, inp: &str, army: Army) -> Option<Group> {
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
            army,
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
            .map(Self::parse_attack_type)
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
        assert!(!sol.groups.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2018_24_parser_long_input() {
        let parser = Parser::new().unwrap();
        let s = "2016 units each with 10188 hit points (weak to slashing, radiation; immune to cold, bludgeoning) with an attack that does 47 bludgeoning damage at initiative 14";
        let group = parser.parse_group(s, Army::Immune).unwrap();
        let test = Group {
            units: 2016,
            hp: 10188,
            weaknesses: ATTACK_TYPE_SLASHING | ATTACK_TYPE_RADIATION,
            immunities: ATTACK_TYPE_COLD | ATTACK_TYPE_BLUDGEONING,
            damage: 47,
            attack_type: ATTACK_TYPE_BLUDGEONING,
            initiative: 14,
            army: Army::Immune,
        };
        assert_eq!(test, group)
    }

    #[test]
    fn aoc2018_24_parser_short_input() {
        let parser = Parser::new().unwrap();
        let s = "4154 units each with 3839 hit points with an attack that does 9 slashing damage at initiative 7";
        let group = parser.parse_group(s, Army::Immune).unwrap();
        let test = Group {
            units: 4154,
            hp: 3839,
            weaknesses: 0,
            immunities: 0,
            damage: 9,
            attack_type: ATTACK_TYPE_SLASHING,
            initiative: 7,
            army: Army::Immune,
        };
        assert_eq!(test, group)
    }

    #[test]
    fn aoc2018_24_parser_ex_1_im_1() {
        let s = "17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2";
        let parser = Parser::new().unwrap();
        let group = parser.parse_group(s, Army::Immune).unwrap();
        let test = Group {
            units: 17,
            hp: 5390,
            weaknesses: ATTACK_TYPE_RADIATION | ATTACK_TYPE_BLUDGEONING,
            immunities: 0,
            damage: 4507,
            attack_type: ATTACK_TYPE_FIRE,
            initiative: 2,
            army: Army::Immune,
        };
        assert_eq!(test, group)
    }

    #[test]
    fn aoc2018_24_parser_ex_1_im_2() {
        let s = "989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3";
        let parser = Parser::new().unwrap();
        let group = parser.parse_group(s, Army::Immune).unwrap();
        let test = Group {
            units: 989,
            hp: 1274,
            weaknesses: ATTACK_TYPE_BLUDGEONING | ATTACK_TYPE_SLASHING,
            immunities: ATTACK_TYPE_FIRE,
            damage: 25,
            attack_type: ATTACK_TYPE_SLASHING,
            initiative: 3,
            army: Army::Immune,
        };
        assert_eq!(test, group)
    }

    #[test]
    fn aoc2018_24_parser_ex_1_inf_1() {
        let s = "801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1";
        let parser = Parser::new().unwrap();
        let group = parser.parse_group(s, Army::Infection).unwrap();
        let test = Group {
            units: 801,
            hp: 4706,
            weaknesses: ATTACK_TYPE_RADIATION,
            immunities: 0,
            damage: 116,
            attack_type: ATTACK_TYPE_BLUDGEONING,
            initiative: 1,
            army: Army::Infection,
        };
        assert_eq!(test, group)
    }

    #[test]
    fn aoc2018_24_parser_ex_1_inf_2() {
        let s = "4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
        let parser = Parser::new().unwrap();
        let group = parser.parse_group(s, Army::Infection).unwrap();
        let test = Group {
            units: 4485,
            hp: 2961,
            weaknesses: ATTACK_TYPE_FIRE | ATTACK_TYPE_COLD,
            immunities: ATTACK_TYPE_RADIATION,
            damage: 12,
            attack_type: ATTACK_TYPE_SLASHING,
            initiative: 4,
            army: Army::Infection,
        };
        assert_eq!(test, group)
    }

    #[test]
    fn aoc2018_24_damage_im2_inf2() {
        let parser = Parser::new().unwrap();

        let imm = {
            let s = "989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3";
            parser.parse_group(s, Army::Immune).unwrap()
        };

        let inf = {
            let s = "4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
            parser.parse_group(s, Army::Infection).unwrap()
        };

        assert_eq!(inf.damage_value_for(&imm), 107640);
    }

    #[test]
    fn aoc2018_24_ex1() {
        let puzzle = example_puzzle();
        assert_eq!(puzzle.part_one(), "5216")
    }

    #[test]
    fn aoc2018_24_ex2() {
        let puzzle = example_puzzle();
        assert_eq!(puzzle.part_two(), "51")
    }

    fn example_puzzle() -> AoC2018_24 {
        let inp = [
            "Immune System:",
            "17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2",
            "989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3",
            "",
            "Infection:",
            "801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1",
            "4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4",
        ].iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
        AoC2018_24::with_lines(&inp)
    }

    #[test]
    fn aoc2018_24_correctness() -> io::Result<()> {
        let sol = AoC2018_24::new()?;
        assert_eq!(sol.part_one(), "24009");
        assert_eq!(sol.part_two(), "379");
        Ok(())
    }
}
