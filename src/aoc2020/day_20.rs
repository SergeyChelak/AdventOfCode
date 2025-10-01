use crate::{
    solution::Solution,
    utils::{not_found, Vec2},
};

use std::{
    collections::{HashMap, HashSet},
    io,
};

struct TileRaw {
    id: usize,
    data: Vec2<char>,
}

impl TileRaw {
    fn edges(&self) -> Vec<TileEdge> {
        let mut collection = HashSet::new();
        let mut current = TileEdge::from(self);
        for _ in 0..4 {
            let next = current.rotated90();
            let flip_ud = next.flipped_up_down();
            let flip_lf = next.flipped_left_right();
            collection.insert(next.clone());
            collection.insert(flip_lf);
            collection.insert(flip_ud);

            current = next;
        }
        collection.into_iter().collect()
    }

    fn col_chars(&self, col: usize) -> Vec<char> {
        self.data.iter().map(|arr| arr[col]).collect::<Vec<_>>()
    }

    fn row_chars(&self, row: usize) -> Vec<char> {
        self.data[row].clone()
    }

    fn rows(&self) -> usize {
        self.data.len()
    }

    fn cols(&self) -> usize {
        self.data.first().map(|elem| elem.len()).unwrap_or(0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TileEdge {
    top: u16,
    right: u16,
    bottom: u16,
    left: u16,
}

impl TileEdge {
    fn rotated90(&self) -> Self {
        Self {
            top: reverse_bits(self.left),
            right: self.top,
            bottom: reverse_bits(self.right),
            left: self.bottom,
        }
    }

    fn flipped_up_down(&self) -> Self {
        Self {
            top: self.bottom,
            right: reverse_bits(self.right),
            bottom: self.top,
            left: reverse_bits(self.left),
        }
    }

    fn flipped_left_right(&self) -> Self {
        Self {
            top: reverse_bits(self.top),
            right: self.left,
            bottom: reverse_bits(self.bottom),
            left: self.right,
        }
    }
}

impl From<&TileRaw> for TileEdge {
    fn from(value: &TileRaw) -> Self {
        let top = number_from_chars(&value.row_chars(0));
        let bottom = number_from_chars(&value.row_chars(value.rows() - 1));

        let left = number_from_chars(&value.col_chars(0));
        let right = number_from_chars(&value.col_chars(value.cols() - 1));

        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

const TILE_SIZE: usize = 10;

fn number_from_chars(chars: &[char]) -> u16 {
    assert!(chars.len() == TILE_SIZE);
    chars
        .iter()
        .map(|ch| if *ch == '#' { 1 } else { 0 })
        .rev()
        .fold(0u16, |acc, val| acc << 1 | val)
}

fn reverse_bits(mut number: u16) -> u16 {
    let mut result = 0;
    for _ in 0..TILE_SIZE {
        result = result << 1 | number & 1;
        number >>= 1;
    }
    result
}

impl From<&str> for TileRaw {
    fn from(value: &str) -> Self {
        let lines = value.split('\n').map(|s| s.trim()).collect::<Vec<_>>();

        assert!(lines.len() > 1, "value: '{value}'");

        let id = {
            let prefix = "Tile ";
            let s = lines[0];
            assert!(s.starts_with(prefix) && s.ends_with(':'));
            s[prefix.len()..s.len() - 1]
                .parse::<usize>()
                .expect("Invalid id")
        };

        let data = lines[1..]
            .iter()
            .map(|s| s.trim().chars().collect::<Vec<_>>())
            .inspect(|chars| assert!(!chars.is_empty()))
            .collect::<Vec<_>>();

        assert_eq!(data[0].len(), TILE_SIZE);
        assert_eq!(data.len(), TILE_SIZE);

        Self { id, data }
    }
}

#[derive(Debug, Default)]
struct Constraints {
    top: Option<u16>,
    right: Option<u16>,
    bottom: Option<u16>,
    left: Option<u16>,
}

impl Constraints {
    fn is_satisfy(&self, edge: &TileEdge) -> bool {
        let mut result = true;

        if let Some(top) = self.top {
            result &= top == edge.top;
        }

        if let Some(right) = self.right {
            result &= right == edge.right;
        }

        if let Some(bottom) = self.bottom {
            result &= bottom == edge.bottom;
        }

        if let Some(left) = self.left {
            result &= left == edge.left;
        }

        result
    }
}

#[derive(Debug, Clone)]
struct Disposition {
    tile_id: usize,
    edges: TileEdge,
}

struct Matrix {
    store: Vec2<Option<Disposition>>,
    dim: usize,
}

impl Matrix {
    fn new(dim: usize) -> Self {
        Self {
            store: vec![vec![None; dim]; dim],
            dim,
        }
    }

    fn encode_pos(&self, pos: usize) -> (usize, usize) {
        (
            pos / self.dim, // row
            pos % self.dim, // col
        )
    }

    fn at_end(&self, pos: usize) -> bool {
        pos >= self.dim * self.dim
    }

    fn set(&mut self, pos: usize, value: Disposition) {
        let (row, col) = self.encode_pos(pos);
        self.store[row][col] = Some(value);
    }

    fn remove(&mut self, pos: usize) {
        let (row, col) = self.encode_pos(pos);
        self.store[row][col] = None;
    }

    fn constraints(&self, pos: usize) -> Constraints {
        let (row, col) = self.encode_pos(pos);

        let top = if row > 0 {
            self.store[row - 1][col]
                .as_ref()
                .map(|elem| elem.edges.bottom)
        } else {
            None
        };

        let left = if col > 0 {
            self.store[row][col - 1]
                .as_ref()
                .map(|elem| elem.edges.right)
        } else {
            None
        };

        let bottom = if row < self.store.len() - 1 {
            self.store[row + 1][col].as_ref().map(|elem| elem.edges.top)
        } else {
            None
        };

        let right = if col < self.store[row].len() - 1 {
            self.store[row][col + 1]
                .as_ref()
                .map(|elem| elem.edges.left)
        } else {
            None
        };

        Constraints {
            top,
            right,
            bottom,
            left,
        }
    }

    fn product(&self) -> Option<usize> {
        let val = self.store[0][0].as_ref()?.tile_id
            * self.store[0][self.dim - 1].as_ref()?.tile_id
            * self.store[self.dim - 1][0].as_ref()?.tile_id
            * self.store[self.dim - 1][self.dim - 1].as_ref()?.tile_id;
        Some(val)
    }
}

struct TileDataStore {
    edges: HashMap<usize, Vec<TileEdge>>,
    // edges: Vec<(usize, Vec<TileEdge>)>,
}

impl TileDataStore {
    fn new() -> Self {
        TileDataStore {
            edges: HashMap::new(),
            // edges: Vec::new(),
        }
    }

    fn with(raw_tiles: &[TileRaw]) -> Self {
        let mut store = Self::new();
        store.fill_store(raw_tiles);
        store
    }

    fn fill_store(&mut self, raw_tiles: &[TileRaw]) {
        raw_tiles.iter().for_each(|raw| {
            self.edges.insert(raw.id, raw.edges());
            // self.edges.push((raw.id, raw.edges()));
        });
    }

    fn find(
        &self,
        constraints: &Constraints,
        exclude_set: &HashSet<usize>,
    ) -> Vec<(usize, TileEdge)> {
        let mut result = Vec::new();
        for (k, v) in self.edges.iter() {
            if exclude_set.contains(k) {
                continue;
            }
            for edge in v {
                if !constraints.is_satisfy(edge) {
                    continue;
                }
                result.push((*k, edge.clone()));
            }
        }
        result
    }
}

pub struct AoC2020_20 {
    tiles: Vec<TileRaw>,
}

fn dfs(
    tile_store: &TileDataStore,
    matrix: &mut Matrix,
    pos: usize,
    in_use: &mut HashSet<usize>,
) -> bool {
    if matrix.at_end(pos) {
        return true;
    }

    let constraints = matrix.constraints(pos);
    let options = tile_store.find(&constraints, in_use);

    for (id, edges) in options {
        in_use.insert(id);
        let disp = Disposition { tile_id: id, edges };
        matrix.set(pos, disp);
        if dfs(tile_store, matrix, pos + 1, in_use) {
            return true;
        }
        matrix.remove(pos);
        in_use.remove(&id);
    }

    false
}

impl AoC2020_20 {
    pub fn new() -> io::Result<Self> {
        let input = std::fs::read_to_string("input/aoc2020_20")?;
        Ok(Self::parse(&input))
    }

    fn parse(input: &str) -> Self {
        let tiles = input
            .trim()
            .split("\n\n")
            .map(TileRaw::from)
            .collect::<Vec<_>>();
        Self { tiles }
    }
}

impl Solution for AoC2020_20 {
    fn part_one(&self) -> String {
        let dim = self.tiles.len().isqrt();
        assert_eq!(dim * dim, self.tiles.len());

        let tile_store = TileDataStore::with(&self.tiles);
        let mut matrix = Matrix::new(dim);
        let is_ok = dfs(&tile_store, &mut matrix, 0, &mut HashSet::new());

        assert!(is_ok);

        matrix
            .product()
            .map(|val| val.to_string())
            .unwrap_or(not_found())
    }

    // fn part_two(&self) -> String {
    // }

    fn description(&self) -> String {
        "Day 20: Jurassic Jigsaw".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020_20_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.tiles.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2020_20_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "60145080587029");
        Ok(())
    }

    #[test]
    fn aoc2020_20_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2020_20> {
        AoC2020_20::new()
    }

    #[test]
    fn aoc2020_20_bit_manipulation() {
        let inp = "..##.#..#.";
        let inp_val = number_from_chars(&inp.chars().collect::<Vec<_>>());
        let rev_val = number_from_chars(&inp.chars().rev().collect::<Vec<_>>());

        assert_eq!(reverse_bits(inp_val), rev_val);
        assert_eq!(reverse_bits(rev_val), inp_val);
    }

    #[test]
    fn aoc2020_20_tile_rotate90() {
        let raw = raw_tile();
        let original = TileEdge::from(&raw);
        let rot90 = original.rotated90();
        let rot180 = rot90.rotated90();
        let rot270 = rot180.rotated90();
        let rot360 = rot270.rotated90();
        assert_eq!(original, rot360)
    }

    #[test]
    fn aoc2020_20_tile_flip() {
        let raw = raw_tile();
        let original = TileEdge::from(&raw);
        let flipped_ud = original.flipped_up_down();
        assert_eq!(original, flipped_ud.flipped_up_down());

        let flipped_lr = original.flipped_left_right();
        assert_eq!(original, flipped_lr.flipped_left_right());
    }

    fn raw_tile() -> TileRaw {
        TileRaw::from(
            "Tile 2311:
        ..##.#..#.
        ##..#.....
        #...##..#.
        ####.#...#
        ##.##.###.
        ##...#.###
        .#.#.#..##
        ..#....#..
        ###...#.#.
        ..###..###",
        )
    }

    #[test]
    fn aoc2020_20_case1() {
        let inp = "Tile 2311:
        ..##.#..#.
        ##..#.....
        #...##..#.
        ####.#...#
        ##.##.###.
        ##...#.###
        .#.#.#..##
        ..#....#..
        ###...#.#.
        ..###..###

        Tile 1951:
        #.##...##.
        #.####...#
        .....#..##
        #...######
        .##.#....#
        .###.#####
        ###.##.##.
        .###....#.
        ..#.#..#.#
        #...##.#..

        Tile 1171:
        ####...##.
        #..##.#..#
        ##.#..#.#.
        .###.####.
        ..###.####
        .##....##.
        .#...####.
        #.##.####.
        ####..#...
        .....##...

        Tile 1427:
        ###.##.#..
        .#..#.##..
        .#.##.#..#
        #.#.#.##.#
        ....#...##
        ...##..##.
        ...#.#####
        .#.####.#.
        ..#..###.#
        ..##.#..#.

        Tile 1489:
        ##.#.#....
        ..##...#..
        .##..##...
        ..#...#...
        #####...#.
        #..#.#.#.#
        ...#.#.#..
        ##.#...##.
        ..##.##.##
        ###.##.#..

        Tile 2473:
        #....####.
        #..#.##...
        #.##..#...
        ######.#.#
        .#...#.#.#
        .#########
        .###.#..#.
        ########.#
        ##...##.#.
        ..###.#.#.

        Tile 2971:
        ..#.#....#
        #...###...
        #.#.###...
        ##.##..#..
        .#####..##
        .#..####.#
        #..#.#..#.
        ..####.###
        ..#.#.###.
        ...#.#.#.#

        Tile 2729:
        ...#.#.#.#
        ####.#....
        ..#.#.....
        ....#..#.#
        .##..##.#.
        .#.####...
        ####.#.#..
        ##.####...
        ##..#.##..
        #.##...##.

        Tile 3079:
        #.#.#####.
        .#..######
        ..#.......
        ######....
        ####.#..#.
        .#...#.##.
        #.#####.##
        ..#.###...
        ..#.......
        ..#.###...
";
        let sol = AoC2020_20::parse(inp);

        assert_eq!(sol.part_one(), "20899048083289")
    }
}
