use crate::solution::Solution;
use crate::utils::*;

use std::io;

type Chars = Vec<char>;

pub struct AoC2021_10 {
    input: Vec<Chars>,
}

impl AoC2021_10 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2021_10")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|x| x.as_ref())
            .map(|s| s.chars().collect::<Chars>())
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2021_10 {
    fn part_one(&self) -> String {
        self.input
            .iter()
            .map(get_error_points)
            .map(|(val, _)| val)
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        let mut result = self
            .input
            .iter()
            .filter_map(autocomplete_score)
            .collect::<Vec<_>>();
        result.sort();
        result[result.len() / 2].to_string()
    }

    fn description(&self) -> String {
        "Day 10: Syntax Scoring".to_string()
    }
}

fn autocomplete_score(input: &Chars) -> Option<usize> {
    let chars = autocomplete(input)?;
    let value = chars.iter().fold(0usize, |acc, ch| {
        acc * 5
            + match *ch {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!(),
            }
    });
    Some(value)
}

fn autocomplete(input: &Chars) -> Option<Chars> {
    let (err, stack) = get_error_points(input);
    if err > 0 {
        return None;
    }
    let completion = stack.iter().rev().map(|ch| opposite_bracket(*ch)).collect();
    Some(completion)
}

fn get_error_points(input: &Chars) -> (usize, Vec<char>) {
    let mut stack = Vec::with_capacity(input.len());
    let open = ['[', '(', '{', '<'];
    for ch in input {
        if open.contains(ch) {
            stack.push(*ch);
            continue;
        }
        let Some(peek) = stack.last() else {
            return (error_points(*ch), stack);
        };
        if *peek == opposite_bracket(*ch) {
            stack.pop();
        } else {
            return (error_points(*ch), stack);
        }
    }
    (0, stack)
}

fn error_points(ch: char) -> usize {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn opposite_bracket(ch: char) -> char {
    match ch {
        '{' => '}',
        '}' => '{',
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '<' => '>',
        '>' => '<',
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_10_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2021_10_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "271245");
        Ok(())
    }

    #[test]
    fn aoc2021_10_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "1685293086");
        Ok(())
    }

    #[test]
    fn aoc2021_10_autocomplete() {
        let result = autocomplete(&"[({(<(())[]>[[{[]{<()<>>".chars().collect());
        assert_eq!(result, Some("}}]])})]".chars().collect::<Vec<_>>()));
    }

    #[test]
    fn aoc2021_10_autocomplete_score() {
        let result = autocomplete_score(&"<{([{{}}[<[[[<>{}]]]>[]]".chars().collect());
        assert_eq!(result, Some(294));
    }

    fn make_solution() -> io::Result<AoC2021_10> {
        AoC2021_10::new()
    }
}
