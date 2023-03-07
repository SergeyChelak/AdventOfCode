use crate::solution::Solution;
use crate::utils::*;

use std::io;
use std::str::FromStr;
use std::num::ParseIntError;

struct Ingredient {
    components: Vec<i32>
}

impl FromStr for Ingredient {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elems = s.split(": ").collect::<Vec<&str>>();
        let elems = elems[1].split(", ").collect::<Vec<&str>>();
        let values = elems.iter()
            .map(|item| {
                let props = item.split(" ").collect::<Vec<&str>>();    
                props[1].parse::<i32>().ok()
                    .expect("Failed to parse ingedient value")
            })
            .collect::<Vec<i32>>();
        Ok(Self {
            components: values
        })
    }
}

fn scores(amount: &Vec<usize>, ingredients: &Vec<Ingredient>, fields: &Vec<usize>) -> i64 {
    let mut result = 1i64;
    for field in fields {
        let value = amount.iter().zip(ingredients.iter()
            .map(|i| i.components[*field]))
            .map(|(a, v)| *a as i32 * v)
            .sum::<i32>();
        if value > 0 {
            result *= value as i64;
        } else {
            return 0;
        }
    }
    result
}

struct IndexSumIterator {
    target: usize,
    size: usize,
    sp: usize,          // stack pointer
    stack_idx: Vec<usize>,
    stack_sum: Vec<usize>,
    items: Vec<usize>,
}

impl IndexSumIterator {
    pub fn new(target: usize, size: usize) -> Self {
        Self {
            target,
            size,
            sp: 0,
            stack_idx: vec![1; size + 1],
            stack_sum: vec![0; size + 1],
            items: vec![0usize; size],
        }
    }
}

impl Iterator for IndexSumIterator {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.target < self.size {
            return None;
        }
        loop {
            if self.sp == self.size {
                if self.stack_sum[self.sp] == self.target {
                    self.sp -= 1;
                    return Some(self.items.clone());
                }
                self.sp -= 1;
            } else {
                let max = self.target as isize - self.stack_sum[self.sp] as isize - self.size as isize + self.sp as isize + 1;
                if self.stack_idx[self.sp] as isize <= max {
                    self.items[self.sp] = self.stack_idx[self.sp];
                    self.stack_idx[self.sp + 1] = 1;
                    self.stack_sum[self.sp + 1] = self.stack_idx[self.sp] + self.stack_sum[self.sp];
                    self.stack_idx[self.sp] += 1;
                    self.sp += 1;
                } else if self.sp == 0 {
                    return None
                } else {
                    self.sp -= 1;
                }
            }
        }
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
    fn part_one(&self) -> String {
        let size = self.ingredients.len();
        let fields = vec![0usize, 1, 2, 3];
        let best = IndexSumIterator::new(100, size)
            .fold(0, |acc, counters| {
                let val = scores(&counters, &self.ingredients, &fields);
                acc.max(val)
            });
        best.to_string()
    }

    fn part_two(&self) -> String {
        let size = self.ingredients.len();
        let fields = vec![0usize, 1, 2, 3];
        let best = IndexSumIterator::new(100, size)
            .fold(0, |acc, counters| {
                let cals = scores(&counters, &self.ingredients, &vec![4]) as i64;
                if cals == 500 {
                    let val = scores(&counters, &self.ingredients, &fields);
                    acc.max(val)
                } else {
                    acc
                }
                
            });
        best.to_string()
    }

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
        assert_eq!(sol.part_two(), "117936");
        Ok(())
    }

    #[test]
    fn aoc2015_15_ingredient_parse() -> Result<(), ParseIntError> {
        let str = "Sugar: capacity 3, durability 1, flavor 0, texture -3, calories 2";
        let item = str.parse::<Ingredient>()?;
        assert_eq!(item.components[0], 3);
        assert_eq!(item.components[1], 1);
        assert_eq!(item.components[2], 0);
        assert_eq!(item.components[3], -3);
        assert_eq!(item.components[4], 2);
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