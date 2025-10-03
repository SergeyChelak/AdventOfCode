use crate::{
    solution::Solution,
    utils::{not_found, Diminishable, Point2d, Transformable2d, Vec2},
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
    fn edges(&self) -> HashMap<TileEdge, Vec<Transformation>> {
        let mut collection = HashMap::new();

        let modify = |inp: &TileEdge,
                      op: Transformation,
                      store: &mut HashMap<TileEdge, Vec<Transformation>>|
         -> TileEdge {
            let val = match op {
                Transformation::Rotate => inp.rotated90(),
                Transformation::FlipVertical => inp.flipped_up_down(),
            };

            let mut arr = store.get(inp).cloned().unwrap_or(vec![]);
            arr.push(op);
            store.insert(val.clone(), arr);
            val
        };

        let mut edges = TileEdge::from(self);
        for _ in 0..4 {
            edges = modify(&edges, Transformation::Rotate, &mut collection); //current.rotated90();
            _ = modify(&edges, Transformation::FlipVertical, &mut collection); // next.flipped_up_down();
        }

        {
            for (k, v) in collection.iter() {
                let x = TileRaw {
                    id: 0,
                    data: transform_series(&self.data, v),
                };
                let e = TileEdge::from(&x);

                assert_eq!(e, *k);
            }
        }

        collection
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Transformation {
    Rotate,
    FlipVertical,
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

    // fn flipped_left_right(&self) -> Self {
    //     Self {
    //         top: reverse_bits(self.top),
    //         right: self.left,
    //         bottom: reverse_bits(self.bottom),
    //         left: self.right,
    //     }
    // }
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

struct TileLayout {
    store: Vec2<Option<Disposition>>,
    dim: usize,
}

impl TileLayout {
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

    fn get(&self, pos: usize) -> Option<&Disposition> {
        if self.at_end(pos) {
            return None;
        }
        let (row, col) = self.encode_pos(pos);
        self.store[row][col].as_ref()
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

#[derive(Default)]
struct TileDataStore {
    edges: HashMap<usize, Vec<TileEdge>>,
    transformations: HashMap<(usize, TileEdge), Vec<Transformation>>,
    data: HashMap<usize, Vec2<char>>,
}

impl TileDataStore {
    fn new() -> Self {
        Default::default()
    }

    fn with(raw_tiles: &[TileRaw]) -> Self {
        let mut store = Self::new();
        store.fill_store(raw_tiles);
        store
    }

    fn fill_store(&mut self, raw_tiles: &[TileRaw]) {
        raw_tiles.iter().for_each(|raw| {
            let id = raw.id;
            self.data.insert(id, raw.data.clone());

            let edges = raw.edges();
            self.edges.insert(id, edges.keys().cloned().collect());

            for (k, v) in edges.into_iter() {
                self.transformations.insert((id, k), v);
            }
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

    fn transformations(&self, disp: &Disposition) -> &[Transformation] {
        self.transformations
            .get(&(disp.tile_id, disp.edges.clone()))
            .expect("transformations can't be None")
    }

    fn data(&self, id: usize) -> &Vec2<char> {
        self.data.get(&id).expect("data can't be missed")
    }
}

fn dfs(
    tile_store: &TileDataStore,
    matrix: &mut TileLayout,
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

pub struct AoC2020_20 {
    tiles: Vec<TileRaw>,
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

    fn make_layout(&self) -> (TileLayout, TileDataStore) {
        let dim = self.tiles.len().isqrt();
        assert_eq!(dim * dim, self.tiles.len());

        let tile_store = TileDataStore::with(&self.tiles);
        let mut layout = TileLayout::new(dim);
        let is_ok = dfs(&tile_store, &mut layout, 0, &mut HashSet::new());

        assert!(is_ok);

        (layout, tile_store)
    }
}

impl Solution for AoC2020_20 {
    fn part_one(&self) -> String {
        self.make_layout()
            .0
            .product()
            .map(|val| val.to_string())
            .unwrap_or(not_found())
    }

    fn part_two(&self) -> String {
        let (layout, tile_store) = self.make_layout();
        let dim = self.tiles.len().isqrt();
        let (mut image, pixels) = assemble_image(layout, tile_store, dim);

        let mut result = pixels;
        let pattern = build_pattern();
        let cols = 1 + pattern.iter().map(|p| p.x).max().unwrap_or(0);
        let rows = 1 + pattern.iter().map(|p| p.y).max().unwrap_or(0);
        for _ in 0..4 {
            for _ in 0..2 {
                image = transform(&image, Transformation::FlipVertical);
                let val = check_pattern(&image, &pattern, rows, cols);
                result = result.min(pixels - val);
            }
            image = transform(&image, Transformation::Rotate);
        }
        result.to_string()
    }

    fn description(&self) -> String {
        "Day 20: Jurassic Jigsaw".to_string()
    }
}

fn assemble_image(
    layout: TileLayout,
    tile_store: TileDataStore,
    dim: usize,
) -> (Vec2<char>, usize) {
    let reduced_dim = TILE_SIZE - 2;
    let image_dim = dim * reduced_dim;
    let mut image = vec![vec!['x'; image_dim]; image_dim];

    let mut pixels = 0;
    for r in 0..dim {
        for c in 0..dim {
            let pos = r * dim + c;
            let disp = layout.get(pos).expect("Can't be none");
            let trans = tile_store.transformations(disp);
            let data = transform_series(tile_store.data(disp.tile_id), trans)
                .diminished(1)
                .expect("Failed to diminish data");

            for (i, row) in data.iter().enumerate() {
                for (j, val) in row.iter().enumerate() {
                    assert_eq!(image[reduced_dim * r + i][reduced_dim * c + j], 'x');
                    image[reduced_dim * r + i][reduced_dim * c + j] = *val;
                    if *val == '#' {
                        pixels += 1;
                    }
                }
            }
        }
    }
    // debug validation
    {
        let valid_fill = image.iter().all(|row| row.iter().all(|ch| *ch != 'x'));
        assert!(valid_fill);
    }

    (image, pixels)
}

fn transform_series<T: Clone>(data: &Vec2<T>, series: &[Transformation]) -> Vec2<T> {
    series
        .iter()
        .fold(data.clone(), |acc, t| transform(&acc, *t))
}

fn transform<T: Clone>(data: &Vec2<T>, t: Transformation) -> Vec2<T> {
    match t {
        Transformation::Rotate => data.transposed().flipped_horizontally(),
        Transformation::FlipVertical => data.flipped_vertically(),
    }
}

fn check_pattern(
    image: &Vec2<char>,
    pattern: &[Point2d<usize>],
    rows: usize,
    cols: usize,
) -> usize {
    let mut sum = 0;

    for r in 0..image.len() - rows {
        for c in 0..image[r].len() - cols {
            let tmp = pattern
                .iter()
                .map(|p| Point2d::new(p.x + c, p.y + r))
                .collect::<Vec<_>>();
            let is_match = tmp.iter().all(|p| image[p.y][p.x] == '#');
            if is_match {
                sum += pattern.len();
            }
        }
    }
    sum
}

fn build_pattern() -> Vec<Point2d<usize>> {
    " x                  #
      x#    ##    ##    ###
      x #  #  #  #  #  #
    "
    .split('\n')
    .map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .enumerate()
    .flat_map(|(r, row)| {
        row.chars()
            .skip(1)
            .enumerate()
            .filter(|(_, ch)| *ch == '#')
            .map(|(c, _)| Point2d::new(c, r))
            .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>()
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
        assert_eq!(sol.part_two(), "1901");
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

        // let flipped_lr = original.flipped_left_right();
        // assert_eq!(original, flipped_lr.flipped_left_right());
    }

    #[test]
    fn aoc2020_20_case1() {
        let sol = test_solution();
        assert_eq!(sol.part_one(), "20899048083289")
    }

    #[test]
    fn aoc2020_20_case2() {
        let sol = test_solution();
        assert_eq!(sol.part_two(), "273")
    }

    #[test]
    fn aoc2020_20_transform_match() {
        let original = raw_tile();

        let rot = TileRaw {
            id: 0,
            data: transform(&original.data, Transformation::Rotate),
        };

        let orig_edges = TileEdge::from(&original);
        let rot_edges = TileEdge::from(&rot);

        assert_eq!(rot_edges, orig_edges.rotated90());
    }

    #[test]
    fn aoc2020_20_build_pattern() {
        let pattern = build_pattern();
        assert_eq!(15, pattern.len());
        assert!(pattern.contains(&Point2d::new(0usize, 18)));
        assert!(pattern.contains(&Point2d::new(1usize, 0)));
        assert!(pattern.contains(&Point2d::new(1usize, 18)));
        assert!(pattern.contains(&Point2d::new(1usize, 19)));
        assert!(pattern.contains(&Point2d::new(2usize, 1)));
        assert!(pattern.contains(&Point2d::new(2usize, 16)));
    }

    #[test]
    fn aoc2020_20_pattern_matching() {
        let mut inp = ".#.#..#.##...#.##..#####
        ###....#.#....#..#......
        ##.##.###.#.#..######...
        ###.#####...#.#####.#..#
        ##.#....#.##.####...#.##
        ...########.#....#####.#
        ....#..#...##..#.#.###..
        .####...#..#.....#......
        #..#.##..#..###.#.##....
        #.####..#.####.#.#.###..
        ###.#.#...#.######.#..##
        #.####....##..########.#
        ##..##.#...#...#.#.#.#..
        ...#..#..#.#.##..###.###
        .#.#....#.##.#...###.##.
        ###.#...#..#.##.######..
        .#.#.###.##.##.#..#.##..
        .####.###.#...###.#..#.#
        ..#.#..#..#.#.#.####.###
        #..####...#.#.#.###.###.
        #####..#####...###....##
        #.##..#..#...#..####...#
        .#.###..##..##..####.##.
        ...###...##...#...#..###"
            .trim()
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        inp = inp
            .transposed()
            .flipped_horizontally()
            .flipped_horizontally();

        for r in inp.iter() {
            for c in r.iter() {
                print!("{c}");
            }
            println!();
        }

        let pattern = build_pattern();
        let cols = pattern.iter().map(|p| p.x).max().unwrap_or(0);
        let rows = pattern.iter().map(|p| p.y).max().unwrap_or(0);

        let count = check_pattern(&inp, &pattern, rows, cols);
        assert_eq!(30, count)
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

    fn test_solution() -> AoC2020_20 {
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
        AoC2020_20::parse(inp)
    }
}
