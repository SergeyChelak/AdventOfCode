use crate::solution::Solution;

use std::io;

const CUPS_SIZE: usize = 9;
type Cups = [u8; CUPS_SIZE];

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
            .map(|x| x as u8)
            .collect::<Vec<_>>();
        assert_eq!(v.len(), CUPS_SIZE);
        Self {
            input: v
                .try_into()
                .expect("Failed to convert input vector to array"),
        }
    }
}

impl Solution for AoC2020_23 {
    fn part_one(&self) -> String {
        let digits = (0..100).fold(self.input, |arr, _| simulate_step(&arr));

        let start = digits
            .iter()
            .enumerate()
            .find(|(_, x)| **x == 1)
            .expect("Unreachable: 1 not found")
            .0;

        (1..CUPS_SIZE)
            .map(|idx| digits[(idx + start) % CUPS_SIZE])
            .map(|x| (x + b'0') as char)
            .collect::<String>()
    }

    // fn part_two(&self) -> String {
    //     let mut arr = self.input;
    //     let mut store = HashSet::new();

    //     for i in 0usize.. {
    //         println!("digits: {:?}", arr);
    //         let key = (0..CUPS_SIZE)
    //             .map(|idx| arr[idx])
    //             .map(|x| (x + b'0') as char)
    //             .collect::<String>();
    //         if store.contains(&key) {
    //             println!("Loop found: {key} at {} step", i + 1);
    //             break;
    //         }
    //         store.insert(key);

    //         arr = simulate_step(&arr);
    //     }

    //     not_found()
    // }

    fn description(&self) -> String {
        "Day 23: Crab Cups".to_string()
    }
}

fn simulate_step(input: &Cups) -> Cups {
    let mut available = [true; CUPS_SIZE + 1];
    (0..4).for_each(|i| available[input[i] as usize] = false);

    let mut dest = input[0];
    while !available[dest as usize] {
        dest -= 1;
        if dest == 0 {
            dest = CUPS_SIZE as u8;
        }
    }

    let mut output_ptr = 0usize;
    let mut output = [0; CUPS_SIZE];
    let mut input_ptr = 4;
    while output_ptr < CUPS_SIZE {
        output[output_ptr] = input[input_ptr];

        let is_dest = input[input_ptr] == dest;
        output_ptr += 1;
        input_ptr = (input_ptr + 1) % CUPS_SIZE;

        if is_dest {
            output[output_ptr] = input[1];
            output_ptr += 1;
            output[output_ptr] = input[2];
            output_ptr += 1;
            output[output_ptr] = input[3];
            output_ptr += 1;
        }
    }

    assert!(output.iter().all(|x| *x != 0));

    output
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
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_23> {
        AoC2020_23::new()
    }

    #[test]
    fn aoc2020_23_simulate_step() {
        {
            let input = [3, 8, 9, 1, 2, 5, 4, 6, 7];
            let output = [2, 8, 9, 1, 5, 4, 6, 7, 3];
            assert_eq!(output, simulate_step(&input));
        }

        {
            let input = [2, 8, 9, 1, 5, 4, 6, 7, 3];
            let output = [5, 4, 6, 7, 8, 9, 1, 3, 2];
            assert_eq!(output, simulate_step(&input));
        }

        {
            let input = [5, 4, 6, 7, 8, 9, 1, 3, 2];
            let output = [8, 9, 1, 3, 4, 6, 7, 2, 5];
            assert_eq!(output, simulate_step(&input));
        }
    }
}
