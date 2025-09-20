//
// Implementation based on Wikipedia article
// https://en.wikipedia.org/wiki/Heap%27s_algorithm
//
pub struct PermutationIterator<T> {
    a: Vec<T>,
    c: Vec<usize>, // stack state
    i: usize,      // stack pointer
}

impl<T: Copy> PermutationIterator<T> {
    pub fn from_array(array: &[T]) -> Self {
        Self {
            a: array.to_owned(),
            c: vec![0; array.len()],
            i: 0,
        }
    }
}

impl<T: Copy> Iterator for PermutationIterator<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 0 {
            self.i = 1;
            return Some(self.a.clone());
        }
        while self.i < self.a.len() {
            if self.c[self.i] < self.i {
                if self.i.is_multiple_of(2) {
                    self.a.swap(0, self.i);
                } else {
                    self.a.swap(self.c[self.i], self.i);
                }
                self.c[self.i] += 1;
                self.i = 1;
                return Some(self.a.clone());
            } else {
                self.c[self.i] = 0;
                self.i += 1;
            }
        }
        None
    }
}

pub trait Permutable<T> {
    fn permut_iter(&self) -> PermutationIterator<T>;
}

impl<T: Copy> Permutable<T> for Vec<T> {
    fn permut_iter(&self) -> PermutationIterator<T> {
        PermutationIterator::from_array(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn utils_permutations_case1() {
        let mut iter = PermutationIterator::from_array(&[1]);
        let val = iter.next();
        assert!(val.is_some());
        let val = val.unwrap();
        assert_eq!(val.len(), 1);
        assert_eq!(val[0], 1);
        let val = iter.next();
        assert!(val.is_none())
    }

    #[test]
    fn utils_permutations_case2() {
        let iter = PermutationIterator::from_array(&[1, 2]);
        let set: HashSet<Vec<usize>> = vec![vec![1, 2], vec![2, 1]].into_iter().collect();
        for v in iter {
            assert!(set.contains(&v));
        }
    }

    #[test]
    fn utils_permutations_case3() {
        let iter = PermutationIterator::from_array(&[1, 2, 3]);
        let set: HashSet<Vec<usize>> = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ]
        .into_iter()
        .collect();
        for v in iter {
            assert!(set.contains(&v));
        }
    }
}
