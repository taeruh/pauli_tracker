use std::ops::Range;

use itertools::{
    Either,
    Itertools,
    Powerset,
};

/// An iterator over all partitions of a set.
#[derive(Debug, Clone)]
pub struct Partition<T> {
    pub(crate) set: T,
    iter: Powerset<Range<usize>>,
}

impl<T: Default> Default for Partition<T> {
    fn default() -> Self {
        Self {
            set: T::default(),
            iter: (0..0).powerset(),
        }
    }
}

impl<T> Partition<T> {
    /// Create a new partition iterator.
    pub fn new(set: T, len: usize) -> Self {
        Self { set, iter: (0..len).powerset() }
    }
}

impl<T> Iterator for Partition<T>
where
    T: IntoIterator + Clone,
{
    type Item = (Vec<T::Item>, Vec<T::Item>);
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(subset) => {
                let mut subset = subset.into_iter().peekable();
                Some(self.set.clone().into_iter().enumerate().partition_map(|(i, e)| {
                    match subset.peek() {
                        Some(&p) if p == i => {
                            subset.next();
                            Either::Left(e)
                        },
                        _ => Either::Right(e),
                    }
                }))
            },
            None => None,
        }
    }
}
