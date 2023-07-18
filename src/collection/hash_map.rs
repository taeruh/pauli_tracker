use std::{
    collections::hash_map::{
        self,
        HashMap,
    },
    iter,
};

use super::Collection;

pub type Map<T> = HashMap<usize, T>;

impl<T: Default + Clone> Collection for Map<T> {
    type T = T;
    type Iter<'l> = iter::Map<
        hash_map::Iter<'l, usize, T>,
        fn((&'l usize, &'l T)) -> (usize, &'l T),
    > where T: 'l;
    type IterMut<'l> = iter::Map<
        hash_map::IterMut<'l, usize, T>,
        fn((&'l usize, &'l mut T)) -> (usize, &'l mut T),
    > where T: 'l;

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
        // Safety: we checked above that the keys are different, so it is impossible
        // that we create two mutable references to the same object (except if there is
        // a bug in the bucket assigment of the HashMap)
        //
        // I do not know why this isn't triggering an stack-borrow error in miri; doing
        // the same with Vec/slice does trigger an error. In general it would be cleaner
        // to go over pointers as I do it for the MappedVector but a HashMap is more
        // complicated and the tools for that are not stable yet
        let a = unsafe { &mut *(self.get_mut(&key_a)? as *mut T) };
        let b = unsafe { &mut *(self.get_mut(&key_b)? as *mut T) };
        // that would catch a bug in the bucket assignment
        // assert!(!std::ptr::eq(a, b));
        Some((a, b))
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        self.iter().map(|(&i, p)| (i, p))
    }

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter_mut().map(|(&i, p)| (i, p))
    }

    fn init(num_keys: usize) -> Self {
        let mut ret = HashMap::with_capacity(num_keys);
        for i in 0..num_keys {
            ret.insert(i, T::default());
        }
        ret
    }
}
