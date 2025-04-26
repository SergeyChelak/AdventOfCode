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
