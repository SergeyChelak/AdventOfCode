use crate::solution::Solution;
use crate::utils::*;

use std::cmp::Ordering;
use std::io;

struct RoomCode {
    encrypted_name: String,
    sector_id: i32,
    stored_checksum: String,
}

impl RoomCode {
    fn from_str(s: &str) -> Self {
        let components = s.split("-").collect::<Vec<&str>>();
        let last_idx = components.len() - 1;
        let name = components[..last_idx]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("-");
        let checksum_compound = components[last_idx];
        let split_index = checksum_compound
            .find('[')
            .expect("Input string should contain [ .. ]");
        let (sector_id, checksum) = checksum_compound.split_at(split_index);
        Self {
            encrypted_name: name,
            sector_id: sector_id
                .parse::<i32>()
                .expect("Sector id expected to be integer value"),
            stored_checksum: checksum[1..checksum.len() - 1].to_string(),
        }
    }

    fn calc_checksum(&self) -> String {
        let mut freq = [0i32; 26];
        let offset = 'a' as u8 as usize;
        for ch in self.encrypted_name.chars() {
            if !ch.is_alphabetic() {
                continue;
            }
            let index = ch as u8 as usize - offset;
            freq[index] += 1;
        }
        let mut char_data: Vec<(usize, i32)> = Vec::new();
        for i in 0..freq.len() {
            let val = freq[i];
            if val == 0 {
                continue;
            }
            char_data.push((i, val));
        }
        char_data.sort_by(|(a_code, a_freq), (b_code, b_freq)| {
            let ordering = b_freq.cmp(a_freq);
            if ordering == Ordering::Equal {
                a_code.cmp(b_code)
            } else {
                ordering
            }
        });
        char_data
            .iter()
            .take(5)
            .map(|(chr, _)| (chr + offset) as u8 as char)
            .collect::<String>()
    }

    fn is_valid_checksum(&self) -> bool {
        self.stored_checksum == self.calc_checksum()
    }

    fn decrypt_name(&self) -> String {
        let offset = 'a' as u8;
        let mut value = self.encrypted_name.clone();
        for _ in 0..self.sector_id {
            value = value
                .chars()
                .map(|ch| match ch {
                    'a'..='z' => {
                        let code = (ch as u8 - offset + 1) % 26;
                        (code + offset) as char
                    }
                    ' ' | '-' => ' ',
                    _ => ch,
                })
                .collect();
        }
        value
    }
}

pub struct AoC2016_04 {
    input: Vec<RoomCode>,
}

impl AoC2016_04 {
    pub fn new() -> io::Result<Self> {
        let input = read_file_as_lines("input/aoc2016_04")?
            .iter()
            .map(|s| RoomCode::from_str(s))
            .collect::<Vec<RoomCode>>();
        Ok(Self { input })
    }
}

impl Solution for AoC2016_04 {
    fn part_one(&self) -> String {
        self.input
            .iter()
            .filter(|&code| code.is_valid_checksum())
            .map(|code| code.sector_id)
            .sum::<i32>()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.input
            .iter()
            .filter(|&code| code.is_valid_checksum())
            .filter(|&code| code.decrypt_name() == "northpole object storage")
            .map(|code| code.sector_id.to_string())
            .take(1)
            .collect::<String>()
    }

    fn description(&self) -> String {
        "AoC 2016/Day 4: Security Through Obscurity".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_04_input_load_test() -> io::Result<()> {
        let sol = AoC2016_04::new()?;
        assert!(sol.input.len() > 0);
        Ok(())
    }

    #[test]
    fn aoc2016_04_correctness() -> io::Result<()> {
        let sol = AoC2016_04::new()?;
        assert_eq!(sol.part_one(), "409147");
        assert_eq!(sol.part_two(), "991");
        Ok(())
    }

    #[test]
    fn aoc2016_04_room_decode() {
        let room = RoomCode::from_str("aaaaa-bbb-z-y-x-123[abxyz]");
        assert_eq!(room.encrypted_name, "aaaaa-bbb-z-y-x");
        assert_eq!(room.sector_id, 123);
        assert_eq!(room.stored_checksum, "abxyz");
    }

    #[test]
    fn aoc2016_04_calc_checksum() {
        let room = RoomCode::from_str("aaaaa-bbb-z-y-x-123[abxyz]");
        assert_eq!(room.calc_checksum(), "abxyz");

        let room = RoomCode::from_str("a-b-c-d-e-f-g-h-987[abcde]");
        assert_eq!(room.calc_checksum(), "abcde");

        let room = RoomCode::from_str("not-a-real-room-404[oarel]");
        assert_eq!(room.calc_checksum(), "oarel");
    }

    #[test]
    fn aoc2016_04_decrypt() {
        let room = RoomCode::from_str("qzmt-zixmtkozy-ivhz-343[abxyz]");
        assert_eq!(room.decrypt_name(), "very encrypted name");
    }
}
