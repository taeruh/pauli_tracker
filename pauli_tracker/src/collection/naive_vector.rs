use std::{iter::Enumerate, slice};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{Base, Full, Init, Iterable, IterableBase};
use crate::slice_extension::GetTwoMutSlice;

/// A newtype wrapper around [Vec], implementing the [collection](super) traits,
/// **unchecked**.
///
/// Similar to [BufferedVector](super::BufferedVector), with the major difference that
/// there are no buffers and no runtime checks. This means, a `remove` is always a
/// **pop** and an `insert` is always a **push**. It's only useful if bits are
/// inserted and removed at the end (otherwise it errors).
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NaiveVector<T>(pub Vec<T>);

// TODO: make macro that implements this stuff for NaiveVector and BufferedVector (it's
// exactly the same, except for the remove and insert methods)

impl<T> NaiveVector<T> {
    /// Creates a new empty [NaiveVector].
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates a new empty [NaiveVector] with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
}

impl<T> From<Vec<T>> for NaiveVector<T> {
    fn from(vec: Vec<T>) -> Self {
        Self(vec)
    }
}

impl<T> FromIterator<(usize, T)> for NaiveVector<T> {
    fn from_iter<I: IntoIterator<Item = (usize, T)>>(iter: I) -> Self {
        let mut res = Vec::new();
        for (key, value) in iter {
            res.insert(key, value);
        }
        Self(res)
    }
}

impl<'l, T> IntoIterator for &'l NaiveVector<T> {
    type Item = (usize, &'l T);
    type IntoIter = Enumerate<slice::Iter<'l, T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().enumerate()
    }
}

impl<'l, T> IntoIterator for &'l mut NaiveVector<T> {
    type Item = (usize, &'l mut T);
    type IntoIter = Enumerate<slice::IterMut<'l, T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut().enumerate()
    }
}

impl<T> IntoIterator for NaiveVector<T> {
    type Item = (usize, T);
    type IntoIter = Enumerate<<Vec<T> as IntoIterator>::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().enumerate()
    }
}

/// A `remove` is always a **pop** and an `insert` is always a **push**.
impl<T> Base for NaiveVector<T>
where
    T: Clone + Default,
{
    type TB = T;
    fn insert(&mut self, _: usize, value: T) -> Option<T> {
        self.0.push(value);
        None
    }

    fn remove(&mut self, _: usize) -> Option<T> {
        self.0.pop()
    }

    fn get(&self, key: usize) -> Option<&T> {
        self.0.get(key)
    }

    fn get_mut(&mut self, key: usize) -> Option<&mut T> {
        self.0.get_mut(key)
    }

    fn get_two_mut(&mut self, key_a: usize, key_b: usize) -> Option<(&mut T, &mut T)> {
        self.0.get_two_mut(key_a, key_b)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T> Iterable for NaiveVector<T>
where
    T: Default + Clone,
{
    type TI = T;
    type Iter<'l>
        = <&'l Self as IntoIterator>::IntoIter
    where
        T: 'l;
    type IterMut<'l>
        = <&'l mut Self as IntoIterator>::IntoIter
    where
        T: 'l;

    fn iter_pairs(&self) -> Self::Iter<'_> {
        self.into_iter()
    }

    fn iter_pairs_mut(&mut self) -> Self::IterMut<'_> {
        self.into_iter()
    }
}

impl<T> Init for NaiveVector<T>
where
    T: Clone + Default,
{
    fn init(len: usize) -> Self {
        Self(vec![Default::default(); len])
    }
}

impl<T> IterableBase for NaiveVector<T>
where
    T: Default + Clone,
{
    type T = T;
}
impl<T> Full for NaiveVector<T> where T: Default + Clone {}
