//
// Combinations iterator
//
pub struct CombinationIterator<'a, T> {
    array: &'a Vec<T>,
    c: Vec<usize>,
    k: usize,
    is_first: bool,
}

impl<'a, T: Copy> CombinationIterator<'a, T> {
    #[allow(clippy::needless_range_loop)]
    pub fn from_vector(array: &'a Vec<T>, k: usize) -> Self {
        let n = array.len();
        let mut c = vec![0; k + 3];
        for i in 1..=k {
            c[i] = i - 1;
        }
        c[k + 1] = n;
        c[k + 2] = 0;
        Self {
            array,
            c,
            k,
            is_first: true,
        }
    }

    fn get_combination(&self) -> Vec<T> {
        self.c[1..=self.k]
            .iter()
            .map(|i| self.array[*i])
            .collect::<Vec<T>>()
    }
}

impl<T: Copy> Iterator for CombinationIterator<'_, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
        } else {
            let mut j = 1;
            while self.c[j] + 1 == self.c[j + 1] {
                self.c[j] = j - 1;
                j += 1;
            }
            if j > self.k {
                return None;
            }
            self.c[j] += 1;
        }
        Some(self.get_combination())
    }
}

pub trait Combinable<T> {
    fn combination_iter(&self, k: usize) -> CombinationIterator<'_, T>;
}

impl<T: Copy> Combinable<T> for Vec<T> {
    fn combination_iter(&self, k: usize) -> CombinationIterator<'_, T> {
        CombinationIterator::from_vector(self, k)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn utils_combination_case4_2() {
        let cases: HashSet<Vec<usize>> = vec![
            vec![0, 1],
            vec![1, 2],
            vec![0, 2],
            vec![2, 3],
            vec![1, 3],
            vec![0, 3],
        ]
        .into_iter()
        .collect();

        let combs = vec![0, 1, 2, 3]
            .combination_iter(2)
            .collect::<Vec<Vec<usize>>>();
        assert_eq!(combs.len(), cases.len());

        combs.iter().for_each(|arr| assert!(cases.contains(arr)));
    }

    #[test]
    fn utils_combination_case6_3() {
        let cases: HashSet<Vec<usize>> = vec![
            vec![0, 1, 2],
            vec![0, 2, 3],
            vec![1, 2, 3],
            vec![0, 1, 3],
            vec![0, 3, 4],
            vec![1, 3, 4],
            vec![2, 3, 4],
            vec![0, 2, 4],
            vec![1, 2, 4],
            vec![0, 1, 4],
            vec![0, 4, 5],
            vec![1, 4, 5],
            vec![2, 4, 5],
            vec![3, 4, 5],
            vec![0, 3, 5],
            vec![1, 3, 5],
            vec![2, 3, 5],
            vec![0, 2, 5],
            vec![1, 2, 5],
            vec![0, 1, 5],
        ]
        .into_iter()
        .collect();
        let combs = vec![0, 1, 2, 3, 4, 5]
            .combination_iter(3)
            .collect::<Vec<Vec<usize>>>();
        assert_eq!(combs.len(), cases.len());

        combs.iter().for_each(|arr| assert!(cases.contains(arr)));
    }
}
