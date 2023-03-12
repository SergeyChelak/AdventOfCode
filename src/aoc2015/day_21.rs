use crate::solution::Solution;

use std::io;

struct Equipment {
    name: String,
    cost: i32,
    damage: i32,
    armor: i32,
}

impl Equipment {
    fn new(name: &str, cost: i32, damage: i32, armor: i32) -> Self {
        Self {
            name: name.to_string(),
            cost,
            damage,
            armor,
        }
    }

    fn weapons() -> Vec::<Self> {
        vec![
            Self::new("Dagger", 8, 4, 0),
            Self::new("Shortsword", 10, 5, 0),
            Self::new("Warhammer", 25, 6, 0),
            Self::new("Longsword", 40, 7, 0),
            Self::new("Greataxe", 74, 8, 0)
        ]
    }

    fn armor() -> Vec::<Self> {
        vec! [
            Self::new("Leather", 13, 0, 1),
            Self::new("Chainmail", 31, 0, 2),
            Self::new("Splintmail", 53, 0, 3),
            Self::new("Bandedmail", 75, 0, 4),
            Self::new("Platemail", 102, 0, 5)
        ]
    }

    fn rings() -> Vec::<Self> {
        vec! [
            Self::new("Damage +1", 25, 1, 0),
            Self::new("Damage +2", 50, 2, 0),
            Self::new("Damage +3", 100, 3, 0),
            Self::new("Defense +1", 20, 0, 1),
            Self::new("Defense +2", 40, 0, 2),
            Self::new("Defense +3", 80, 0, 3)
        ]
    }
}

struct Player {
    hit_points: i32,
    damage: i32,
    armor: i32
}

impl Player {
    fn with_equipment(equip: &Vec<Equipment>) -> Self {
        let params = equip.iter()
            .map(|e| (e.damage, e.armor))
            .fold((0, 0), |(total_dmg, total_armor), (dmg, armor)| {
                (total_dmg + dmg, total_armor + armor)
            });
        Self {
            hit_points: 100,
            damage: params.0,
            armor: params.1,
        }
    }

    fn boss() -> Self {
        Self {
            hit_points: 100,
            damage: 8,
            armor: 2,
        }
    }

    fn attack(&self, other: &mut Self) {
        let damage = (self.damage - other.armor).max(1);
        other.hit_points -= damage;
    }

    fn is_alive(&self) -> bool {
        self.hit_points > 0
    }
}

fn battle(first: &mut Player, second: &mut Player) {
    let mut is_first = true;
    while first.is_alive() && second.is_alive() {
        if is_first {
            first.attack(second);
        } else {
            second.attack(first);
        }
        is_first = !is_first;
    }
}

pub struct AoC2015_21;
impl AoC2015_21 {
    pub fn new() -> io::Result<Self> {
        Ok(Self)
    }
}

impl Solution for AoC2015_21 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2015/Day 21: RPG Simulator 20XX"
        .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_21_correctness() -> io::Result<()> {
        let sol = AoC2015_21::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2015_21_attack() {
        let mut player = Player {
            hit_points: 8,
            damage: 5,
            armor: 5,
        };

        let mut boss = Player {
            hit_points: 12,
            damage: 7,
            armor: 2,
        };

        player.attack(&mut boss);
        assert_eq!(boss.hit_points, 9);

        boss.attack(&mut player);
        assert_eq!(player.hit_points, 6);

        player.attack(&mut boss);
        assert_eq!(boss.hit_points, 6);

        boss.attack(&mut player);
        assert_eq!(player.hit_points, 4);
        
        player.attack(&mut boss);
        assert_eq!(boss.hit_points, 3);

        boss.attack(&mut player);
        assert_eq!(player.hit_points, 2);

        player.attack(&mut boss);
        assert_eq!(boss.hit_points, 0);
    }

    #[test]
    fn aoc2015_21_battle() {
        let mut player = Player {
            hit_points: 8,
            damage: 5,
            armor: 5,
        };

        let mut boss = Player {
            hit_points: 12,
            damage: 7,
            armor: 2,
        };
        battle(&mut player, &mut boss);
        assert_eq!(player.hit_points, 2);
        assert_eq!(boss.hit_points, 0);
    }
}