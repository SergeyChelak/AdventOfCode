use crate::utils::Vec2;

pub struct Cartesian<T> {
    options: Vec2<T>,
    indices: Vec<usize>,
    is_first: bool,
}

impl<T> Cartesian<T> {
    fn new(options: Vec2<T>) -> Self {
        let count = options.len();
        let indices = vec![0; count];
        Self {
            options,
            indices,
            is_first: true,
        }
    }
}

impl<T> Iterator for Cartesian<T>
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

impl<T> Cartesian<T>
where
    T: Copy,
{
    fn reset_indices(&mut self, from: usize) {
        self.indices.iter_mut().skip(from).for_each(|x| *x = 0);
    }

    fn yeild(&self) -> Vec<T> {
        self.indices
            .iter()
            .zip(self.options.iter())
            .map(|(idx, arr)| arr[*idx])
            .collect::<Vec<_>>()
    }

    fn increment(&mut self, index: usize) -> bool {
        let last = self.options[index].len();
        if self.indices[index] == last - 1 {
            return false;
        }
        self.indices[index] += 1;
        self.reset_indices(index + 1);
        true
    }
}

pub trait CartesianIter<T: Copy> {
    fn cartesian_iter(&self, times: usize) -> Cartesian<T>;
}

impl<T> CartesianIter<T> for [T]
where
    T: Copy,
{
    fn cartesian_iter(&self, times: usize) -> Cartesian<T> {
        let mut v = Vec::new();
        for _ in 0..times {
            v.push(self.to_vec());
        }
        Cartesian::new(v)
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
    fn cartesian_t1() {
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
