use std::{
    ops::{AddAssign, SubAssign},
    str::FromStr,
};

use super::{checked_ops::CheckedOps, Direction};

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
    T: From<u8>,
{
    pub fn zero() -> Self {
        Self::new(T::from(0), T::from(0))
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

impl<T> Point2d<T>
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + CheckedOps + From<u8>,
{
    pub fn safe_moved_by(&self, direction: &Direction) -> Option<Self> {
        match *direction {
            Direction::Up => self.safe_up(),
            Direction::Down => self.safe_down(),
            Direction::Left => self.safe_left(),
            Direction::Right => self.safe_right(),
        }
    }

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn coordinate_safe_up_test() {
        let mut p = Point2d::<u8>::zero();
        assert_eq!(p.safe_up(), None);
        p.y = 1;
        assert_eq!(p.safe_up(), Some(Point2d::new(0, 0)));
    }

    #[test]
    fn coordinate_safe_down_test() {
        let mut p = Point2d::new(0, u8::MAX);
        assert_eq!(p.safe_down(), None);
        p.y = 0;
        assert_eq!(p.safe_down(), Some(Point2d::new(0, 1)));
    }

    #[test]
    fn coordinate_safe_left_test() {
        let mut p = Point2d::<u8>::zero();
        assert_eq!(p.safe_left(), None);
        p.x = 1;
        assert_eq!(p.safe_left(), Some(Point2d::new(0, 0)));
    }

    #[test]
    fn coordinate_safe_right_test() {
        let mut p = Point2d::new(u8::MAX, 0);
        assert_eq!(p.safe_right(), None);
        p.x = 0;
        assert_eq!(p.safe_right(), Some(Point2d::new(1, 0)));
    }
}
