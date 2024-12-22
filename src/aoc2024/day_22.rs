use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io;

type Int = isize;

pub struct AoC2024_22 {
    input: Vec<Int>,
}

impl AoC2024_22 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2024_22")?;
        Ok(Self::with_lines(&lines))
    }

    fn with_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|s| s.as_ref())
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<Int>().expect("Invalid input value"))
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2024_22 {
    fn part_one(&self) -> String {
        self.input
            .iter()
            .map(|x| generate(*x, 2000))
            .sum::<Int>()
            .to_string()
    }

    fn part_two(&self) -> String {
        let mut accumulator = HashMap::<u64, Int>::new();
        for secret in self.input.iter() {
            for (key, value) in make_sequences(*secret) {
                let entry = accumulator.entry(key).or_default();
                *entry += value;
            }
        }
        accumulator
            .values()
            .max()
            .expect("Empty accumulator isn't expected")
            .to_string()
    }

    fn description(&self) -> String {
        "2024/Day 22: Monkey Market".to_string()
    }
}

fn make_sequences(secret: Int) -> HashMap<u64, Int> {
    let info = generate_price_info(secret, 2000);
    let mut map = HashMap::new();
    for (i, seq) in info.diff[..].windows(4).enumerate() {
        let mut hasher = DefaultHasher::new();
        seq.hash(&mut hasher);
        let hash = hasher.finish();
        if map.contains_key(&hash) {
            continue;
        }
        let val = info.price[i + 4];
        map.insert(hash, val);
    }
    map
}

struct PriceInfo {
    price: Vec<Int>,
    diff: Vec<Int>,
}

fn generate_price_info(mut secret: Int, length: usize) -> PriceInfo {
    let mut prev = secret % 10;
    let mut price = vec![prev];
    let mut diff = Vec::new();
    for _ in 0..length {
        secret = generate_step(secret);
        let val = secret % 10;
        price.push(val);
        diff.push(val - prev);
        prev = val;
    }
    PriceInfo { price, diff }
}

fn generate(mut secret: Int, number: usize) -> Int {
    for _ in 0..number {
        secret = generate_step(secret);
    }
    secret
}

fn generate_step(mut secret: Int) -> Int {
    let mix = |val: Int, secret: Int| val ^ secret;
    let prune = |val: Int| val % 16777216;

    secret = mix(secret * 64, secret);
    secret = prune(secret);

    secret = mix(secret / 32, secret);
    secret = prune(secret);

    secret = mix(secret * 2048, secret);
    prune(secret)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_22_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2024_22_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "12664695565");
        Ok(())
    }

    #[test]
    fn aoc2024_22_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "1444");
        Ok(())
    }

    #[test]
    fn aoc2024_22_generate() {
        assert_eq!(15887950, generate(123, 1));
        assert_eq!(16495136, generate(123, 2));
        assert_eq!(527345, generate(123, 3));
        assert_eq!(704524, generate(123, 4));
        assert_eq!(1553684, generate(123, 5));
        assert_eq!(12683156, generate(123, 6));
        assert_eq!(11100544, generate(123, 7));
        assert_eq!(12249484, generate(123, 8));
        assert_eq!(7753432, generate(123, 9));
        assert_eq!(5908254, generate(123, 10));
    }

    #[test]
    fn aoc2024_22_gen_price_info() {
        let info = generate_price_info(1, 2000);
        assert_eq!(1, info.price.len() - info.diff.len());
    }

    fn make_solution() -> io::Result<AoC2024_22> {
        AoC2024_22::new()
    }
}
