use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
use std::io;

type Component = String;
type ComponentStore = HashSet<Component>;

#[derive(Debug, Clone)]
struct Food {
    ingredients: ComponentStore,
    allergens: ComponentStore,
}

impl Food {
    fn remove(&mut self, ingredient: &str, allergens: &ComponentStore) {
        if !self.ingredients.remove(ingredient) {
            return;
        }

        for allergen in allergens {
            if self.allergens.remove(allergen) {
                break;
            }
        }
    }

    fn intersection(&self, other: &Self) -> Self {
        let intersect = |f: &ComponentStore, s: &ComponentStore| -> ComponentStore {
            f.intersection(s).cloned().collect::<ComponentStore>()
        };
        let ingredients = intersect(&self.ingredients, &other.ingredients);
        let allergens = intersect(&self.allergens, &other.allergens);
        Self {
            ingredients,
            allergens,
        }
    }

    fn has_ingredients(&self) -> bool {
        !self.ingredients.is_empty()
    }

    fn is_only_ingredient(&self) -> bool {
        self.ingredients.len() == 1
    }

    fn has_allergens(&self) -> bool {
        !self.allergens.is_empty()
    }

    fn any_ingredient(&self) -> Option<Component> {
        self.ingredients.iter().next().cloned()
    }

    fn simplified(&self) -> Self {
        let allergens = self
            .allergens
            .iter()
            .map(|s| {
                if let Some(x) = s.strip_prefix("contains ") {
                    x
                } else {
                    s.as_str()
                }
            })
            .map(|s| s.to_string())
            .collect::<ComponentStore>();
        assert_eq!(self.allergens.len(), allergens.len());
        Self {
            ingredients: self.ingredients.clone(),
            allergens,
        }
    }
}

impl From<&str> for Food {
    fn from(value: &str) -> Self {
        let (part1, part2) = value.split_once(" (").expect("Invalid input format");
        let ingredients = part1
            .split(' ')
            .map(|s| s.trim())
            .map(|s| s.to_string())
            .collect::<ComponentStore>();

        let part2 = part2.trim();
        let len = part2.len();
        let allergens = part2[..len - 1]
            .split(", ")
            .map(|s| s.to_string())
            .collect::<ComponentStore>();

        Self {
            ingredients,
            allergens,
        }
    }
}

pub struct AoC2020_21 {
    input: Vec<Food>,
}

impl AoC2020_21 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2020_21")?;
        Ok(Self::parse(&lines))
    }

    fn parse<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(Food::from)
            .collect::<Vec<_>>();
        Self { input }
    }

    fn simplified_input(&self) -> Vec<Food> {
        self.input
            .iter()
            .map(|f| f.simplified())
            .collect::<Vec<_>>()
    }

    fn solve(&self) -> (Vec<Food>, HashMap<Component, Component>) {
        let mut allergens = HashMap::<String, String>::new();
        let mut food = self.simplified_input();

        let unresolved = |all_food: &[Food], seen: &HashSet<usize>| -> Option<usize> {
            all_food
                .iter()
                .enumerate()
                .find(|(idx, f)| !seen.contains(idx) && f.has_ingredients() && f.has_allergens())
                .map(|x| x.0)
        };

        let mut seen = HashSet::<usize>::new();

        while let Some(idx) = unresolved(&food, &seen) {
            seen.insert(idx);
            let mut candidate: Option<(String, ComponentStore)> = None;

            let mut f = food[idx].clone();
            for other in food.iter() {
                let next = f.intersection(other);
                match (next.has_ingredients(), next.has_allergens()) {
                    (true, true) if next.is_only_ingredient() => {
                        candidate = Some((
                            next.any_ingredient()
                                .expect("Not expected to be reachable: no ingredient"),
                            next.allergens,
                        ));
                        break;
                    }
                    (true, true) => {
                        f = next;
                    }
                    _ => {
                        continue;
                    }
                }
            }

            let Some(candidate) = candidate else {
                continue;
            };

            assert_eq!(1, candidate.1.len());
            for allergen in &candidate.1 {
                allergens.insert(candidate.0.clone(), allergen.clone());
            }

            seen.clear();
            food.iter_mut()
                .for_each(|elem| elem.remove(&candidate.0, &candidate.1));
        }

        (food, allergens)
    }
}

impl Solution for AoC2020_21 {
    fn part_one(&self) -> String {
        let (safe_food, _) = self.solve();
        safe_food
            .iter()
            .map(|f| f.ingredients.len())
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        let (_, allergens) = self.solve();
        let mut arr = allergens.iter().collect::<Vec<_>>();
        arr.sort_by_key(|x| x.1);

        arr.iter()
            .map(|x| x.0.clone())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn description(&self) -> String {
        "Day 21: Allergen Assessment".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_21_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2020_21_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "2203");
        Ok(())
    }

    #[test]
    fn aoc2020_21_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(
            sol.part_two(),
            "fqfm,kxjttzg,ldm,mnzbc,zjmdst,ndvrq,fkjmz,kjkrm"
        );
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_21> {
        AoC2020_21::new()
    }

    #[test]
    fn aoc2020_21_case1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "5");
    }

    fn make_test_solution() -> AoC2020_21 {
        let lines = [
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)",
            "trh fvjkl sbzzf mxmxvkd (contains dairy)",
            "sqjhc fvjkl (contains soy)",
            "sqjhc mxmxvkd sbzzf (contains fish)",
        ];
        AoC2020_21::parse(&lines)
    }

    #[test]
    fn aoc2020_21_parse_food() {
        let s = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)";
        let food = Food::from(s);

        let components =
            |arr: &[&str]| -> ComponentStore { arr.iter().map(|s| s.to_string()).collect() };

        assert_eq!(
            food.ingredients,
            components(&["mxmxvkd", "kfcds", "sqjhc", "nhms"])
        );

        assert_eq!(food.allergens, components(&["contains dairy", "fish"]))
    }
}
