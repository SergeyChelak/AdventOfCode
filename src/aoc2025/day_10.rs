use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, VecDeque};
use std::io;

struct MachineConfiguration {
    indicators: usize,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

pub struct AoC2025_10 {
    input: Vec<MachineConfiguration>,
    // -- temp
    machines: Vec<Machine>,
}

impl AoC2025_10 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2025_10")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(MachineConfiguration::from)
            .collect();

        let machines = parse(lines);

        Self { input, machines }
    }
}

impl Solution for AoC2025_10 {
    fn part_one(&self) -> String {
        self.input
            .iter()
            .filter_map(|x| indicator_setup_presses(x.indicators, &x.buttons))
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.machines.iter().map(solve_lp).sum::<i64>().to_string()
    }

    fn description(&self) -> String {
        "Day 10: Factory".to_string()
    }
}

fn bitmap(arr: &[usize]) -> usize {
    arr.iter().fold(0, |acc, x| acc | (1 << *x))
}

fn indicator_setup_presses(target: usize, buttons: &[Vec<usize>]) -> Option<usize> {
    let bitmaps = buttons.iter().map(|arr| bitmap(arr)).collect::<Vec<_>>();
    let mut queue = VecDeque::new();
    queue.push_back(0usize);
    let mut count_map = HashMap::<usize, usize>::new();
    count_map.insert(0usize, 0);
    while let Some(state) = queue.pop_back() {
        let count = 1 + count_map.get(&state).expect("Value must be preserved");
        for bitmap in &bitmaps {
            let next = state ^ *bitmap;
            if count_map.contains_key(&next) {
                continue;
            }
            if next == target {
                return Some(count);
            }
            count_map.insert(next, count);
            queue.push_front(next);
        }
    }
    None
}

impl From<&str> for MachineConfiguration {
    fn from(value: &str) -> Self {
        let mut iter = value.split(' ');
        let indicators =
            parse_indicators(iter.next().expect("Empty str for machine configuration"));
        let joltage = parse_csv(
            iter.next_back()
                .expect("Not enough data for machine config"),
        );
        let buttons = iter.map(parse_csv).collect::<Vec<_>>();
        Self {
            indicators,
            buttons,
            joltage,
        }
    }
}

fn parse_csv(s: &str) -> Vec<usize> {
    remove_first_and_last(s)
        .split(',')
        .map(|val| val.parse::<usize>().expect("Invalid input format"))
        .collect()
}

fn parse_indicators(s: &str) -> usize {
    let mut iter = s.chars();
    iter.next();
    iter.next_back();
    iter.rev()
        .fold(0usize, |acc, ch| acc << 1 | if ch == '#' { 1 } else { 0 })
}

// -------------------- BEGIN --------------------

// based on soltuion by u/RussellDash332
// https://www.reddit.com/r/adventofcode/comments/1pity70/comment/nt988z4/?context=3
pub struct Machine {
    pub goal_mask: u32,
    pub goal_counters: Vec<i64>,
    pub button_masks: Vec<u32>,
}

pub fn parse<T: AsRef<str>>(lines: &[T]) -> Vec<Machine> {
    lines
        .iter()
        .map(|x| x.as_ref())
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            let goal_str = &parts[0][1..parts[0].len() - 1];
            let mut goal_mask = 0;
            for (i, c) in goal_str.chars().enumerate() {
                if c == '#' {
                    goal_mask |= 1 << i;
                }
            }

            let last_part = parts.last().unwrap();
            let counter_str = &last_part[1..last_part.len() - 1];
            let goal_counters = counter_str
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect();

            let mut button_masks = Vec::new();
            for part in &parts[1..parts.len() - 1] {
                let mut mask = 0;
                let inner = if part.starts_with('(') || part.starts_with('{') {
                    &part[1..part.len() - 1]
                } else {
                    part
                };

                for num_str in inner.split(',') {
                    if let Ok(bit) = num_str.parse::<u32>() {
                        mask |= 1 << bit;
                    }
                }
                button_masks.push(mask);
            }

            Machine {
                goal_mask,
                goal_counters,
                button_masks,
            }
        })
        .collect()
}

const INF: f64 = f64::INFINITY;
const EPS: f64 = 1e-9;

fn simplex(lhs: &[Vec<f64>], c: &[f64]) -> (f64, Option<Vec<f64>>) {
    let m = lhs.len();
    let n = lhs[0].len() - 1;

    let mut n_indices: Vec<i32> = (0..n as i32).collect();
    n_indices.push(-1);

    let mut b_indices: Vec<i32> = (n as i32..(n + m) as i32).collect();

    let mut d = vec![vec![0.0; n + 2]; m + 2];

    for (d_row, lhs_row) in d.iter_mut().zip(lhs.iter()) {
        d_row[..=n].copy_from_slice(lhs_row);
        d_row[n + 1] = -1.0;
    }

    for row in d.iter_mut().take(m) {
        row.swap(n, n + 1);
    }

    d[m][..n].copy_from_slice(&c[..n]);
    d[m + 1][n] = 1.0;

    let pivot =
        |d: &mut Vec<Vec<f64>>, b_idx: &mut Vec<i32>, n_idx: &mut Vec<i32>, r: usize, s: usize| {
            let k = 1.0 / d[r][s];

            for i in 0..m + 2 {
                if i == r {
                    continue;
                }
                for j in 0..n + 2 {
                    if j != s {
                        d[i][j] -= d[r][j] * d[i][s] * k;
                    }
                }
            }

            for val in d[r].iter_mut() {
                *val *= k;
            }
            for row in d.iter_mut() {
                row[s] *= -k;
            }
            d[r][s] = k;

            std::mem::swap(&mut b_idx[r], &mut n_idx[s]);
        };

    let find =
        |d: &mut Vec<Vec<f64>>, b_idx: &mut Vec<i32>, n_idx: &mut Vec<i32>, p_idx: usize| -> bool {
            loop {
                let mut best_s = usize::MAX;
                let mut best_val = (INF, i32::MAX);

                for i in 0..=n {
                    if p_idx != 0 || n_idx[i] != -1 {
                        let val = d[m + p_idx][i];
                        let key = (val, n_idx[i]);
                        if best_s == usize::MAX
                            || key.0 < best_val.0 - EPS
                            || ((key.0 - best_val.0).abs() <= EPS && key.1 < best_val.1)
                        {
                            best_s = i;
                            best_val = key;
                        }
                    }
                }
                let s = best_s;

                if d[m + p_idx][s] > -EPS {
                    return true;
                }

                let mut best_r = usize::MAX;
                let mut best_r_key = (INF, i32::MAX);

                for i in 0..m {
                    if d[i][s] > EPS {
                        let ratio = d[i][n + 1] / d[i][s];
                        let key = (ratio, b_idx[i]);
                        if best_r == usize::MAX
                            || key.0 < best_r_key.0 - EPS
                            || ((key.0 - best_r_key.0).abs() <= EPS && key.1 < best_r_key.1)
                        {
                            best_r = i;
                            best_r_key = key;
                        }
                    }
                }
                let r = best_r;

                if r == usize::MAX {
                    return false;
                }

                pivot(d, b_idx, n_idx, r, s);
            }
        };

    let mut split_r = 0;
    let mut min_val = d[0][n + 1];
    for (i, row) in d.iter().enumerate().take(m).skip(1) {
        if row[n + 1] < min_val {
            min_val = row[n + 1];
            split_r = i;
        }
    }

    if d[split_r][n + 1] < -EPS {
        pivot(&mut d, &mut b_indices, &mut n_indices, split_r, n);
        if !find(&mut d, &mut b_indices, &mut n_indices, 1) || d[m + 1][n + 1] < -EPS {
            return (-INF, None);
        }
        for i in 0..m {
            if b_indices[i] == -1 {
                let mut best_s = 0;
                let mut best_key = (d[i][0], n_indices[0]);
                for j in 1..n {
                    let key = (d[i][j], n_indices[j]);
                    if key.0 < best_key.0 - EPS
                        || ((key.0 - best_key.0).abs() <= EPS && key.1 < best_key.1)
                    {
                        best_s = j;
                        best_key = key;
                    }
                }
                pivot(&mut d, &mut b_indices, &mut n_indices, i, best_s);
            }
        }
    }

    if find(&mut d, &mut b_indices, &mut n_indices, 0) {
        let mut x = vec![0.0; n];
        for i in 0..m {
            if b_indices[i] >= 0 && (b_indices[i] as usize) < n {
                x[b_indices[i] as usize] = d[i][n + 1];
            }
        }
        let mut sum_val = 0.0;
        for i in 0..n {
            sum_val += c[i] * x[i];
        }
        return (sum_val, Some(x));
    }

    (-INF, None)
}

fn solve_ilp_bnb(initial_a: Vec<Vec<f64>>, obj_coeffs: &[f64]) -> i64 {
    let mut best_val = INF;
    let mut stack = Vec::new();
    stack.push(initial_a);

    while let Some(current_a) = stack.pop() {
        let (val, x_opt) = simplex(&current_a, obj_coeffs);

        if val == -INF || val >= best_val - EPS {
            continue;
        }

        let mut fractional_idx = None;
        let mut fractional_val = 0.0;

        if let Some(x) = x_opt {
            for (i, &xv) in x.iter().enumerate() {
                if (xv - xv.round()).abs() > EPS {
                    fractional_idx = Some(i);
                    fractional_val = xv;
                    break;
                }
            }

            if let Some(idx) = fractional_idx {
                let floor_v = fractional_val.floor();
                let n_cols = current_a[0].len();

                let mut row1 = vec![0.0; n_cols];
                row1[idx] = 1.0;
                row1[n_cols - 1] = floor_v;
                let mut a1 = current_a.clone();
                a1.push(row1);
                stack.push(a1);

                let ceil_v = fractional_val.ceil();
                let mut row2 = vec![0.0; n_cols];
                row2[idx] = -1.0;
                row2[n_cols - 1] = -ceil_v;
                let mut a2 = current_a.clone();
                a2.push(row2);
                stack.push(a2);
            } else if val < best_val {
                best_val = val;
            }
        }
    }

    if best_val == INF {
        0
    } else {
        best_val.round() as i64
    }
}

fn solve_lp(machine: &Machine) -> i64 {
    let num_goals = machine.goal_counters.len();
    let num_buttons = machine.button_masks.len();

    let rows = 2 * num_goals + num_buttons;
    let cols = num_buttons + 1;

    let mut matrix = vec![vec![0.0; cols]; rows];

    for (j, row) in matrix.iter_mut().rev().take(num_buttons).enumerate() {
        row[j] = -1.0;
    }

    for (j, &mask) in machine.button_masks.iter().enumerate() {
        for i in 0..num_goals {
            if (mask >> i) & 1 == 1 {
                matrix[i][j] = 1.0;
                matrix[i + num_goals][j] = -1.0;
            }
        }
    }

    for i in 0..num_goals {
        let val = machine.goal_counters[i] as f64;
        matrix[i][cols - 1] = val;
        matrix[i + num_goals][cols - 1] = -val;
    }

    let obj_coeffs = vec![1.0; num_buttons];
    solve_ilp_bnb(matrix, &obj_coeffs)
}

// -------------------- END --------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2025_10_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2025_10_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "498");
        Ok(())
    }

    #[test]
    fn aoc2025_10_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "17133");
        Ok(())
    }

    #[test]
    fn aoc2025_10_bitmap_button_test() {
        assert_eq!(0b1000, bitmap(&[3]));
        assert_eq!(0b1010, bitmap(&[1, 3]));
        assert_eq!(0b101, bitmap(&[0, 2]));
    }

    #[test]
    fn aoc2025_10_parse_machine_config() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let config = MachineConfiguration::from(input);
        assert_eq!(config.indicators, 0b110);
        assert_eq!(config.joltage, [3, 5, 4, 7]);
        assert_eq!(config.buttons.len(), 6);
        assert_eq!(config.buttons[0], [3]);
        assert_eq!(config.buttons[1], [1, 3]);
        assert_eq!(config.buttons[2], [2]);
        assert_eq!(config.buttons[3], [2, 3]);
        assert_eq!(config.buttons[4], [0, 2]);
        assert_eq!(config.buttons[5], [0, 1]);
    }

    fn make_solution() -> io::Result<AoC2025_10> {
        AoC2025_10::new()
    }
}
