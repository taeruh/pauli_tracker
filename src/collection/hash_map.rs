use std::{
    collections::hash_map::{
        self,
        HashMap,
    },
    iter,
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
        if key_a == key_b {
            return None;
        }
        // Safety: We checked above that the keys are different, so it is impossible
        // that we create two mutable references to the same object (except if the
        // hashing is broken). Regarding temporary aliasing: If we would do exactly the
        // same with, let's say, a Vec, we would get some Stack-borrow errors from Miri.
        // This is an example where the Stacked-borrow rules are too strict. It would be
        // okay under the Tree-borrow rules. The question here is: why do we not get any
        // Stacked-borrow errors (Tree-borrow is fine)? The answer is that the HashMap
        // does not access values through a single pointer with offsets, but through
        // multiple pointers, one for each value. This means when we have pointer to a
        // value, it will not be invalidated by getting a pointer to another reference,
        // because they are not accessed through the same pointer (This is actually an
        // implementation detail of the hashbrown::HashMap and instead of relying on it
        // we should rather use the hashbrown::HashMap directly and its get_many_mut
        // method).
        //
        // not creating the &mut directly ensures that we at least fulfill the
        // Tree-borrow rules if the implementation of HashMap changes (if it changes too
        // drastically, this might not be true anymore)
        let a = self.get_mut(&key_a)? as *mut T;
        let b = self.get_mut(&key_b)? as *mut T;
        debug_assert!(!std::ptr::eq(a, b));
        unsafe { Some((&mut *a, &mut *b)) }
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
