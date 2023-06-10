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
