use crate::solution::Solution;
use crate::utils::*;

use std::io;

struct RoomCode {
    encrypted_name: String,
    sector_id: i32,
    stored_checksum: String
}

impl RoomCode {
    fn from_str(s: &str) -> Self {
        let components = s.split("-").collect::<Vec<&str>>();
        let last_idx = components.len() - 1;
        let name = components[..last_idx]
                .iter()
                .map(|s| s.to_string())
                .collect::<String>();
        let checksum_compound = components[last_idx];
        let split_index = checksum_compound.find('[').expect("Input string should contain [ .. ]");
        let (sector_id, checksum) = checksum_compound.split_at(split_index);
        Self {
            encrypted_name: name,
            sector_id: sector_id.parse::<i32>().expect("Sector id expected to be integer value"),
            stored_checksum: checksum[1..checksum.len() - 1].to_string(),
        }
    }

    fn calc_checksum(&self) -> String {
        todo!()
    }
}

pub struct AoC2016_04 {
    input: Vec<RoomCode>
}

impl AoC2016_04 {
    pub fn new() -> io::Result<Self> {
        let input = read_file_as_lines("input/aoc2016_04")?
            .iter()
            .map(|s| RoomCode::from_str(s))
            .collect::<Vec<RoomCode>>();
        Ok(Self {
            input
        })
    }
}

impl Solution for AoC2016_04 {
    // fn part_one(&self) -> String {
    // }

    // fn part_two(&self) -> String {
    // }

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
        assert_eq!(sol.part_one(), "");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2016_04_room_decode() {
        let room = RoomCode::from_str("aaaaa-bbb-z-y-x-123[abxyz]");
        assert_eq!(room.encrypted_name, "aaaaabbbzyx");
        assert_eq!(room.sector_id, 123);
        assert_eq!(room.stored_checksum, "abxyz");
    }
}