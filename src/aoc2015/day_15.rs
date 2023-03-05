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

fn scores(amount: &Vec<i32>, ingredients: &Vec<Ingredient>, fields: &Vec<usize>) -> i64 {
    let mut result = 1i64;
    for field in fields {
        let value = amount.iter().zip(ingredients.iter()
            .map(|i| {
                match field {
                    0 => i.capacity,
                    1 => i.durability,
                    2 => i.flavor,
                    3 => i.texture,
                    4 => i.calories,
                    _ => panic!("field id isn't supported")
                }
            }))
            .map(|(a, v)| *a * v)
            .sum::<i32>();
        if value > 0 {
            result *= value as i64;
        } else {
            return 0;
        }
    }
    result
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
    fn part_one(&self) -> String {
        let size = self.ingredients.len();
        let mut counters = vec![1i32; size];
        let max = (100 - size) as i32 + 1;
        let fieds = vec![0usize, 1, 2, 3];
        counters[size - 1] = max;
        let mut best = 0i64;
        loop {
            let sum = counters.iter().sum::<i32>();
            if sum == 100 {
                let val = scores(&counters, &self.ingredients, &fieds) as i64;
                best = best.max(val);
            }
            let mut carry = 1;
            for val in counters.iter_mut().rev() {
                let next = *val + carry;
                if next > max {
                    carry = 1;
                    *val = 1;
                } else {
                    carry = 0;
                    *val = next;
                    break;
                }
            }
            if carry == 1 {
                break;
            }
        }
        best.to_string()
    }

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
        assert_eq!(sol.part_one(), "222870");
        //assert_eq!(sol.part_two(), "");
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

    #[test]
    fn aoc2015_15_calc_scores() -> Result<(), ParseIntError> {
        let i1 = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8"
            .parse::<Ingredient>()?;
        let i2 = "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"
            .parse::<Ingredient>()?;
        assert_eq!(scores(&vec![44, 56], &vec![i1, i2], &vec![0, 1, 2, 3]), 62842880);
        Ok(())
    }
}