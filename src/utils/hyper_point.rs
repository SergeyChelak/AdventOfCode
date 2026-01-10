use std::{
    ops::{Add, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct HyperPoint<T>(pub Vec<T>);

impl<T> HyperPoint<T> {
    pub fn expand(&mut self, values: Vec<T>) {
        for v in values {
            self.0.push(v);
        }
    }

    pub fn dimension(&self) -> usize {
        self.0.len()
    }
}

impl<T> From<Vec<T>> for HyperPoint<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<T> HyperPoint<T>
where
    T: Copy,
{
    pub fn binary_operation(&self, operation: impl Fn(T, T) -> T, other: &Self) -> Self {
        assert_eq!(self.dimension(), other.dimension());
        Self(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(a, b)| operation(*a, *b))
                .collect(),
        )
    }
}

impl<T> HyperPoint<T>
where
    T: Copy + Add<Output = T>,
{
    pub fn add(&self, other: &Self) -> Self {
        self.binary_operation(|a, b| a + b, other)
    }
}

impl<T> HyperPoint<T>
where
    T: Copy + Sub<Output = T>,
{
    pub fn sub(&self, other: &Self) -> Self {
        self.binary_operation(|a, b| a - b, other)
    }
}

impl<T> HyperPoint<T>
where
    T: FromStr,
{
    pub fn from_csv(value: &str) -> Result<Self, T::Err> {
        let data = value
            .split(',')
            .map(|x| x.parse::<T>())
            .collect::<Result<Vec<T>, _>>()?;
        Ok(Self(data))
    }
}
