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

#[derive(Debug, Clone, Copy)]
pub struct PlainInterval<T> {
    pub begin: T,
    pub end: T,
}

impl<T> PlainInterval<T> {
    pub fn new(begin: T, end: T) -> Self {
        Self { begin, end }
    }
}

impl<T> PlainInterval<T>
where
    T: Copy + Ord,
{
    pub fn _with_disordered(begin: T, end: T) -> Self {
        Self {
            begin: begin.min(end),
            end: end.max(begin),
        }
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let (l, r) = if self.begin < other.begin {
            (self, other)
        } else {
            (other, self)
        };

        if r.begin > l.end {
            return None;
        }

        Some(PlainInterval {
            begin: self.begin.max(other.begin),
            end: self.end.min(other.end),
        })
    }
}

impl Point2d<i32> {
    pub fn left(&self) -> Self {
        Self::new(self.x, self.y - 1)
    }

    pub fn right(&self) -> Self {
        Self::new(self.x, self.y + 1)
    }

    pub fn up(&self) -> Self {
        Self::new(self.x - 1, self.y)
    }

    pub fn down(&self) -> Self {
        Self::new(self.x + 1, self.y)
    }
}
