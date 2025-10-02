use std::str::FromStr;

use super::Point2d;

pub type Vec2<T> = Vec<Vec<T>>;

pub trait Transformable2d {
    fn flipped_horizontally(&self) -> Self;
    fn flipped_vertically(&self) -> Self;
    fn transposed(&self) -> Self;
}

impl<T: Clone> Transformable2d for Vec2<T> {
    fn flipped_horizontally(&self) -> Self {
        self.iter()
            .map(|row| row.iter().rev().cloned().collect())
            .collect::<Self>()
    }

    fn flipped_vertically(&self) -> Self {
        self.iter().rev().cloned().collect::<Self>()
    }

    fn transposed(&self) -> Self {
        let Some(cols) = self.first().map(|arr| arr.len()) else {
            return self.clone();
        };

        let rows = self.len();

        (0..cols)
            .map(|col| (0..rows).map(|row| self[row][col].clone()).collect())
            .collect::<Self>()
    }
}

pub trait Diminishable: Sized {
    fn diminished(&self, size: usize) -> Option<Self>;
}

impl<T: Clone> Diminishable for Vec2<T> {
    fn diminished(&self, size: usize) -> Option<Self> {
        let len = self.len();
        if 2 * size >= len {
            return None;
        }
        let res = self
            .iter()
            .skip(size)
            .take(len - 2 * size)
            .map(|row| {
                row.iter()
                    .skip(size)
                    .take(len - 2 * size)
                    .cloned()
                    .collect()
            })
            .collect();
        Some(res)
    }
}

pub fn get_first_position<T: Eq>(map: &[Vec<T>], element: T) -> Option<Point2d<usize>> {
    for (row, arr) in map.iter().enumerate() {
        for (col, val) in arr.iter().enumerate() {
            if *val == element {
                return Some(Point2d { y: row, x: col });
            }
        }
    }
    None
}

pub trait ArraySpin {
    fn spin_left(&mut self, count: usize);
    fn spin_right(&mut self, count: usize);
}

impl<T> ArraySpin for Vec<T> {
    fn spin_left(&mut self, count: usize) {
        let len = self.len();
        let step = count % len;
        let seg = &mut self[..step];
        seg.reverse();
        let seg = &mut self[step..];
        seg.reverse();
        self.reverse();
    }

    fn spin_right(&mut self, count: usize) {
        let len = self.len();
        let step = count % len;
        let seg = &mut self[..len - step];
        seg.reverse();
        let seg = &mut self[len - step..];
        seg.reverse();
        self.reverse();
    }
}

pub fn parse<I: AsRef<str>, O: FromStr>(input: &[I]) -> Result<Vec<O>, O::Err> {
    input.iter().map(|x| x.as_ref().parse()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn transformable_transpose() {
        let src = vec![vec![1, 2]];
        let expected = vec![vec![1], vec![2]];
        assert_eq!(expected, src.transposed());
        assert_eq!(src, expected.transposed());
    }

    #[test]
    fn transformable_flip_hor() {
        let src = vec![vec![1, 2], vec![3, 4]];
        let expected = vec![vec![2, 1], vec![4, 3]];
        assert_eq!(expected, src.flipped_horizontally());
        assert_eq!(expected.flipped_horizontally(), src);
    }

    #[test]
    fn transformable_flip_vert() {
        let src = vec![vec![1, 2], vec![3, 4]];
        let expected = vec![vec![3, 4], vec![1, 2]];
        assert_eq!(expected, src.flipped_vertically());
        assert_eq!(expected.flipped_vertically(), src);
    }

    #[test]
    fn diminishable_0() {
        let inp = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 2, 2, 2, 1],
            vec![1, 2, 2, 2, 1],
            vec![1, 2, 2, 2, 1],
            vec![1, 1, 1, 1, 1],
        ];
        #[rustfmt::skip]
        let exp = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 2, 2, 2, 1],
            vec![1, 2, 2, 2, 1],
            vec![1, 2, 2, 2, 1],
            vec![1, 1, 1, 1, 1],
        ];
        assert_eq!(inp.diminished(0), Some(exp));
    }

    #[test]
    fn diminishable_1() {
        let inp = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 2, 2, 2, 1],
            vec![1, 2, 2, 2, 1],
            vec![1, 2, 2, 2, 1],
            vec![1, 1, 1, 1, 1],
        ];
        #[rustfmt::skip]
        let exp =  vec![
            vec![ 2, 2, 2],
            vec![ 2, 2, 2],
            vec![ 2, 2, 2],
        ];
        assert_eq!(inp.diminished(1), Some(exp));
    }

    #[test]
    fn diminishable_2() {
        let inp = vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 1, 1, 0],
            vec![0, 1, 2, 2, 2, 1, 0],
            vec![0, 1, 2, 2, 2, 1, 0],
            vec![0, 1, 2, 2, 2, 1, 0],
            vec![0, 1, 1, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
        ];
        #[rustfmt::skip]
        let exp =  vec![
            vec![ 2, 2, 2],
            vec![ 2, 2, 2],
            vec![ 2, 2, 2],
        ];
        assert_eq!(inp.diminished(2), Some(exp));
    }

    #[test]
    fn diminishable_3() {
        let inp = vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 1, 1, 0],
            vec![0, 1, 2, 2, 2, 1, 0],
            vec![0, 1, 2, 3, 2, 1, 0],
            vec![0, 1, 2, 2, 2, 1, 0],
            vec![0, 1, 1, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
        ];
        let exp = vec![vec![3]];
        assert_eq!(inp.diminished(3), Some(exp));
    }

    #[test]
    fn diminishable_oversize() {
        let inp = vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 1, 1, 0],
            vec![0, 1, 2, 2, 2, 1, 0],
            vec![0, 1, 2, 3, 2, 1, 0],
            vec![0, 1, 2, 2, 2, 1, 0],
            vec![0, 1, 1, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
        ];
        assert_eq!(inp.diminished(4), None);
        assert_eq!(inp.diminished(5), None);
    }
}
