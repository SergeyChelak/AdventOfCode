use crate::utils::Vec2;

enum Store<'l, T> {
    Reference(&'l [Vec<T>]),
    Ownership(Vec2<T>),
}

impl<'l, T> Store<'l, T> {
    fn len(&self) -> usize {
        self.options().len()
    }

    fn len_at(&self, index: usize) -> usize {
        self.options()[index].len()
    }

    fn options(&self) -> &[Vec<T>] {
        match self {
            Store::Ownership(options) => options,
            Store::Reference(options) => options,
        }
    }
}

pub struct Cartesian<'l, T> {
    store: Store<'l, T>,
    indices: Vec<usize>,
    is_first: bool,
}

impl<'l, T> Cartesian<'l, T> {
    fn with(store: Store<'l, T>) -> Self {
        let count = store.len();
        let indices = vec![0; count];
        Self {
            store,
            indices,
            is_first: true,
        }
    }

    fn options(&self) -> &[Vec<T>] {
        self.store.options()
    }
}

impl<'l, T> Iterator for Cartesian<'l, T>
where
    T: Copy,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
            return Some(self.yeild());
        }

        let mut index = self.indices.len() - 1;
        loop {
            if self.increment(index) {
                return Some(self.yeild());
            }
            if index == 0 {
                break;
            }
            index -= 1;
        }
        None
    }
}

impl<'l, T> Cartesian<'l, T>
where
    T: Copy,
{
    fn reset_indices(&mut self, from: usize) {
        self.indices.iter_mut().skip(from).for_each(|x| *x = 0);
    }

    fn yeild(&self) -> Vec<T> {
        self.indices
            .iter()
            .zip(self.options().iter())
            .map(|(idx, arr)| arr[*idx])
            .collect::<Vec<_>>()
    }

    fn increment(&mut self, index: usize) -> bool {
        let last = self.store.len_at(index);
        if self.indices[index] == last - 1 {
            return false;
        }
        self.indices[index] += 1;
        self.reset_indices(index + 1);
        true
    }
}

pub trait CartesianIter<T: Copy> {
    fn cartesian_iter(&self) -> Cartesian<'_, T>;
}

impl<T> CartesianIter<T> for &[Vec<T>]
where
    T: Copy,
{
    fn cartesian_iter(&self) -> Cartesian<'_, T> {
        Cartesian::with(Store::Reference(self))
    }
}

pub trait RepeativeCartesianIter<T: Copy> {
    fn cartesian_iter(&self, times: usize) -> Cartesian<'_, T>;
}

impl<T> RepeativeCartesianIter<T> for [T]
where
    T: Copy,
{
    fn cartesian_iter(&self, times: usize) -> Cartesian<'_, T> {
        let mut v = Vec::new();
        for _ in 0..times {
            v.push(self.to_vec());
        }
        Cartesian::with(Store::Ownership(v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cartesian_() {
        //
    }

    #[test]
    fn cartesian_owning() {
        let range = [-1, 0, 1];

        let mut collected = Vec::new();
        for a in range.iter() {
            for b in range.iter() {
                for c in range.iter() {
                    collected.push(vec![*a, *b, *c]);
                }
            }
        }

        let cartesian = range.cartesian_iter(3).collect::<Vec<_>>();

        assert_eq!(cartesian, collected);
    }
}
