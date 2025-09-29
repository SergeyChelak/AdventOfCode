use std::marker::PhantomData;

pub trait CartesianStore<T> {
    fn len(&self) -> usize;
    fn len_at(&self, index: usize) -> usize;
    fn output(&self, indices: &[usize]) -> Vec<T>;
}

pub struct Cartesian<T, S: CartesianStore<T>> {
    store: S,
    indices: Vec<usize>,
    is_first: bool,
    phantom: PhantomData<T>,
}

impl<T, S> Cartesian<T, S>
where
    S: CartesianStore<T>,
{
    fn with(store: S) -> Self {
        let count = store.len();
        let indices = vec![0; count];
        Self {
            store,
            indices,
            is_first: true,
            phantom: PhantomData,
        }
    }
}

impl<T, S> Iterator for Cartesian<T, S>
where
    T: Clone,
    S: CartesianStore<T>,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
            return Some(self.yield_state());
        }

        let mut index = self.indices.len();
        while index > 0 {
            index -= 1;
            if self.increment(index) {
                return Some(self.yield_state());
            }
        }
        None
    }
}
impl<T, S> Cartesian<T, S>
where
    T: Clone,
    S: CartesianStore<T>,
{
    fn reset_indices(&mut self, from: usize) {
        self.indices.iter_mut().skip(from).for_each(|x| *x = 0);
    }

    fn yield_state(&self) -> Vec<T> {
        self.store.output(&self.indices)
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

pub struct OwnedVecStore<T> {
    options: crate::utils::Vec2<T>,
}

impl<T> OwnedVecStore<T> {
    fn new(options: crate::utils::Vec2<T>) -> Self {
        Self { options }
    }
}

impl<T> CartesianStore<T> for OwnedVecStore<T>
where
    T: Clone,
{
    fn len(&self) -> usize {
        self.options.len()
    }

    fn len_at(&self, index: usize) -> usize {
        self.options[index].len()
    }

    fn output(&self, indices: &[usize]) -> Vec<T> {
        indices
            .iter()
            .zip(self.options.iter())
            .map(|(idx, arr)| arr[*idx].clone())
            .collect::<Vec<_>>()
    }
}

pub trait RepeativeCartesianIter<T>
where
    T: Clone,
{
    fn cartesian_iter(&self, times: usize) -> Cartesian<T, OwnedVecStore<T>>;
}

impl<T> RepeativeCartesianIter<T> for [T]
where
    T: Clone,
{
    fn cartesian_iter(&self, times: usize) -> Cartesian<T, OwnedVecStore<T>> {
        let mut v = Vec::new();
        for _ in 0..times {
            v.push(self.to_vec());
        }
        let store = OwnedVecStore::new(v);
        Cartesian::with(store)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
