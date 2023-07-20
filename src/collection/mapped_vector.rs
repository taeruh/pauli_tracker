use std::{
    collections::HashMap,
    iter::{
        Map,
        Zip,
    },
    mem,
    ops::{
        Index,
        IndexMut,
    },
    slice,
};

use itertools::Itertools;
#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    Collection,
    CollectionRequired,
};
use crate::slice_extension::GetTwoMutSlice;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MappedVector<T> {
    frames: Vec<T>,
    position: HashMap<usize, usize>,
    inverse_position: Vec<usize>,
}

impl<T> MappedVector<T> {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            position: HashMap::new(),
            inverse_position: Vec::new(),
        }
    }

    pub fn frames(&self) -> &Vec<T> {
        &self.frames
    }

    pub fn inverse_position(&self) -> &Vec<usize> {
        &self.inverse_position
    }

    fn insert(&mut self, key: usize, value: T) -> Option<T> {
        if let Some(&key) = self.position.get(&key) {
            let old = mem::replace(self.frames.index_mut(key), value);
            return Some(old);
        }
        self.position.insert(key, self.frames.len());
        self.frames.push(value);
        self.inverse_position.push(key);
        None
    }
}

impl<T> FromIterator<(usize, T)> for MappedVector<T> {
    fn from_iter<I: IntoIterator<Item = (usize, T)>>(iter: I) -> Self {
        let mut res = MappedVector::new();
        for (key, value) in iter {
            res.insert(key, value);
        }
        res
    }
}

impl<'l, T> IntoIterator for &'l MappedVector<T> {
    type Item = (usize, &'l T);
    type IntoIter =
        Zip<Map<slice::Iter<'l, usize>, fn(&usize) -> usize>, slice::Iter<'l, T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.inverse_position
            .iter()
            .map((|i| *i) as fn(&usize) -> usize)
            .zip(self.frames.iter())
    }
}

impl<'l, T> IntoIterator for &'l mut MappedVector<T> {
    type Item = (usize, &'l mut T);
    type IntoIter =
        Zip<Map<slice::Iter<'l, usize>, fn(&usize) -> usize>, slice::IterMut<'l, T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.inverse_position
            .iter()
            .map((|i| *i) as fn(&usize) -> usize)
            .zip(self.frames.iter_mut())
    }
}

impl<T> IntoIterator for MappedVector<T> {
    type Item = (usize, T);
    type IntoIter =
        Zip<<Vec<usize> as IntoIterator>::IntoIter, <Vec<T> as IntoIterator>::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.inverse_position.into_iter().zip(self.frames)
    }
}

impl<T: Default + Clone> CollectionRequired for MappedVector<T> {
    type T = T;
    type IterMut<'l> = <&'l mut Self as IntoIterator>::IntoIter where T: 'l;

    #[inline]
    fn insert(&mut self, key: usize, value: T) -> Option<T> {
        self.insert(key, value)
    }

    fn remove(&mut self, key: usize) -> Option<T> {
        let key_position = self.position.remove(&key)?;
        self.inverse_position.swap_remove(key_position);
        if key_position != self.inverse_position.len() {
            // when things are thoroughly tested, use get_unchecked here
            *self
                .position
                .get_mut(
                    self.inverse_position
                        .get(key_position)
                        .expect("that's an implementation bug; please report"),
                )
                .expect("that's an implementation bug; please report") = key_position;
        }
        Some(self.frames.swap_remove(key_position))
    }

    #[inline]
    fn get_mut(&mut self, key: usize) -> Option<&mut T> {
        Some(self.frames.index_mut(*self.position.get(&key)?))
    }

    fn get_two_mut(&mut self, key_a: usize, key_b: usize) -> Option<(&mut T, &mut T)> {
        self.frames
            .get_two_mut(*self.position.get(&key_a)?, *self.position.get(&key_b)?)
    }

    #[inline]
    fn len(&self) -> usize {
        self.frames.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.into_iter()
    }

    fn init(num_keys: usize) -> Self {
        let (frames, position, inverse_position) =
            (0..num_keys).map(|i| (T::default(), (i, i), i)).multiunzip();
        Self {
            frames,
            position,
            inverse_position,
        }
    }
}

impl<T: Default + Clone> Collection for MappedVector<T> {
    type Iter<'l> = <&'l Self as IntoIterator>::IntoIter where T: 'l;

    #[inline]
    fn get(&self, key: usize) -> Option<&T> {
        Some(self.frames.index(*self.position.get(&key)?))
    }

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        self.into_iter()
    }
}
