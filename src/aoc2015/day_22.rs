use crate::solution::Solution;

use std::{io, collections::HashMap};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge
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

    fn initial_duration(&self) -> i32 {
        match self {
            Self::MagicMissile => 1,
            Self::Drain => 1,
            Self::Shield => 6,
            Self::Poison => 6,
            Self::Recharge => 5,
        }
    }
}

struct Effect {
    duration: i32,
    spell: Spell
}

impl Effect {
    fn with_spell(spell: Spell) -> Self {
        let duration = spell.initial_duration();
        Self {
            spell,
            duration,
        }
    }

    fn is_active(&self) -> bool {
        self.duration > 0
    }

    fn cast(&mut self, wizard: &mut Wizard, boss: &mut Boss) {
        match self.spell {
            Spell::Poison => boss.hit_points -= 3,
            Spell::Recharge => wizard.mana += 101,
            _ => ()
        }
    }

    fn activate(&self, wizard: &mut Wizard, boss: &mut Boss) {
        match self.spell {
            Spell::MagicMissile => boss.hit_points -= 4,
            Spell::Drain => {
                boss.hit_points -= 2;
                wizard.hit_points += 2;
            },
            Spell::Shield => wizard.armor += 7,
            _ => ()
        }
    }

    fn decrease(&mut self, wizard: &mut Wizard) {
        self.duration -= 1;
        if self.is_active() {
            return;
        }
        match self.spell {
            Spell::Shield => wizard.armor -= 7,
            _ => ()
        }
    }
}

struct Boss {
    hit_points: i32,
    damage: i32
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

struct Wizard {
    hit_points: i32,
    mana: i32,
    armor: i32,
}

impl Wizard {
    fn new() -> Self {
        Self {
            hit_points: 50,
            mana: 500,
            armor: 0,
        }
    }

    fn is_alive(&self) -> bool {
        self.hit_points > 0 && self.mana > 0   
    }
}

fn perform_battle(spells: &Vec<Spell>, wizard: &mut Wizard, boss: &mut Boss) {
    let mut timer = 0;
    let mut ptr = 0;
    let mut active_effects: Vec<Effect> = Vec::new();
    while wizard.is_alive() && boss.is_alive() {
        if timer % 2 == 0 { 
            if ptr < spells.len () {
                let spell = spells[ptr];
                ptr += 1;
                if spell.cost() < wizard.mana {
                    wizard.mana -= spell.cost();
                    let effect = Effect::with_spell(spell);
                    effect.activate(wizard, boss);            
                    active_effects.push(effect);
                } else {
                    wizard.mana = 0;
                    break;
                } 
            }
        }
        active_effects.iter_mut()
            .for_each(|effect| effect.cast(wizard, boss));        
        if timer % 2 != 0 {
            let damage = (boss.damage - wizard.armor).max(1);
            wizard.hit_points -= damage;
        }
        timer += 1;
        active_effects.iter_mut()
            .for_each(|e| e.decrease(wizard));

        active_effects = active_effects.into_iter()
            .filter(|ef| ef.is_active())
            .collect();
    }
}

fn is_wizard_win(spells: &Vec<Spell>) -> bool {
    let mut wizard = Wizard::new();
    let mut boss = Boss::new();
    perform_battle(spells, &mut wizard, &mut boss);
    wizard.is_alive()
}

fn is_valid(spells: &Vec<Spell>) -> bool {
    let mut map: HashMap<Spell, u32> = HashMap::new();
    for spell in Spell::all_cases() {
        map.insert(spell, 0);
    }
    for s in spells.iter() {
        let val = *map.get(s).unwrap();
        if val < 2 {
            map.insert(*s, s.initial_duration() as u32);
        } else {
            return false;
        }
        map.iter_mut()
            .for_each(|(_, v)| *v = v.saturating_sub(1));
    }
    true
}

fn calc_min_mana(spells: &mut Vec<Spell>, cost: &mut i32, min_mana: &mut i32) {
    if spells.len() > 12 { 
        return;
    }
    for spell in Spell::all_cases() {
        let s_cost = spell.cost();
        *cost += s_cost;
        spells.push(spell);
        if *cost < *min_mana && is_valid(spells) {
            if is_wizard_win(spells) {
                *min_mana = *cost;
            } else {
                calc_min_mana(spells, cost, min_mana);
            }
        }
        spells.pop();
        *cost -= s_cost;
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
        let mut spells = vec![];
        let mut result = i32::MAX;
        let mut cost = 0;
        calc_min_mana(&mut spells, &mut cost, &mut result);
        result.to_string()
    }

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

    #[test]
    fn aoc2015_22_battle_test1() {
        let mut wizard = Wizard {
            hit_points: 10,
            armor: 0,
            mana: 250,
        };

        let mut boss = Boss {
            hit_points: 13,
            damage: 8,
        };
        perform_battle(&vec![Spell::Poison, Spell::MagicMissile], &mut wizard, &mut boss);
        assert!(!boss.is_alive());
        assert_eq!(wizard.hit_points, 2);
        assert_eq!(wizard.armor, 0);
        assert_eq!(wizard.mana, 24);
    }

    #[test]
    fn aoc2015_22_battle_test2() {
        let mut wizard = Wizard {
            hit_points: 10,
            armor: 0,
            mana: 250,
        };

        let mut boss = Boss {
            hit_points: 14,
            damage: 8,
        };
        let spells = vec![
            Spell::Recharge, 
            Spell::Shield, 
            Spell::Drain, 
            Spell::Poison, 
            Spell::MagicMissile
        ];
        perform_battle(&spells, &mut wizard, &mut boss);
        assert!(!boss.is_alive());
        assert_eq!(wizard.hit_points, 1);
        assert_eq!(wizard.armor, 0);
        assert_eq!(wizard.mana, 114);
    }
}