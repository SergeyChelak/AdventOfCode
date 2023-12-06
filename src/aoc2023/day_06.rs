use crate::solution::Solution;

use std::io;

type Int = u64;

struct Record {
    time: Int,
    distance: Int,
}

pub struct AoC2023_06 {
    input: Vec<Record>,
    // input_part2: Record,
}

impl AoC2023_06 {
    pub fn new() -> io::Result<Self> {
        //
        // Time:        61     67     75     71
        // Distance:   430   1036   1307   1150
        //
        Ok(Self {
            #[rustfmt::skip]
            input: vec![
                Record { time: 61, distance: 430 },
                Record { time: 67, distance: 1036 },
                Record { time: 75, distance: 1307 },
                Record { time: 71, distance: 1150 },
            ],
        })
    }

    fn merged_input(&self) -> Record {
        let join_digit = |arr: &[Int]| -> Int {
            arr.iter()
                .map(|val| val.to_string())
                .collect::<Vec<_>>()
                .join("")
                .parse::<Int>()
                .expect("Int value is expected")
        };
        let time = self.input.iter().map(|x| x.time).collect::<Vec<_>>();
        let distance = self.input.iter().map(|x| x.distance).collect::<Vec<_>>();

        Record {
            time: join_digit(&time),
            distance: join_digit(&distance),
        }
    }
}

impl Solution for AoC2023_06 {
    fn part_one(&self) -> String {
        let mut prod = 1;
        for rec in &self.input {
            let mut count = 0;
            for charge in 1..rec.time {
                let speed = charge;
                let dist = speed * (rec.time - charge);
                if dist > rec.distance {
                    count += 1;
                }
            }
            if count > 0 {
                prod *= count;
            }
        }
        prod.to_string()
    }

    fn part_two(&self) -> String {
        let rec = self.merged_input();
        let mut count = 0;
        for charge in 1..rec.time {
            let speed = charge;
            let dist = speed * (rec.time - charge);
            if dist > rec.distance {
                count += 1;
            }
        }
        count.to_string()
    }

    fn description(&self) -> String {
        "AoC 2023/Day 6: Wait For It".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2023_06_ex1() {
        let sol = AoC2023_06 {
            #[rustfmt::skip]
            input: vec![
                Record { time: 7, distance: 9 },
                Record { time: 15, distance: 40 },
                Record { time: 30, distance: 200 },
            ],
        };
        assert_eq!("288", sol.part_one())
    }

    #[test]
    fn aoc2023_06_correctness() -> io::Result<()> {
        let sol = AoC2023_06::new()?;
        assert_eq!(sol.part_one(), "316800");
        assert_eq!(sol.part_two(), "45647654");
        Ok(())
    }
}
