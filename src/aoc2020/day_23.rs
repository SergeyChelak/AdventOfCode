use crate::solution::Solution;

use std::io;

type Int = usize;
type Cups = Vec<Int>;

pub struct AoC2020_23 {
    input: Cups,
}

impl AoC2020_23 {
    pub fn new() -> io::Result<Self> {
        let input = std::fs::read_to_string("input/aoc2020_23")?;
        Ok(Self::parse(&input))
    }

    fn parse(input: &str) -> Self {
        let v = input
            .trim()
            .chars()
            .map(|x| {
                x.to_digit(10)
                    .expect("Input shouldn't contain non digit characters")
            })
            .map(|x| x as Int)
            .collect::<Vec<_>>();
        Self { input: v }
    }
}

impl Solution for AoC2020_23 {
    fn part_one(&self) -> String {
        let result = simulate(&self.input, 100);
        let mut output = String::new();
        let mut idx = 1;
        (0..self.input.len() - 1).for_each(|_| {
            output.push_str(&result[idx].to_string());
            idx = result[idx];
        });
        output
    }

    fn part_two(&self) -> String {
        let mut cups = self.input.clone();
        let max = *cups.iter().max().expect("Cups can't be empty");
        for x in max + 1..=1000000 {
            cups.push(x);
        }
        let result = simulate(&cups, 10000000);
        let one = result[1];
        let two = result[one];
        (one * two).to_string()
    }

    fn description(&self) -> String {
        "Day 23: Crab Cups".to_string()
    }
}

fn simulate(cups: &Cups, steps: usize) -> Cups {
    let mut next = vec![0; cups.len() + 1];
    for i in 0..cups.len() - 1 {
        next[cups[i]] = cups[i + 1];
    }
    next[cups[cups.len() - 1]] = cups[0];

    let mut current = cups[0];
    for _ in 0..steps {
        let mut pickup = Vec::new();
        let mut tmp = current;
        for _ in 0..3 {
            tmp = next[tmp];
            pickup.push(tmp);
        }

        let mut new;
        let mut i = 1;
        loop {
            new = if current > i {
                current - i
            } else {
                cups.len() + current - i
            };
            if !pickup.contains(&new) {
                break;
            }
            i += 1;
        }

        next.swap(new, current);
        next.swap(current, pickup[pickup.len() - 1]);

        current = next[current];
    }
    next
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_23_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(sol.input.iter().all(|x| *x != 0));
        Ok(())
    }

    #[test]
    fn aoc2020_23_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "38756249");
        Ok(())
    }

    #[test]
    fn aoc2020_23_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "21986479838");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_23> {
        AoC2020_23::new()
    }
}
