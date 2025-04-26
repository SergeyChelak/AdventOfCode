use std::ops::{AddAssign, SubAssign};

use super::Point2d;

pub struct Bounds<T> {
    pub low: Point2d<T>,
    pub high: Point2d<T>,
}

impl<T: Ord + Copy + AddAssign + SubAssign> Bounds<T> {
    pub fn size(&self) -> Point2d<T> {
        self.high.sub(&self.low)
    }
}

/// Returns top left and bottom right points
pub fn bounds<T: Ord + Copy>(points: &[Point2d<T>]) -> Option<Bounds<T>> {
    let min_x = points.iter().map(|p| p.x).min()?;
    let min_y = points.iter().map(|p| p.y).min()?;
    let max_x = points.iter().map(|p| p.x).max()?;
    let max_y = points.iter().map(|p| p.y).max()?;
    Some(Bounds {
        low: Point2d { x: min_x, y: min_y },
        high: Point2d { x: max_x, y: max_y },
    })
}

pub fn normalize_with_bounds<T>(points: &[Point2d<T>], bounds: &Bounds<T>) -> Vec<Point2d<T>>
where
    T: Ord + Copy + AddAssign + SubAssign,
{
    let a = bounds.low;
    points
        .iter()
        .map(|p| p.sub(&a))
        .collect::<Vec<Point2d<T>>>()
}
