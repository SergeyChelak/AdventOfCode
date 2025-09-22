use crate::solution::Solution;

use std::{io, ops::RangeInclusive};

#[derive(Debug, Default)]
struct Passport {
    byr: Option<String>, // (Birth Year)
    iyr: Option<String>, // (Issue Year)
    eyr: Option<String>, // (Expiration Year)
    hgt: Option<String>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, // (Passport ID)
}

impl Passport {
    fn is_valid_by_present(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid_by_value(&self) -> bool {
        self.is_byr_valid()
            && self.is_iyr_valid()
            && self.is_eyr_valid()
            && self.is_hgt_valid()
            && self.is_hcl_valid()
            && self.is_ecl_valid()
            && self.is_pid_valid()
    }

    fn is_byr_valid(&self) -> bool {
        Self::is_year_valid(&self.byr, 1920, 2002)
    }

    fn is_iyr_valid(&self) -> bool {
        Self::is_year_valid(&self.iyr, 2010, 2020)
    }

    fn is_eyr_valid(&self) -> bool {
        Self::is_year_valid(&self.eyr, 2020, 2030)
    }

    fn is_hcl_valid(&self) -> bool {
        let Some(clr) = &self.hcl else {
            return false;
        };
        if !clr.starts_with("#") || clr.len() != 7 {
            return false;
        }
        clr[1..]
            .chars()
            .all(|ch| ch.is_numeric() || ('a'..='f').contains(&ch))
    }

    fn is_ecl_valid(&self) -> bool {
        let Some(clr) = &self.ecl else {
            return false;
        };
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&clr.as_str())
    }

    fn is_hgt_valid(&self) -> bool {
        let Some(height) = &self.hgt else {
            return false;
        };

        let mut range: Option<RangeInclusive<usize>> = None;
        if height.ends_with("cm") {
            range = Some(150..=193);
        }
        if height.ends_with("in") {
            range = Some(59..=76)
        }

        let len = height.len();
        if len < 2 {
            return false;
        }
        let Ok(val) = height[..len - 2].parse::<usize>() else {
            return false;
        };
        range.map(|r| r.contains(&val)).unwrap_or(false)
    }

    fn is_pid_valid(&self) -> bool {
        let Some(num) = &self.pid else {
            return false;
        };
        num.chars().filter(|x| x.is_numeric()).count() == 9
    }

    fn is_year_valid(val: &Option<String>, low: usize, high: usize) -> bool {
        let Some(byr) = val else {
            return false;
        };
        let Ok(year) = byr.parse::<usize>() else {
            return false;
        };
        (low..=high).contains(&year)
    }
}

impl From<&str> for Passport {
    fn from(value: &str) -> Self {
        let mut passport = Passport::default();
        for s in value
            .split("\n")
            .filter(|x| !x.is_empty())
            .flat_map(|x| x.split(" "))
        {
            let (key, value) = s.split_once(":").expect("Invalid pair format");
            let value = value.to_string();
            match key {
                "byr" => passport.byr = Some(value),
                "iyr" => passport.iyr = Some(value),
                "eyr" => passport.eyr = Some(value),
                "hgt" => passport.hgt = Some(value),
                "hcl" => passport.hcl = Some(value),
                "ecl" => passport.ecl = Some(value),
                "pid" => passport.pid = Some(value),
                _ => continue,
            }
        }
        passport
    }
}

pub struct AoC2020_04 {
    input: Vec<Passport>,
}

impl AoC2020_04 {
    pub fn new() -> io::Result<Self> {
        let input = std::fs::read_to_string("input/aoc2020_04")?;
        Ok(Self::parse(&input))
    }

    pub fn parse(s: &str) -> Self {
        let input = s.split("\n\n").map(Passport::from).collect();
        Self { input }
    }

    fn valid_count(&self, criteria: impl Fn(&Passport) -> bool) -> String {
        self.input
            .iter()
            .filter(|x| criteria(x))
            .count()
            .to_string()
    }
}

impl Solution for AoC2020_04 {
    fn part_one(&self) -> String {
        self.valid_count(|x| x.is_valid_by_present())
    }

    fn part_two(&self) -> String {
        self.valid_count(|x| x.is_valid_by_value())
    }

    fn description(&self) -> String {
        "Day 4: Passport Processing".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_04_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2020_04_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "196");
        Ok(())
    }

    #[test]
    fn aoc2020_04_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "114");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_04> {
        AoC2020_04::new()
    }

    #[test]
    fn aoc2020_04_case_1() {
        let input = r#"
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"#;
        let sol = AoC2020_04::parse(input);
        assert_eq!(sol.part_one(), "2");
    }

    #[test]
    fn aoc2020_04_invalid() {
        let input = "
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        let sol = AoC2020_04::parse(input);
        assert!(sol.input.iter().all(|x| !x.is_valid_by_value()));
    }

    #[test]
    fn aoc2020_04_valid() {
        let input = "
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
";
        let sol = AoC2020_04::parse(input);
        assert!(sol.input.iter().all(|x| x.is_valid_by_value()));
    }
}
