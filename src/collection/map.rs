use std::{
    hash::BuildHasher,
    iter,
};

use hashbrown::{
    hash_map::{
        self,
        DefaultHashBuilder,
    },
    HashMap,
};

use super::{
    Base,
    Full,
    Init,
    Iterable,
    IterableBase,
};

/// A [HashMap](https://docs.rs/hashbrown/latest/hashbrown/struct.HashMap.html#) of with
/// [usize] keys.
pub type Map<T, S = DefaultHashBuilder> = HashMap<usize, T, S>;

impl<T, S> Base for Map<T, S>
where
    T: Clone,
    S: BuildHasher + Default,
{
    type TB = T;

    #[inline]
    fn insert(&mut self, key: usize, value: T) -> Option<T> {
        self.insert(key, value)
    }

    #[inline]
    fn remove(&mut self, key: usize) -> Option<T> {
        self.remove(&key)
    }

    #[inline]
    fn get(&self, key: usize) -> Option<&T> {
        self.get(&key)
    }

    #[inline]
    fn get_mut(&mut self, key: usize) -> Option<&mut T> {
        self.get_mut(&key)
    }

    fn get_two_mut(&mut self, key_a: usize, key_b: usize) -> Option<(&mut T, &mut T)> {
        self.get_many_mut([&key_a, &key_b]).map(|[a, b]| (a, b))
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T, S> Iterable for Map<T, S>
where
    T: Clone,
    S: BuildHasher + Default,
{
    type TI = T;
    type Iter<'l> = iter::Map<
        hash_map::Iter<'l, usize, T>,
        fn((&'l usize, &'l T)) -> (usize, &'l T),
    > where T: 'l, S: 'l;

    type IterMut<'l> = iter::Map<
        hash_map::IterMut<'l, usize, T>,
        fn((&'l usize, &'l mut T)) -> (usize, &'l mut T),
    > where T: 'l, S: 'l;

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        self.iter().map(|(&i, p)| (i, p))
    }

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter_mut().map(|(&i, p)| (i, p))
    }
}

impl<T: Clone + Default, S: BuildHasher + Default> Init for Map<T, S> {
    fn init(num_keys: usize) -> Self {
        let init_val = T::default();
        let mut ret = HashMap::with_capacity_and_hasher(num_keys, S::default());
        for i in 0..num_keys {
            ret.insert(i, init_val.clone());
        }
        ret
    }
}

impl<T, S> IterableBase for Map<T, S>
where
    T: Clone,
    S: BuildHasher + Default,
{
    type T = T;
}
impl<T, S> Full for Map<T, S>
where
    T: Clone + Default,
    S: BuildHasher + Default,
{
}
