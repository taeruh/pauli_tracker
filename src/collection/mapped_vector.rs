use std::{
    hash::BuildHasher,
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

use hashbrown::{
    hash_map::DefaultHashBuilder,
    HashMap,
};
use itertools::Itertools;
#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    Base,
    Full,
    Init,
    Iterable,
    IterableBase,
};
use crate::slice_extension::GetTwoMutSlice;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(serialize = "S: BuildHasher + Serialize, T: Serialize"))
)]
#[cfg_attr(
    feature = "serde",
    serde(bound(deserialize = "S: Default + BuildHasher + for<'a>  \
                               Deserialize<'a>, T: for<'a> Deserialize<'a>"))
)]
pub struct MappedVector<T, S = DefaultHashBuilder> {
    frames: Vec<T>,
    position: HashMap<usize, usize, S>,
    inverse_position: Vec<usize>,
}

impl<T, S> PartialEq for MappedVector<T, S>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.frames == other.frames && self.inverse_position == other.inverse_position
    }
}
impl<T, S> Eq for MappedVector<T, S> where T: Eq {}

impl<T> MappedVector<T> {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            position: HashMap::new(),
            inverse_position: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            frames: Vec::with_capacity(capacity),
            position: HashMap::with_capacity(capacity),
            inverse_position: Vec::with_capacity(capacity),
        }
    }
}

impl<T, S> MappedVector<T, S> {
    pub fn with_hasher(hash_builder: S) -> Self {
        Self {
            frames: Vec::new(),
            position: HashMap::with_hasher(hash_builder),
            inverse_position: Vec::new(),
        }
    }

    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        Self {
            frames: Vec::with_capacity(capacity),
            position: HashMap::with_capacity_and_hasher(capacity, hash_builder),
            inverse_position: Vec::with_capacity(capacity),
        }
    }
}

impl<T, S> MappedVector<T, S>
where
    S: BuildHasher,
{
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

impl<T, S> FromIterator<(usize, T)> for MappedVector<T, S>
where
    S: BuildHasher + Default,
{
    fn from_iter<I: IntoIterator<Item = (usize, T)>>(iter: I) -> Self {
        let mut res = MappedVector::with_hasher(Default::default());
        for (key, value) in iter {
            res.insert(key, value);
        }
        res
    }
}

impl<'l, T, S> IntoIterator for &'l MappedVector<T, S> {
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

impl<'l, T, S> IntoIterator for &'l mut MappedVector<T, S> {
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

impl<T, S> IntoIterator for MappedVector<T, S> {
    type Item = (usize, T);
    type IntoIter =
        Zip<<Vec<usize> as IntoIterator>::IntoIter, <Vec<T> as IntoIterator>::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.inverse_position.into_iter().zip(self.frames)
    }
}

impl<T, S> Base for MappedVector<T, S>
where
    S: BuildHasher,
{
    type TB = T;
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
    fn get(&self, key: usize) -> Option<&T> {
        Some(self.frames.index(*self.position.get(&key)?))
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
}

impl<T, S> Iterable for MappedVector<T, S>
where
    T: Clone,
{
    type TI = T;
    type Iter<'l> = <&'l Self as IntoIterator>::IntoIter where T: 'l, S: 'l;
    type IterMut<'l> = <&'l mut Self as IntoIterator>::IntoIter where T: 'l, S: 'l;

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        self.into_iter()
    }

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.into_iter()
    }
}

impl<T, S> Init for MappedVector<T, S>
where
    T: Clone + Default,
    S: BuildHasher + Default,
{
    fn init(len: usize) -> Self {
        let init_val = T::default();
        let (frames, position, inverse_position) =
            (0..len).map(|i| (init_val.clone(), (i, i), i)).multiunzip();
        Self {
            frames,
            position,
            inverse_position,
        }
    }
}

impl<T, S> IterableBase for MappedVector<T, S>
where
    T: Clone,
    S: BuildHasher,
{
    type T = T;
}
impl<T: Clone + Default, S: BuildHasher + Default> Full for MappedVector<T, S> {}
