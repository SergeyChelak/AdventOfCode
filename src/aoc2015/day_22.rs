use crate::solution::Solution;

use std::io;

#[derive(Clone, Copy)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn all_cases() -> Vec<Spell> {
        vec![
            Self::MagicMissile,
            Self::Drain,
            Self::Shield,
            Self::Poison,
            Self::Recharge,
        ]
    }

    fn cost(&self) -> i32 {
        match self {
            Self::MagicMissile => 53,
            Self::Drain => 73,
            Self::Shield => 113,
            Self::Poison => 173,
            Self::Recharge => 229,
        }
    }

    fn effect_duration(&self) -> i32 {
        match self {
            Self::MagicMissile => 0,
            Self::Drain => 0,
            Self::Shield => 6,
            Self::Poison => 6,
            Self::Recharge => 5,
        }
    }

    fn has_effect(&self) -> bool {
        self.effect_duration() > 0
    }
}

enum Aftermath {
    Win,
    Lose,
    InsufficientSpells,
}

struct Wizard {
    hit_points: i32,
    armor: i32,
    mana: i32,
}

impl Wizard {
    fn new() -> Self {
        Self {
            hit_points: 50,
            armor: 0,
            mana: 500,
        }
    }

    fn is_alive(&self) -> bool {
        self.hit_points > 0 && self.mana > 0
    }
}

struct Boss {
    hit_points: i32,
    damage: i32,
}

impl Boss {
    fn new() -> Self {
        Self {
            hit_points: 58,
            damage: 9,
        }
    }

    fn is_alive(&self) -> bool {
        self.hit_points > 0
    }
}

struct Battlefield {
    wizard: Wizard,
    boss: Boss,
    spells: Vec<Spell>,
    timer_shield: Option<i32>,
    timer_poison: Option<i32>,
    timer_recharge: Option<i32>,
    is_hard: bool,
}

impl Battlefield {
    pub fn with_spells(spells: &[Spell], is_hard: bool) -> Self {
        Self::new(Wizard::new(), Boss::new(), spells, is_hard)
    }

    fn new(wizard: Wizard, boss: Boss, spells: &[Spell], is_hard: bool) -> Self {
        let spells = spells.iter().rev().copied().collect::<Vec<Spell>>();
        Self {
            wizard,
            boss,
            spells,
            timer_shield: None,
            timer_poison: None,
            timer_recharge: None,
            is_hard,
        }
    }

    fn battle(&mut self) -> Aftermath {
        let mut is_wizard_move = true;
        while self.wizard.is_alive() && self.boss.is_alive() {
            if self.is_hard {
                self.wizard.hit_points -= 1;
                if !self.wizard.is_alive() {
                    break;
                }
            }
            self.cast_effects();
            if is_wizard_move {
                let spell = self.spells.pop();
                if spell.is_none() {
                    return Aftermath::InsufficientSpells;
                }
                let spell = spell.unwrap();
                if spell.cost() > self.wizard.mana {
                    return Aftermath::Lose;
                }
                self.wizard.mana -= spell.cost();
                if spell.has_effect() {
                    if !self.try_effect(&spell) {
                        return Aftermath::Lose;
                    }
                } else {
                    match spell {
                        Spell::MagicMissile => {
                            self.boss.hit_points -= 4;
                        }
                        Spell::Drain => {
                            self.boss.hit_points -= 2;
                            self.wizard.hit_points += 2;
                        }
                        _ => panic!("Unexpected spell without effect"),
                    }
                }
            } else {
                let damage = (self.boss.damage - self.wizard.armor).max(1);
                self.wizard.hit_points -= damage;
            }
            is_wizard_move = !is_wizard_move;
        }
        if self.wizard.is_alive() && self.spells.is_empty() {
            Aftermath::Win
        } else {
            Aftermath::Lose
        }
    }

    fn cast_effects(&mut self) {
        self.cast_poison();
        self.cast_recharge();
        self.cast_shield();
    }

    fn cast_poison(&mut self) {
        if let Some(val) = self.timer_poison {
            self.timer_poison = if val == 0 {
                None
            } else {
                self.boss.hit_points -= 3;
                Some(val - 1)
            }
        }
    }

    fn cast_recharge(&mut self) {
        if let Some(val) = self.timer_recharge {
            self.timer_recharge = if val == 0 {
                None
            } else {
                self.wizard.mana += 101;
                Some(val - 1)
            }
        }
    }

    fn cast_shield(&mut self) {
        if let Some(val) = self.timer_shield {
            self.timer_shield = if val == 0 {
                self.wizard.armor -= 7;
                None
            } else {
                if val == Spell::Shield.effect_duration() {
                    self.wizard.armor += 7;
                }
                Some(val - 1)
            }
        }
    }

    fn try_effect(&mut self, spell: &Spell) -> bool {
        match spell {
            Spell::Poison => {
                if self.timer_poison.is_none() {
                    self.timer_poison = Some(spell.effect_duration());
                    self.cast_poison();
                    true
                } else {
                    false
                }
            }
            Spell::Recharge => {
                if self.timer_recharge.is_none() {
                    self.timer_recharge = Some(spell.effect_duration());
                    self.cast_recharge();
                    true
                } else {
                    false
                }
            }
            Spell::Shield => {
                if self.timer_shield.is_none() {
                    self.timer_shield = Some(spell.effect_duration());
                    self.cast_shield();
                    true
                } else {
                    false
                }
            }
            _ => panic!("Unexpected effect type"),
        }
    }
}

fn calc_min_mana_amount(is_hard: bool, cost: i32, spells: &mut Vec<Spell>, result: &mut i32) {
    if cost >= *result {
        return;
    }
    for spell in Spell::all_cases() {
        let new_cost = cost + spell.cost();
        spells.push(spell);
        match Battlefield::with_spells(spells, is_hard).battle() {
            Aftermath::Win => *result = new_cost.min(*result),
            Aftermath::InsufficientSpells => {
                calc_min_mana_amount(is_hard, new_cost, spells, result)
            }
            _ => (),
        }
        spells.pop();
    }
}

pub struct AoC2015_22;

impl AoC2015_22 {
    pub fn new() -> io::Result<Self> {
        Ok(Self)
    }
}

impl Solution for AoC2015_22 {
    fn part_one(&self) -> String {
        let mut result = i32::MAX;
        calc_min_mana_amount(false, 0, &mut Vec::new(), &mut result);
        result.to_string()
    }

    fn part_two(&self) -> String {
        let mut result = i32::MAX;
        calc_min_mana_amount(true, 0, &mut Vec::new(), &mut result);
        result.to_string()
    }

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
        assert_eq!(sol.part_one(), "1269");
        assert_eq!(sol.part_two(), "1309");
        Ok(())
    }

    #[test]
    fn aoc2015_22_battle_test1() {
        let wizard = Wizard {
            hit_points: 10,
            armor: 0,
            mana: 250,
        };

        let boss = Boss {
            hit_points: 13,
            damage: 8,
        };
        let mut bf = Battlefield::new(wizard, boss, &[Spell::Poison, Spell::MagicMissile], false);
        bf.battle();
        assert!(!bf.boss.is_alive());
        assert_eq!(bf.wizard.hit_points, 2);
        assert_eq!(bf.wizard.armor, 0);
        assert_eq!(bf.wizard.mana, 24);
    }

    #[test]
    fn aoc2015_22_battle_test2() {
        let wizard = Wizard {
            hit_points: 10,
            armor: 0,
            mana: 250,
        };

        let boss = Boss {
            hit_points: 14,
            damage: 8,
        };
        let spells = vec![
            Spell::Recharge,
            Spell::Shield,
            Spell::Drain,
            Spell::Poison,
            Spell::MagicMissile,
        ];
        let mut bf = Battlefield::new(wizard, boss, &spells, false);
        bf.battle();
        assert!(!bf.boss.is_alive());
        assert_eq!(bf.wizard.hit_points, 1);
        assert_eq!(bf.wizard.armor, 0);
        assert_eq!(bf.wizard.mana, 114);
    }
}
