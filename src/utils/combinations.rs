//
// Generate combinations with Gray codes
// Based on
// https://cp-algorithms.com/combinatorics/generating_combinations.html
//

pub struct CombinationIterator<'a, T> {
    array: &'a Vec<T>,
    k: usize,
    pos: usize,
}

impl<'a, T> CombinationIterator<'a, T> {
    pub fn from_vector(v: &'a Vec<T>, k: usize) -> Self {
        Self {
            array: v,
            k,
            pos: 0,
        }
    }
}

impl<'a, T: Copy> Iterator for CombinationIterator<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.array.len();
        if self.pos < 1 << n {
            let cur = gray_code(self.pos);
            if count_bits(cur) == self.k {
                let mut output = Vec::with_capacity(self.k);
                for j in 0..n {
                    if cur & (1 << j) != 0 {
                        output.push(self.array[j]);
                    }
                }
                self.pos += 1;
                return Some(output);
            }
        }
        None
    }
}

#[inline(always)]
fn gray_code(n: usize) -> usize {
    n ^ (n >> 1)
}

fn count_bits(n: usize) -> usize {
    let mut res = 0;
    let mut i = n;
    while i > 0 {
        res += i & 1;
        i >>= 1;
    }
    res
}

pub trait Combinable<T> {
    fn combination_iter(&self, k: usize) -> CombinationIterator<T>;
}

impl<T: Copy> Combinable<T> for Vec<T> {
    fn combination_iter(&self, k: usize) -> CombinationIterator<T> {
        CombinationIterator::from_vector(&self, k)
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
        ].into_iter().collect();
        vec![0, 1, 2, 3]
            .combination_iter(2)
            .for_each(|arr| {
                assert!(cases.contains(&arr));
            });
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
        ].into_iter().collect();
        vec![0, 1, 2, 3, 4, 5]
            .combination_iter(3)
            .for_each(|arr| {
                assert!(cases.contains(&arr));
            });
    }
}