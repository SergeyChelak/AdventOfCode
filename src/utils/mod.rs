pub mod files;
pub use files::*;

pub mod options;
pub use options::*;

pub mod string2id;
pub use string2id::*;

pub mod permutations;
pub use permutations::*;

pub mod combinations;
pub use combinations::*;

pub mod arrays;
pub use arrays::*;

pub mod strings;
pub use strings::*;

pub mod coordinate;
pub use coordinate::*;

pub mod math;
pub use math::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
