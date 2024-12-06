#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position2<T> {
    pub row: T,
    pub col: T,
}

impl<T> Position2<T> {
    pub fn new(row: T, col: T) -> Self {
        Self { row, col }
    }
}
