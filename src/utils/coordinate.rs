use std::{
    ops::{AddAssign, SubAssign},
    str::FromStr,
};

use super::Direction;

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

impl<T> Point2d<T>
where
    T: From<u8>,
{
    pub fn zero() -> Self {
        Self::new(T::from(0), T::from(0))
    }
}

pub trait CheckedOps {
    fn checked_add(self, rhs: Self) -> Option<Self>
    where
        Self: Sized;
    fn checked_sub(self, rhs: Self) -> Option<Self>
    where
        Self: Sized;
}

impl<T> Point2d<T>
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + From<u8>,
{
    pub fn moved_by(&self, direction: &Direction) -> Self {
        match *direction {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
        }
    }

    pub fn up(&self) -> Self {
        Self::new(self.x, self.y - T::from(1))
    }

    pub fn down(&self) -> Self {
        Self::new(self.x, self.y + T::from(1))
    }

    pub fn left(&self) -> Self {
        Self::new(self.x - T::from(1), self.y)
    }

    pub fn right(&self) -> Self {
        Self::new(self.x + T::from(1), self.y)
    }
}

macro_rules! impl_checked_ops {
    ($($t:ty),*) => {
        $(
            impl CheckedOps for $t {
                fn checked_add(self, rhs: Self) -> Option<Self> {
                    Self::checked_add(self, rhs)
                }
                fn checked_sub(self, rhs: Self) -> Option<Self> {
                    Self::checked_sub(self, rhs)
                }
            }
        )*
    };
}

impl_checked_ops!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

impl<T> Point2d<T>
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + CheckedOps + From<u8>,
{
    pub fn safe_up(&self) -> Option<Self> {
        Some(Self {
            x: self.x,
            y: self.y.checked_sub(T::from(1))?,
        })
    }

    pub fn safe_down(&self) -> Option<Self> {
        Some(Self {
            x: self.x,
            y: self.y.checked_add(T::from(1))?,
        })
    }

    pub fn safe_right(&self) -> Option<Self> {
        Some(Self {
            x: self.x.checked_add(T::from(1))?,
            y: self.y,
        })
    }

    pub fn safe_left(&self) -> Option<Self> {
        Some(Self {
            x: self.x.checked_sub(T::from(1))?,
            y: self.y,
        })
    }
}
