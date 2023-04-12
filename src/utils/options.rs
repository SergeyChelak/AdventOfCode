pub fn less_option<T: Ord + Copy>(first: &Option<T>, second: &Option<T>) -> Option<T> {
    match (first, second) {
        (Some(v1), Some(v2)) => Some(*v1.min(v2)),
        (Some(val), None) | (None, Some(val)) => Some(*val),
        _ => None,
    }
}

pub fn bigger_option<T: Ord + Copy>(first: &Option<T>, second: &Option<T>) -> Option<T> {
    match (first, second) {
        (Some(v1), Some(v2)) => Some(*v1.max(v2)),
        (Some(val), None) | (None, Some(val)) => Some(*val),
        _ => None,
    }
}
