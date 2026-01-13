use std::str::FromStr;

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
    /// creates interval with element in arbitrary order
    /// function will arrange them in the expected way
    pub fn with_arbitrary(a: T, b: T) -> Self {
        Self::new(a.min(b), a.max(b))
    }

    pub fn has_intersection(self, other: &Self) -> bool {
        self.begin.max(other.begin) <= self.end.min(other.end)
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

    pub fn union(&self, other: &Self) -> Option<Self> {
        let (l, r) = if self.begin < other.begin {
            (self, other)
        } else {
            (other, self)
        };

        if r.begin > l.end {
            return None;
        }

        Some(PlainInterval {
            begin: self.begin.min(other.begin),
            end: self.end.max(other.end),
        })
    }

    pub fn custom_contain(&self, value: T, include_begin: bool, include_end: bool) -> bool {
        ((self.begin < value) || include_begin && (self.begin == value))
            && ((self.end > value) || include_end && (self.end == value))
    }

    pub fn close_contain(&self, value: T) -> bool {
        self.custom_contain(value, true, true)
    }
}

impl<T> PlainInterval<T>
where
    T: FromStr,
{
    pub fn parse(value: &str, delimiter: &str) -> Result<Self, IntervalParseError> {
        let (l, r) = value
            .split_once(delimiter)
            .ok_or(IntervalParseError::WrongFormat)?;
        let begin = l
            .parse::<T>()
            .map_err(|_| IntervalParseError::InvalidValue)?;
        let end = r
            .parse::<T>()
            .map_err(|_| IntervalParseError::InvalidValue)?;
        Ok(Self::new(begin, end))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IntervalParseError {
    WrongFormat,
    InvalidValue,
}
