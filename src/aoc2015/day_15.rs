use crate::solution::Solution;
use crate::utils::*;

use std::io;
use std::str::FromStr;
use std::num::ParseIntError;

struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32
}

impl FromStr for Ingredient {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elems = s.split(": ").collect::<Vec<&str>>();
        let name = elems[0].to_string();
        let elems = elems[1].split(", ").collect::<Vec<&str>>();
        let values = elems.iter()
            .map(|item| {
                let props = item.split(" ").collect::<Vec<&str>>();    
                props[1].parse::<i32>().ok()
                    .expect("Failed to parse ingedient value")
            })
            .collect::<Vec<i32>>();
        Ok(Self {
            name,
            capacity: values[0],
            durability: values[1],
            flavor: values[2],
            texture: values[3],
            calories: values[4]
        })
    }
}

pub struct AoC2015_15 {
    ingredients: Vec<Ingredient>,
}

impl AoC2015_15 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2015_15")?;
        let mut ingredients: Vec<Ingredient> = Vec::with_capacity(lines.len());
        for line in lines {
            let ingr = line.parse::<Ingredient>().ok()
                .expect("Failed to parse ingedient");
            ingredients.push(ingr);
        }
        Ok(Self {
            ingredients
        })
    }
}

impl Solution for AoC2015_15 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2015/Day 15: Science for Hungry People".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_15_input_load_test() -> io::Result<()> {
        let sol = AoC2015_15::new()?;
        assert!(sol.ingredients.len() > 0);
        Ok(())
    }

    #[test]
    fn aoc2015_15_correctness() -> io::Result<()> {
        let sol = AoC2015_15::new()?;
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2015_15_ingredient_parse() -> Result<(), ParseIntError> {
        let str = "Sugar: capacity 3, durability 1, flavor 0, texture -3, calories 2";
        let item = str.parse::<Ingredient>()?;
        assert_eq!(item.name, "Sugar");
        assert_eq!(item.capacity, 3);
        assert_eq!(item.durability, 1);
        assert_eq!(item.flavor, 0);
        assert_eq!(item.texture, -3);
        assert_eq!(item.calories, 2);
        Ok(())
    }
}