use crate::solution::Solution;

use std::io;

type Int = i64;

pub struct AoC2019_04 {
    from: Int,
    to: Int,
}

impl AoC2019_04 {
    pub fn new() -> io::Result<Self> {
        let input = "130254-678275";
        Ok(Self::with_str(input))
    }

    fn with_str(s: &str) -> Self {
        let (from, to) = s.split_once('-').expect("Incorrect input");
        let from = from.parse::<Int>().expect("Non integer start value");
        let to = to.parse::<Int>().expect("Non integer end value");
        Self { from, to }
    }
}

impl Solution for AoC2019_04 {
    fn part_one(&self) -> String {
        let mut count = 0;
        for val in self.from..=self.to {
            if is_valid(val) {
                count += 1;
            }
        }
        count.to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 4: Secure Container".to_string()
    }
}

fn is_valid(value: Int) -> bool {
    let mut usage = [0; 10];
    let mut rest = value;
    let mut has_double = false;
    let mut prev = 11;
    while rest > 0 {
        let x = rest % 10;
        usage[x as usize] += 1;
        has_double |= usage[x as usize] > 1;
        if x > prev {
            return false;
        }
        prev = x;
        rest /= 10;
    }
    has_double
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2019_04_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "2090");
        Ok(())
    }

    #[test]
    fn aoc2019_04_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2019_04> {
        AoC2019_04::new()
    }
}
