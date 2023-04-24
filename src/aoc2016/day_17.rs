use crate::solution::Solution;

use std::io;

pub struct AoC2016_17 {
    prefix: String,
}

impl AoC2016_17 {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            prefix: "pvhmgsws".to_string(),
        })
    }
}

impl Solution for AoC2016_17 {
    fn part_one(&self) -> String {
        find_path(&self.prefix).expect("Path to exit should exist")
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "AoC 2016/Day 17: Two Steps Forward".to_string()
    }
}

struct Room {
    x: i32,
    y: i32,
    path: String,
}

impl Room {
    fn new() -> Self {
        Room {
            x: 0,
            y: 0,
            path: "".to_string(),
        }
    }

    fn adjacent_rooms(&self, prefix: &str) -> Vec<Self> {
        let chars = {
            let hash_input = format!("{prefix}{}", self.path);
            let mut hash = format!("{:x}", md5::compute(hash_input));
            hash.truncate(4);
            hash.chars().collect::<Vec<char>>()
        };
        let mut adjacent = Vec::with_capacity(4);
        // up, down, left, and right
        for (i, ch) in chars.iter().enumerate().take(4) {
            if !('b'..='f').contains(ch) {
                continue;
            }
            let room = match i {
                0 => {
                    // up
                    if self.y > 0 {
                        Some(Self {
                            x: self.x,
                            y: self.y - 1,
                            path: format!("{}U", self.path),
                        })
                    } else {
                        None
                    }
                }
                1 => {
                    // down
                    if self.y < 3 {
                        Some(Self {
                            x: self.x,
                            y: self.y + 1,
                            path: format!("{}D", self.path),
                        })
                    } else {
                        None
                    }
                }
                2 => {
                    // left
                    if self.x > 0 {
                        Some(Self {
                            x: self.x - 1,
                            y: self.y,
                            path: format!("{}L", self.path),
                        })
                    } else {
                        None
                    }
                }
                3 => {
                    // right
                    if self.x < 3 {
                        Some(Self {
                            x: self.x + 1,
                            y: self.y,
                            path: format!("{}R", self.path),
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            };
            if let Some(room) = room {
                adjacent.push(room);
            }
        }
        adjacent
    }

    fn is_exit(&self) -> bool {
        self.x == 3 && self.y == 3
    }
}

fn find_path(prefix: &str) -> Option<String> {
    let mut locations = vec![Room::new()];
    while !locations.is_empty() {
        let mut next_locations = Vec::with_capacity(2 * locations.len());
        for loc in &locations {
            let mut adjacent = loc.adjacent_rooms(prefix);
            let exit_cells = adjacent
                .iter()
                .filter(|room| room.is_exit())
                .collect::<Vec<&Room>>();
            if let Some(room) = exit_cells.first() {
                return Some(room.path.clone());
            }
            next_locations.append(&mut adjacent);
        }
        locations = next_locations;
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2016_17_correctness() -> io::Result<()> {
        let sol = AoC2016_17::new()?;
        assert_eq!(sol.part_one(), "DRRDRLDURD");
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    #[test]
    fn aoc2016_17_examples() {
        assert_eq!(find_path("ihgpwlah").unwrap(), "DDRRRD");
        assert_eq!(find_path("kglvqrro").unwrap(), "DDUDRLRRUDRD");
        assert_eq!(
            find_path("ulqzkmiv").unwrap(),
            "DRURDRUDDLLDLUURRDULRLDUUDDDRR"
        );
    }
}
