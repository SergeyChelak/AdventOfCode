use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Int = usize;

pub struct AoC2020_25 {
    pk1: Int,
    pk2: Int,
}

impl AoC2020_25 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2020_25")?;
        Ok(Self::parse(&lines))
    }

    fn parse<T: AsRef<str>>(lines: &[T]) -> Self {
        assert_eq!(2, lines.len());
        let pk1 = lines[0].as_ref().parse::<Int>().expect("Invalid PK1");
        let pk2 = lines[1].as_ref().parse::<Int>().expect("Invalid PK2");

        Self { pk1, pk2 }
    }
}

impl Solution for AoC2020_25 {
    fn part_one(&self) -> String {
        let Some(loops) = calc_loops(7, self.pk1) else {
            return not_found();
        };
        let key = transform(self.pk2, loops);
        key.to_string()
    }

    fn description(&self) -> String {
        "Day 25: Combo Breaker".to_string()
    }
}

const MODULUS: Int = 20201227;

fn transform(subject: Int, loops: Int) -> Int {
    let mut val = 1;
    for _ in 0..loops {
        val *= subject;
        val %= MODULUS;
    }
    val
}

fn calc_loops(subject: Int, pk: Int) -> Option<Int> {
    let mut val = 1;
    for i in 1.. {
        val *= subject;
        val %= MODULUS;
        if val == pk {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_25_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(sol.pk1 > 0);
        assert!(sol.pk2 > 0);
        Ok(())
    }

    #[test]
    fn aoc2020_25_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "296776");
        Ok(())
    }

    #[test]
    fn aoc2020_25_case1() {
        let sol = AoC2020_25 {
            pk1: 5764801,
            pk2: 17807724,
        };
        assert_eq!(sol.part_one(), "14897079");
    }

    #[test]
    fn aoc2020_25_loops_count() {
        {
            let pk = 5764801;
            let count = calc_loops(7, pk);
            assert_eq!(Some(8), count);
        }

        {
            let pk = 17807724;
            let count = calc_loops(7, pk);
            assert_eq!(Some(11), count);
        }
    }

    fn make_solution() -> io::Result<AoC2020_25> {
        AoC2020_25::new()
    }
}
