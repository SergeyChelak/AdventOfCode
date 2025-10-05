use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

pub struct AoC2020_24 {
    input: Vec2<HexagonalDirection>,
}

impl AoC2020_24 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2020_24")?;
        Ok(Self::parse(&lines))
    }

    fn parse<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|s| s.as_ref())
            .map(Self::parse_directions)
            .collect::<Vec<_>>();

        Self { input }
    }

    fn parse_directions(line: &str) -> Vec<HexagonalDirection> {
        let mut result = Vec::new();
        let mut iter = line.chars().peekable();
        while iter.peek().is_some() {
            let ch1 = iter.next().expect("Can't be none");
            let mut ch2 = '\0';

            if ch1 == 's' || ch1 == 'n' {
                ch2 = iter.next().expect("Parsing error");
            }

            use HexagonalDirection::*;
            let val = match (ch1, ch2) {
                ('w', '\0') => West,
                ('e', '\0') => East,
                ('n', 'e') => NorthEast,
                ('n', 'w') => NorthWest,
                ('s', 'e') => SouthEast,
                ('s', 'w') => SouthWest,
                _ => panic!("Unexpected chars {ch1}{ch2}"),
            };
            result.push(val);
        }
        result
    }
}

impl Solution for AoC2020_24 {
    fn part_one(&self) -> String {
        let mut tiles: HashMap<HexagonalTile, bool> = HashMap::new();
        for directions in self.input.iter() {
            let tile = directions
                .iter()
                .fold(HexagonalTile::default(), |tile, dir| tile.moved_by(*dir));

            let entry = tiles.entry(tile).or_insert(false);
            *entry = !*entry;
        }

        tiles.values().filter(|x| **x).count().to_string()
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 24: Lobby Layout".to_string()
    }
}

type Int = isize;

#[derive(Debug, Clone, Copy)]
enum HexagonalDirection {
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct HexagonalTile {
    a: Int,
    r: Int,
    c: Int,
}

impl HexagonalTile {
    fn moved_by(&self, direction: HexagonalDirection) -> Self {
        match direction {
            HexagonalDirection::East => self.moved_east(),
            HexagonalDirection::West => self.moved_west(),
            HexagonalDirection::NorthEast => self.moved_north_east(),
            HexagonalDirection::NorthWest => self.moved_north_west(),
            HexagonalDirection::SouthEast => self.moved_south_east(),
            HexagonalDirection::SouthWest => self.moved_south_west(),
        }
    }

    fn moved_east(&self) -> Self {
        Self {
            a: self.a,
            r: self.r,
            c: self.c + 1,
        }
    }

    fn moved_west(&self) -> Self {
        Self {
            a: self.a,
            r: self.r,
            c: self.c - 1,
        }
    }

    fn moved_north_east(&self) -> Self {
        Self {
            a: 1 - self.a,
            r: self.r - (1 - self.a),
            c: self.c + self.a,
        }
    }

    fn moved_north_west(&self) -> Self {
        Self {
            a: 1 - self.a,
            r: self.r - (1 - self.a),
            c: self.c - (1 - self.a),
        }
    }

    fn moved_south_east(&self) -> Self {
        Self {
            a: 1 - self.a,
            r: self.r + self.a,
            c: self.c + self.a,
        }
    }

    fn moved_south_west(&self) -> Self {
        Self {
            a: 1 - self.a,
            r: self.r + self.a,
            c: self.c - (1 - self.a),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_24_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2020_24_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "230");
        Ok(())
    }

    #[test]
    fn aoc2020_24_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_24> {
        AoC2020_24::new()
    }

    #[test]
    fn aoc2020_24_case1() {
        let lines = [
            "sesenwnenenewseeswwswswwnenewsewsw",
            "neeenesenwnwwswnenewnwwsewnenwseswesw",
            "seswneswswsenwwnwse",
            "nwnwneseeswswnenewneswwnewseswneseene",
            "swweswneswnenwsewnwneneseenw",
            "eesenwseswswnenwswnwnwsewwnwsene",
            "sewnenenenesenwsewnenwwwse",
            "wenwwweseeeweswwwnwwe",
            "wsweesenenewnwwnwsenewsenwwsesesenwne",
            "neeswseenwwswnwswswnw",
            "nenwswwsewswnenenewsenwsenwnesesenew",
            "enewnwewneswsewnwswenweswnenwsenwsw",
            "sweneswneswneneenwnewenewwneswswnese",
            "swwesenesewenwneswnwwneseswwne",
            "enesenwswwswneneswsenwnewswseenwsese",
            "wnwnesenesenenwwnenwsewesewsesesew",
            "nenewswnwewswnenesenwnesewesw",
            "eneswnwswnwsenenwnwnwwseeswneewsenese",
            "neswnwewnwnwseenwseesewsenwsweewe",
            "wseweeenwnesenwwwswnew",
        ];
        let sol = AoC2020_24::parse(&lines);
        assert_eq!(sol.part_one(), "10");
    }
}
