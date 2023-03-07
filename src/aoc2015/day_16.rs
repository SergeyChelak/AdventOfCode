use crate::solution::Solution;
use crate::utils::*;

use std::io;

struct Profile {
    title: String,
    children: Option<i32>,
    cats: Option<i32>,
    samoyeds: Option<i32>,
    pomeranians: Option<i32>,
    akitas: Option<i32>,
    vizslas: Option<i32>,
    goldfish: Option<i32>,
    trees: Option<i32>,
    cars: Option<i32>,
    perfumes: Option<i32>
}

impl Profile {
    pub fn default(title: &str) -> Self {
        Self {
            title: title.to_string(),
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None
        }
    }

    pub fn from_str(s: &str) -> Self {
        let tokens = s.split_once(": ").expect("Separator not found");
        let mut profile = Profile::default(tokens.0);
        let tokens = tokens.1.split(", ").collect::<Vec<&str>>();
        for token in tokens {
            let pair = token.split(": ").collect::<Vec<&str>>();
            let property = pair[0];
            let value = pair[1].parse::<i32>().ok();
            match property {
                "children" => profile.children = value,
                "cats" => profile.cats = value,
                "samoyeds" => profile.samoyeds = value,
                "pomeranians" => profile.pomeranians = value,
                "akitas" => profile.akitas = value,
                "vizslas" => profile.vizslas = value,
                "goldfish" => profile.goldfish = value,
                "trees" => profile.trees = value,
                "cars" => profile.cars = value,
                "perfumes" => profile.perfumes = value,
                _ => panic!("unexpected property name: {}", property),
            }
        }
        profile
    }

    pub fn sender() -> Self {
        Self {
            title: String::from("Sender"),
            children: Some(3),
            cats: Some(7),
            samoyeds: Some(2),
            pomeranians: Some(3),
            akitas: Some(0),
            vizslas: Some(0),
            goldfish: Some(5),
            trees: Some(3),
            cars: Some(2),
            perfumes: Some(1)
        }
    }

    pub fn diff(&self, other: &Self) -> i32 {
        let mut result = 0;
        for (a, b) in self.prop_values().iter().zip(other.prop_values()) {
            match (a, b) {
                (Some(v1), Some(v2)) => if *v1 != v2 { return i32::MAX; },
                (Some(_), None) => result += 1,
                _ => return i32::MAX,
            }
        }
        result
    }

    pub fn fuzzy_diff(&self, other: &Self) -> i32 {
        let mut result = 0;
        for (idx, (a, b)) in self.prop_values().iter().zip(other.prop_values()).enumerate() {
            match (a, b) {
                (Some(v1), Some(v2)) if matches!(idx,  1 | 7) => if v2 <= *v1 { return i32::MAX; }
                (Some(v1), Some(v2)) if matches!(idx,  3 | 6) => if v2 >= *v1 { return i32::MAX; }
                (Some(v1), Some(v2)) => if *v1 != v2 { return i32::MAX; },
                (Some(_), None) => result += 1,
                _ => return i32::MAX,
            }
        }
        result
    }

    fn prop_values(&self) -> Vec<Option<i32>> {
        vec![
            self.children, // 0
            self.cats, // 1
            self.samoyeds, // 2
            self.pomeranians, // 3
            self.akitas, // 4
            self.vizslas, // 5
            self.goldfish, // 6
            self.trees, // 7
            self.cars, // 8
            self.perfumes, // 9
        ]
    }
}

pub struct AoC2015_16 {
    profiles: Vec<Profile>,
    sender: Profile,
}

impl AoC2015_16 {
    pub fn new() -> io::Result<Self> {
        let profiles = read_file_as_lines("input/aoc2015_16")?
            .iter()
            .map(|s| Profile::from_str(s))
            .collect::<Vec<Profile>>();
        Ok(Self {
            profiles,
            sender: Profile::sender(),
        })
    }

    fn find_best(&self, comparator: &(dyn Fn(&Profile, &Profile) -> i32)) -> String {
        let mut min = i32::MAX;
        let mut best = String::from("");
        for profile in &self.profiles {
            let diff = comparator(&self.sender, profile);
            if diff < min {
                min = diff;
                best = profile.title.clone();
            }
        }
        best
    }
}

impl Solution for AoC2015_16 {
    fn part_one(&self) -> String {
        self.find_best(&Profile::diff)
    }

    fn part_two(&self) -> String {
        self.find_best(&Profile::fuzzy_diff)
    }

    fn description(&self) -> String {
        "AoC 2015/Day 16: Aunt Sue".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_16_input_load_test() -> io::Result<()> {
        let sol = AoC2015_16::new()?;
        assert_eq!(sol.profiles.len(), 500);
        Ok(())
    }

    #[test]
    fn aoc2015_16_correctness() -> io::Result<()> {
        let sol = AoC2015_16::new()?;
        assert_eq!(sol.part_one(), "Sue 213");
        assert_eq!(sol.part_two(), "Sue 323");
        Ok(())
    }
}