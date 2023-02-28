use crate::solution::Solution;

use std::io;

pub struct AoC2015_10 {
    input: String
}

impl AoC2015_10 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            input: "3113322113".to_string()
        })
    }
}

fn look_say(s: &str) -> String {
    let mut prev = '\0';
    let mut count = 0usize;
    let mut buffer = Vec::new();
    let mut chars = s.chars().collect::<Vec<char>>();
    chars.push(prev);
    for ch in chars {
        if ch == prev {
            count += 1;
        } else {
            buffer.push(count.to_string());
            buffer.push(prev.to_string());
            prev = ch;
            count = 1;            
        }
    }
    buffer[2..].join("")
}

fn play_look_say(times: usize, initial: &str) -> String {
    let mut s = initial.to_string();
    for _ in 0..times {
        s = look_say(&s);
    }
    s.len().to_string()
}

impl Solution for AoC2015_10 {
    fn part_one(&self) -> String {
        play_look_say(40, &self.input)
    }

    fn part_two(&self) -> String {
        play_look_say(50, &self.input)
    }

    fn description(&self) -> String {
    	"AoC 2015/Day 10: Elves Look, Elves Say".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_10_look_say_test() {
        assert_eq!(look_say("1"), "11");
        assert_eq!(look_say("11"), "21");
        assert_eq!(look_say("21"), "1211");
        assert_eq!(look_say("1211"), "111221");
        assert_eq!(look_say("111221"), "312211");
    }

    #[test]
    fn aoc2015_10_correctness() -> io::Result<()> {
        let sol = AoC2015_10::new()?;
        assert_eq!(sol.part_one(), "329356");
        assert_eq!(sol.part_two(), "4666278");
        Ok(())
    }
}