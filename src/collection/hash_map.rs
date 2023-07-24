use std::iter;

use hashbrown::{
    hash_map,
    HashMap,
};

use super::{
    Base,
    Full,
    Iterable,
};

pub type Map<T> = HashMap<usize, T>;

impl<T: Clone> Base for Map<T> {
    type T = T;

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

    fn init(num_keys: usize, init_val: T) -> Self {
        let mut ret = HashMap::with_capacity(num_keys);
        for i in 0..num_keys {
            ret.insert(i, init_val.clone());
        }
        ret
    }
}

impl<T: Clone> Iterable for Map<T> {
    type Iter<'l> = iter::Map<
        hash_map::Iter<'l, usize, T>,
        fn((&'l usize, &'l T)) -> (usize, &'l T),
    > where T: 'l;

    type IterMut<'l> = iter::Map<
        hash_map::IterMut<'l, usize, T>,
        fn((&'l usize, &'l mut T)) -> (usize, &'l mut T),
    > where T: 'l;

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        self.iter().map(|(&i, p)| (i, p))
    }

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter_mut().map(|(&i, p)| (i, p))
    }
}

impl<T: Clone> Full for Map<T> {}
