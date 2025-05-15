use std::{
    cmp::Eq,
    collections::{HashSet, VecDeque},
    hash::Hash,
};

pub struct BreadthFirstSearch<T, F, I, Acc>
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> I,
{
    adjacency: F,
    early_exit_fn: Option<Box<dyn Fn(&T) -> bool>>,
    acc: Option<Acc>,
    accumulate_fn: Option<Box<dyn Fn(&mut Acc, T, T)>>,
}

impl<T, F, I, Acc> BreadthFirstSearch<T, F, I, Acc>
where
    T: Hash + Eq + Clone,
    I: Iterator<Item = T>,
    F: Fn(&T) -> I,
{
    pub fn with_adjacency(func: F) -> Self {
        Self {
            adjacency: func,
            early_exit_fn: None,
            acc: None,
            accumulate_fn: None,
        }
    }

    pub fn early_exit<E>(mut self, func: E) -> Self
    where
        E: Fn(&T) -> bool + 'static,
    {
        self.early_exit_fn = Some(Box::new(move |val: &T| -> bool { func(val) }));
        self
    }

    pub fn accumulate<U>(mut self, initial: Acc, func: U) -> Self
    where
        U: Fn(&mut Acc, T, T) + 'static,
    {
        self.acc = Some(initial);
        self.accumulate_fn = Some(Box::new(move |acc: &mut Acc, prev: T, current: T| {
            func(acc, prev, current)
        }));
        self
    }

    pub fn search(mut self, initial: T) -> SearchResult<Acc> {
        let mut dequeue = VecDeque::<T>::new();
        dequeue.push_back(initial.clone());
        let mut seen = HashSet::new();
        seen.insert(initial);
        let mut steps = 0;
        let mut is_early_exited = false;
        'outer: while let Some(node) = dequeue.pop_front() {
            steps += 1;
            for adjacent in (self.adjacency)(&node) {
                if seen.contains(&adjacent) {
                    continue;
                }
                if self
                    .early_exit_fn
                    .as_ref()
                    .map(|func| func(&adjacent))
                    .unwrap_or(false)
                {
                    is_early_exited = true;
                    break 'outer;
                }
                if let (Some(acc), Some(accumulate_fn)) =
                    (self.acc.as_mut(), self.accumulate_fn.as_ref())
                {
                    accumulate_fn(acc, node.clone(), adjacent.clone())
                }
                seen.insert(adjacent.clone());
                dequeue.push_back(adjacent);
            }
        }
        SearchResult {
            steps,
            accumulator: self.acc,
            is_early_exit: is_early_exited,
        }
    }
}

pub struct SearchResult<A> {
    pub steps: usize,
    pub accumulator: Option<A>,
    pub is_early_exit: bool,
}
