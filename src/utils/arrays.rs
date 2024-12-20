use super::Position2;

pub type Vec2<T> = Vec<Vec<T>>;

pub fn get_first_position<T: Eq>(map: &[Vec<T>], element: T) -> Option<Position2<usize>> {
    for (row, arr) in map.iter().enumerate() {
        for (col, val) in arr.iter().enumerate() {
            if *val == element {
                return Some(Position2 { row, col });
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
