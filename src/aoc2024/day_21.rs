use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, VecDeque};
use std::{io, iter};

type Position = Position2<isize>;
type KeyMap = HashMap<Position, char>;

const NUMPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    ['#', '0', 'A'],
];

const DIR_PAD: [[char; 3]; 2] = [['#', '^', 'A'], ['<', 'v', '>']];

pub struct AoC2024_21 {
    codes: Vec<String>,
}

impl AoC2024_21 {
    pub fn new() -> io::Result<Self> {
        let codes = read_file_as_lines("input/aoc2024_21")?;
        Ok(Self { codes })
    }
}

impl Solution for AoC2024_21 {
    fn part_one(&self) -> String {
        self.codes
            .iter()
            .map(|code| solve(code, 2))
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.codes
            .iter()
            .map(|code| solve(code, 25))
            .sum::<usize>()
            .to_string()
    }

    fn description(&self) -> String {
        "2024/Day 21: Keypad Conundrum".to_string()
    }
}

type PathMap = HashMap<(char, char), Vec<String>>;

fn calculate_length(code: &str, depth: usize) -> usize {
    fn dfs(
        a: char,
        b: char,
        depth: usize,
        path_map: &PathMap,
        memo: &mut HashMap<(char, char, usize), usize>,
    ) -> usize {
        let key = (a, b, depth);
        if depth == 1 {
            return path_map
                .get(&(a, b))
                .and_then(|x| x.first())
                .expect("Direction pad path map created incorrectly")
                .len();
        }
        if let Some(val) = memo.get(&key) {
            return *val;
        }
        let mut result = usize::MAX;
        for path in path_map
            .get(&(a, b))
            .expect("Path wasn't calculated initially")
        {
            let mut acc = 0;
            for (x, y) in iter::once('A').chain(path.chars()).zip(path.chars()) {
                acc += dfs(x, y, depth - 1, path_map, memo);
            }
            result = result.min(acc);
        }
        memo.insert(key, result);
        result
    }

    let inputs = {
        let num_path_map = make_paths_map(&NUMPAD);
        code_to_direction(code, &num_path_map)
    };
    let dir_path_map = make_paths_map(&DIR_PAD);
    let mut memo = HashMap::new();

    let mut result = usize::MAX;

    for path in inputs {
        let mut acc = 0;
        for (x, y) in iter::once('A').chain(path.chars()).zip(path.chars()) {
            acc += dfs(x, y, depth, &dir_path_map, &mut memo);
        }
        result = result.min(acc);
    }

    result
}

fn solve(code: &str, depth: usize) -> usize {
    let len = calculate_length(code, depth);
    let l = code.len();
    len * code[..l - 1].parse::<usize>().expect("Invalid input")
}

fn code_to_direction(code: &str, path_map: &PathMap) -> Vec<String> {
    let input = iter::once('A')
        .chain(code.chars())
        .zip(code.chars())
        .filter_map(|(a, b)| path_map.get(&(a, b)))
        .cloned()
        .collect::<Vec<_>>();

    let input = cartesian_product(&input);
    let len = input.iter().map(|v| v.len()).min().unwrap_or_default();
    input
        .into_iter()
        .filter(|v| v.len() == len)
        .collect::<Vec<_>>()
}

fn make_paths_map<T: AsRef<[char]>>(pad: &[T]) -> PathMap {
    let map = make_map(pad);
    let mut paths = HashMap::new();
    for (k1, v1) in map.iter() {
        for (k2, v2) in map.iter() {
            let mut arr = find_paths(&map, *k1, *k2);
            arr.iter_mut().for_each(|s| s.push('A'));
            paths.insert((*v1, *v2), arr);
        }
    }
    paths
}

fn cartesian_product(inp: &[Vec<String>]) -> Vec<String> {
    fn dfs(inp: &[Vec<String>], current: String, section: usize, out: &mut Vec<String>) {
        if section == inp.len() {
            out.push(current);
            return;
        }
        for val in &inp[section] {
            let mut next = current.clone();
            next.push_str(val);
            dfs(inp, next, section + 1, out);
        }
    }

    let mut result = Vec::new();
    dfs(inp, String::new(), 0, &mut result);
    result
}

fn find_paths(map: &KeyMap, from: Position, to: Position) -> Vec<String> {
    let mut deque = VecDeque::new();
    deque.push_back(from);

    let mut path_map = HashMap::new();
    path_map.insert(from, vec![String::new()]);

    while let Some(elem) = deque.pop_front() {
        if elem == to {
            break;
        }
        let paths = path_map.get(&elem).expect("Bug (1)").clone();
        let len = paths.first().map(|x| x.len()).expect("Bug (2)");

        for (dr, dc, ch) in [(0, 1, '>'), (0, -1, '<'), (1, 0, 'v'), (-1, 0, '^')] {
            let next = Position::new(elem.row + dr, elem.col + dc);
            if !map.contains_key(&next) {
                continue;
            }
            let mut next_len = usize::MAX;
            if let Some(arr) = path_map.get(&next) {
                next_len = arr.first().map(|x| x.len()).expect("Bug (3)")
            }
            if next_len < 1 + len {
                continue;
            }

            let entry = path_map.entry(next).or_default();
            for s in paths.iter() {
                let mut p = s.clone();
                p.push(ch);
                if !entry.contains(&p) {
                    entry.push(p);
                }
            }

            deque.push_back(next);
        }
    }

    path_map.get(&to).cloned().unwrap_or_default()
}

fn make_map<T: AsRef<[char]>>(pad: &[T]) -> KeyMap {
    let mut map = KeyMap::new();
    for (r, row) in pad.iter().enumerate() {
        for (c, val) in row.as_ref().iter().enumerate() {
            if *val == '#' {
                continue;
            }
            let p = Position::new(r as isize, c as isize);
            map.insert(p, *val);
        }
    }
    map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_21_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(5, sol.codes.len());
        Ok(())
    }

    #[test]
    fn aoc2024_21_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "278568");
        Ok(())
    }

    #[test]
    fn aoc2024_21_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "341460772681012");
        Ok(())
    }

    #[test]
    fn aoc2024_21_ex1() {
        let aoc = AoC2024_21 {
            codes: vec!["029A".to_string()],
        };
        assert_eq!((68 * 29).to_string(), aoc.part_one())
    }

    #[test]
    fn aoc2024_21_case_1() {
        let aoc = AoC2024_21 {
            codes: vec![
                "029A".to_string(),
                "980A".to_string(),
                "179A".to_string(),
                "456A".to_string(),
                "379A".to_string(),
            ],
        };
        assert_eq!("126384", aoc.part_one())
    }

    #[test]
    fn aoc2024_21_make_pads() {
        assert_eq!(5, make_map(&DIR_PAD).len());
        assert_eq!(11, make_map(&NUMPAD).len());
    }

    fn make_solution() -> io::Result<AoC2024_21> {
        AoC2024_21::new()
    }
}
