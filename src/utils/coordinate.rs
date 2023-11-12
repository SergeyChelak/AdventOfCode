use std::{
    ops::{AddAssign, SubAssign},
    str::FromStr,
};

#[derive(Debug)]
pub enum PointParseError {
    InvalidFormat,
    TokenError,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point2d<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2d<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Point2d<T>
where
    T: FromStr,
{
    pub fn parse_csv(s: &str) -> Result<Self, PointParseError> {
        let tokens = s.split_once(',');
        if let Some((x, y)) = tokens {
            let parse = |inp: &str| {
                inp.trim()
                    .parse::<T>()
                    .map_err(|_| PointParseError::TokenError)
            };
            let x = parse(x)?;
            let y = parse(y)?;
            Ok(Self { x, y })
        } else {
            Err(PointParseError::InvalidFormat)
        }
    }
}

impl<T> Point2d<T>
where
    T: Copy + AddAssign + SubAssign,
{
    pub fn add(&self, other: &Self) -> Self {
        let mut res = *self;
        res.x += other.x;
        res.y += other.y;
        res
    }

    pub fn sub(&self, other: &Self) -> Self {
        let mut res = *self;
        res.x -= other.x;
        res.y -= other.y;
        res
    }
}

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
    let Some(min_x) = points.iter().map(|p| p.x).min() else {
        return None;
    };
    let Some(min_y) = points.iter().map(|p| p.y).min() else {
        return None;
    };
    let Some(max_x) = points.iter().map(|p| p.x).max() else {
        return None;
    };
    let Some(max_y) = points.iter().map(|p| p.y).max() else {
        return None;
    };
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
