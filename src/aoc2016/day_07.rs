use crate::solution::Solution;
use crate::utils::*;

use std::io;

pub struct AoC2016_07 {
    input: Vec<String>,
}

impl AoC2016_07 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: read_file_as_lines("input/aoc2016_07")?,
        })
    }
}

impl Solution for AoC2016_07 {
    fn part_one(&self) -> String {
        self.input
            .iter()
            .filter(|s| is_tls_supported(s))
            .count()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.input
            .iter()
            .filter(|s| is_ssl_supported(s))
            .count()
            .to_string()
    }

    fn description(&self) -> String {
        "AoC 2016/Day 7: Internet Protocol Version 7".to_string()
    }
}

#[allow(clippy::needless_range_loop)]
fn is_tls_supported(s: &str) -> bool {
    let arr = split_address(s);
    let mut result = false;
    for i in 0..arr.len() {
        let is_notated = is_abba_notated(&arr[i]);
        if i % 2 != 0 {
            if is_notated {
                return false;
            }
        } else {
            result |= is_notated;
        }
    }
    result
}

fn is_abba_notated(s: &str) -> bool {
    let len = s.len();
    if len < 4 {
        return false;
    }
    let chars = s.chars().collect::<Vec<char>>();
    for i in 0..=chars.len() - 4 {
        if chars[i] != chars[i + 1] && chars[i] == chars[i + 3] && chars[i + 1] == chars[i + 2] {
            return true;
        }
    }
    false
}

fn is_ssl_supported(s: &str) -> bool {
    let arr = split_address(s);
    for i in (0..arr.len()).step_by(2) {
        let chars = arr[i].chars().collect::<Vec<char>>();
        for ch in 0..=chars.len() - 3 {
            if chars[ch] != chars[ch + 1] && chars[ch] == chars[ch + 2] {
                let inv = [chars[ch + 1], chars[ch], chars[ch + 1]]
                    .iter()
                    .collect::<String>();
                for j in (1..arr.len()).step_by(2) {
                    if arr[j].contains(&inv) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn split_address(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut s = s;
    while let Some((p1, rest)) = s.split_once('[') {
        result.push(p1.to_string());
        let (p2, r) = rest
            .split_once(']')
            .expect("Close square bracket should be present");
        result.push(p2.to_string());
        s = r;
    }
    result.push(s.to_string());
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_07_input_load_test() -> io::Result<()> {
        let sol = AoC2016_07::new()?;
        assert!(sol.input.len() > 0);
        Ok(())
    }

    #[test]
    fn aoc2016_07_correctness() -> io::Result<()> {
        let sol = AoC2016_07::new()?;
        assert_eq!(sol.part_one(), "110");
        assert_eq!(sol.part_two(), "242");
        Ok(())
    }

    #[test]
    fn aoc2016_07_split() {
        let comps = split_address("kajqeqlafxtmzirn[mkftybdukmghmyoclxd]plvjnikiozkikifpodt[cmufoktkndkhaeqbztz]drjixnnsdxqnrmn[cmzsnhlirtskunngcee]upgxlcjhmoethppx");
        assert_eq!(comps[0], "kajqeqlafxtmzirn");
        assert_eq!(comps[1], "mkftybdukmghmyoclxd");
        assert_eq!(comps[2], "plvjnikiozkikifpodt");
        assert_eq!(comps[3], "cmufoktkndkhaeqbztz");
        assert_eq!(comps[4], "drjixnnsdxqnrmn");
        assert_eq!(comps[5], "cmzsnhlirtskunngcee");
        assert_eq!(comps[6], "upgxlcjhmoethppx");
        assert_eq!(comps.len(), 7);
    }

    #[test]
    fn aoc2016_07_abba_notate() {
        assert!(is_abba_notated("abba"));
        assert!(is_abba_notated("bddb"));
        assert!(is_abba_notated("xyyx"));
        assert!(!is_abba_notated("qwer"));
        assert!(!is_abba_notated("aaaa"));
    }

    #[test]
    fn aoc2016_07_ssl_support() {
        assert!(is_ssl_supported("aba[bab]xyz"));
        assert!(!is_ssl_supported("xyx[xyx]xyx"));
        assert!(is_ssl_supported("aaa[kek]eke"));
        assert!(is_ssl_supported("zazbz[bzb]cdb"));
    }
}
