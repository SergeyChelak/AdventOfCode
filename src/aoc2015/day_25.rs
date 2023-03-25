use crate::solution::Solution;

use std::io;

pub struct AoC2015_25 {
    row: usize,
    col: usize,
}

impl AoC2015_25 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            row: 2947,
            col: 3029,
        })
    }
}

fn calc_code(number: usize) -> usize {
    let mut value = 20151125;
    if number > 1 {
        for _ in 2..=number {
            value = value * 252533 % 33554393;
        }
    }
    value
}

fn position_to_number(row: usize, col: usize) -> usize {
    let mut num = 1;
    if row > 1 {
        for i in 2..=row {
            num += i - 1;
        }
    }
    if col > 1 {
        let mut val = row + 1;
        for _ in 2..=col {
            num += val;
            val += 1;
        }
    }
    num
}

impl Solution for AoC2015_25 {
    fn part_one(&self) -> String {
        calc_code(position_to_number(self.row, self.col))
            .to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2015/Day 25: Let It Snow".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_25_correctness() -> io::Result<()> {
        let sol = AoC2015_25::new()?;
        assert_eq!(sol.part_one(), "19980801");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2015_25_number_calc() {
        let matrix = matrix();
        assert_eq!(calc_code(1), matrix[0][0]);
        assert_eq!(calc_code(2), matrix[1][0]);
        assert_eq!(calc_code(3), matrix[0][1]);
        assert_eq!(calc_code(4), matrix[2][0]);
        assert_eq!(calc_code(5), matrix[1][1]);
        assert_eq!(calc_code(6), matrix[0][2]);
    }

    #[test]
    fn aoc2015_25_pos2num_rows() {
        assert_eq!(position_to_number(1, 1), 1);
        assert_eq!(position_to_number(2, 1), 2);
        assert_eq!(position_to_number(3, 1), 4);
        assert_eq!(position_to_number(4, 1), 7);
        assert_eq!(position_to_number(5, 1), 11);
        assert_eq!(position_to_number(6, 1), 16);
    }

    #[test]
    fn aoc2015_25_pos2num_cols() {
        assert_eq!(position_to_number(1, 1), 1);
        assert_eq!(position_to_number(1, 2), 3);
        assert_eq!(position_to_number(1, 3), 6);
        assert_eq!(position_to_number(1, 4), 10);
        assert_eq!(position_to_number(1, 5), 15);
        assert_eq!(position_to_number(1, 6), 21);
    }

    #[test]
    fn aoc2015_25_pos2num() {
        assert_eq!(position_to_number(2, 2), 5);
        assert_eq!(position_to_number(2, 3), 9);
        assert_eq!(position_to_number(2, 4), 14);
        assert_eq!(position_to_number(2, 5), 20);

        assert_eq!(position_to_number(3, 2), 8);
        assert_eq!(position_to_number(3, 3), 13);
        assert_eq!(position_to_number(3, 4), 19);

        assert_eq!(position_to_number(4, 2), 12);
        assert_eq!(position_to_number(4, 3), 18);

    }

    fn matrix() -> Vec<Vec<usize>> {
        vec![
            vec![20151125, 18749137, 17289845, 30943339, 10071777, 33511524],
            vec![31916031, 21629792, 16929656, 7726640,  15514188, 4041754],
            vec![16080970, 8057251,  1601130,  7981243,  11661866, 16474243],
            vec![24592653, 32451966, 21345942, 9380097,  10600672, 31527494],
            vec![77061,    17552253, 28094349, 6899651,  9250759,  31663883],
            vec![33071741, 6796745,  25397450, 24659492, 1534922,  27995004],
        ]
    }
}