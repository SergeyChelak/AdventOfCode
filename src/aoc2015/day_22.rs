use crate::solution::Solution;

use std::io;

#[derive(Clone, Copy)]
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
}

enum Aftermath {
    Win,
    Lose,
    // Overlap,
    // ManaOver,
    // ExcessiveSpells,
    InsufficientSpells,
}

struct Wizard {
    hit_points: i32,
    armor: i32,
    mana: i32,
}

struct Boss {
    hit_points: i32,
    damage: i32,
}

fn calc_min_mana_amount(cost: i32, spells: &mut Vec<Spell>, result: &mut i32) {
    if cost >= *result {
        return;
    }
    for spell in Spell::all_cases() {
        spells.push(spell);
        match eval(&spells) {
            Aftermath::Win => *result = cost.min(*result),
            Aftermath::InsufficientSpells => calc_min_mana_amount(cost + spell.cost(), spells, result),
            _ => (),
        }
        spells.pop();
    }
}

fn eval(spells: &Vec<Spell>) -> Aftermath {
    let mut wizard = Wizard {
        hit_points: 50,
        armor: 0,
        mana: 500,
    };
    let mut boss = Boss {
        hit_points: 58,
        damage: 9,
    };
    battle(&mut wizard, &mut boss, spells)
}

fn battle(wizard: &mut Wizard, boss: &mut Boss, spells: &Vec<Spell>) -> Aftermath {
    let mut spells = spells.into_iter().rev()
        .map(|el| el.clone())
        .collect::<Vec<Spell>>();
    let mut is_wizard_move = true;
    while wizard.hit_points > 0 && boss.hit_points > 0 {
        // todo: cast effects...
        if is_wizard_move {
            let spell = spells.pop();
            if spell.is_none() {
                return Aftermath::InsufficientSpells;
            }
            let spell = spell.unwrap();
            if spell.cost() > wizard.mana {
                return Aftermath::Lose;
            }
            todo!()
        } else {
            let damage = (boss.damage - wizard.armor).max(1);
            wizard.hit_points -= damage;
        }
        is_wizard_move = !is_wizard_move;
    }
    if wizard.hit_points > 0 { Aftermath::Win } else { Aftermath::Lose }
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
        calc_min_mana_amount(0, &mut Vec::new(), &mut result);
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
        battle(&mut wizard, &mut boss, &vec![Spell::Poison, Spell::MagicMissile]);
        assert!(boss.hit_points <= 0);
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
        battle(&mut wizard, &mut boss, &spells);
        assert!(boss.hit_points <= 0);
        assert_eq!(wizard.hit_points, 1);
        assert_eq!(wizard.armor, 0);
        assert_eq!(wizard.mana, 114);
    }
}
