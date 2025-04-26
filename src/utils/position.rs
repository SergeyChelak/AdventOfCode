use super::Point2d;

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

impl<T> From<Point2d<T>> for Position2<T> {
    fn from(value: Point2d<T>) -> Self {
        Self {
            row: value.y,
            col: value.x,
        }
    }
}

impl<T> From<Position2<T>> for Point2d<T> {
    fn from(value: Position2<T>) -> Self {
        Point2d {
            x: value.col,
            y: value.row,
        }
    }
}
