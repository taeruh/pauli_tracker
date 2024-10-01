use std::{
    hash::BuildHasher,
    iter::{Map, Zip},
    mem,
    ops::{Index, IndexMut},
    slice,
};

use hashbrown::{HashMap, hash_map::DefaultHashBuilder};
use itertools::Itertools;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{Base, Full, Init, Iterable, IterableBase};
use crate::slice_extension::GetTwoMutSlice;

/// A mixture of a [Vec] and a [HashMap].
///
/// The elements are stored in a [Vec] storage while accessing them is done through a
/// [HashMap] to get the right index in the storage. Inserting elements is done by
/// pushing to the storage and removing is done via swap-removes.
///
/// [HashMap]: https://docs.rs/hashbrown/latest/hashbrown/struct.HashMap.html#
#[derive(Debug, Clone, Default)]
/// instead of going through _MappedVector we should implement it directly, at least for
/// the serialization, because we are unnecessarily cloning it there
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(from = "_MappedVector<T>"))]
#[cfg_attr(feature = "serde", serde(into = "_MappedVector<T>"))]
#[cfg_attr(
    feature = "serde",
    serde(bound(serialize = "T: Clone + Serialize, S: Clone"))
)]
#[cfg_attr(
    feature = "serde",
    serde(bound(deserialize = "T: for<'a> Deserialize<'a>, S: Default + BuildHasher"))
)]
pub struct MappedVector<T, S = DefaultHashBuilder> {
    storage: Vec<T>,
    position: HashMap<usize, usize, S>,
    inverse_position: Vec<usize>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct _MappedVector<T> {
    storage: Vec<T>,
    inverse_position: Vec<usize>,
}

impl<T, S: Default + BuildHasher> From<_MappedVector<T>> for MappedVector<T, S> {
    fn from(value: _MappedVector<T>) -> Self {
        Self {
            storage: value.storage,
            position: HashMap::from_iter(
                value
                    .inverse_position
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(position, key)| (key, position)),
            ),
            inverse_position: value.inverse_position,
        }
    }
}

impl<T, S> From<MappedVector<T, S>> for _MappedVector<T> {
    fn from(value: MappedVector<T, S>) -> Self {
        Self {
            storage: value.storage,
            inverse_position: value.inverse_position,
        }
    }
}

impl<T, S> PartialEq for MappedVector<T, S>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.storage == other.storage && self.inverse_position == other.inverse_position
    }
}
impl<T, S> Eq for MappedVector<T, S> where T: Eq {}

impl<T> MappedVector<T> {
    /// Creates a new empty [MappedVector].
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
            position: HashMap::new(),
            inverse_position: Vec::new(),
        }
    }

    /// Creates a new empty [MappedVector] with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            storage: Vec::with_capacity(capacity),
            position: HashMap::with_capacity(capacity),
            inverse_position: Vec::with_capacity(capacity),
        }
    }
}

impl<T, S> MappedVector<T, S> {
    /// Creates a new empty [MappedVector] with the given hasher.
    pub fn with_hasher(hash_builder: S) -> Self {
        Self {
            storage: Vec::new(),
            position: HashMap::with_hasher(hash_builder),
            inverse_position: Vec::new(),
        }
    }

    /// Creates a new empty [MappedVector] with the given capacity and hasher.
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        Self {
            storage: Vec::with_capacity(capacity),
            position: HashMap::with_capacity_and_hasher(capacity, hash_builder),
            inverse_position: Vec::with_capacity(capacity),
        }
    }
}

impl<T, S> MappedVector<T, S>
where
    S: BuildHasher,
{
    /// Get the inner storage of the elements. Together with the return value from
    /// [inverse_position](Self::inverse_position), the zipped values are the correct
    /// key-value pairs.
    pub fn storage(&self) -> &Vec<T> {
        &self.storage
    }

    /// Get the inverse position of the elements. Together with the return value from
    /// [storage](Self::storage), the zipped values are the correct key-value pairs.
    pub fn inverse_position(&self) -> &Vec<usize> {
        &self.inverse_position
    }

    fn insert(&mut self, key: usize, value: T) -> Option<T> {
        if let Some(&key) = self.position.get(&key) {
            let old = mem::replace(self.storage.index_mut(key), value);
            return Some(old);
        }
        self.position.insert(key, self.storage.len());
        self.storage.push(value);
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
            .zip(self.storage.iter())
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
            .zip(self.storage.iter_mut())
    }
}

impl<T, S> IntoIterator for MappedVector<T, S> {
    type Item = (usize, T);
    type IntoIter =
        Zip<<Vec<usize> as IntoIterator>::IntoIter, <Vec<T> as IntoIterator>::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.inverse_position.into_iter().zip(self.storage)
    }
}

impl<T, S> Base for MappedVector<T, S>
where
    S: BuildHasher,
{
    type TB = T;
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
        Some(self.storage.swap_remove(key_position))
    }

    fn get(&self, key: usize) -> Option<&T> {
        Some(self.storage.index(*self.position.get(&key)?))
    }

    fn get_mut(&mut self, key: usize) -> Option<&mut T> {
        Some(self.storage.index_mut(*self.position.get(&key)?))
    }

    fn get_two_mut(&mut self, key_a: usize, key_b: usize) -> Option<(&mut T, &mut T)> {
        self.storage
            .get_two_mut(*self.position.get(&key_a)?, *self.position.get(&key_b)?)
    }

    fn len(&self) -> usize {
        self.storage.len()
    }

    fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }
}

impl<T, S> Iterable for MappedVector<T, S>
where
    T: Clone,
{
    type TI = T;
    type Iter<'l>
        = <&'l Self as IntoIterator>::IntoIter
    where
        T: 'l,
        S: 'l;
    type IterMut<'l>
        = <&'l mut Self as IntoIterator>::IntoIter
    where
        T: 'l,
        S: 'l;

    fn iter_pairs(&self) -> Self::Iter<'_> {
        self.into_iter()
    }

    fn iter_pairs_mut(&mut self) -> Self::IterMut<'_> {
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
            storage: frames,
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
